use cerium_util::auth::GameProfile;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Failed to decrypt secret")]
    DecryptionError,
    #[error("Failed to connect to Mojang session server")]
    FailedRequest,
    #[error("Failed to parse profile json")]
    MalformedJson,
}

#[derive(Debug)]
pub struct KeyStore {
    pub private_key: rsa::RsaPrivateKey,
    pub public_key_der: Box<[u8]>,
}

impl KeyStore {
    pub fn new() -> Self {
        use rsa::{RsaPrivateKey, traits::PublicKeyParts as _};

        let mut rand = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rand, 1024).unwrap();

        let public_key_der = rsa_der::public_key_to_der(
            &private_key.n().to_bytes_be(),
            &private_key.e().to_bytes_be(),
        )
        .into_boxed_slice();

        Self {
            private_key,
            public_key_der,
        }
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, AuthError> {
        self.private_key
            .decrypt(rsa::Pkcs1v15Encrypt, data)
            .map_err(|_| AuthError::DecryptionError)
    }

    pub fn digest_secret(&self, secret: &[u8]) -> String {
        use sha1::{Digest as _, Sha1};

        num_bigint::BigInt::from_signed_bytes_be(
            &Sha1::new()
                .chain_update(&secret)
                .chain_update(&self.public_key_der)
                .finalize(),
        )
        .to_str_radix(16)
    }
}

pub type Decryptor = cfb8::Decryptor<aes::Aes128>;
pub type Encryptor = cfb8::Encryptor<aes::Aes128>;

const MOJANG_AUTH_URL: &'static str = "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={username}&serverId={hash}";

pub fn authenthicate(
    username: &String,
    hash: &String,
    ip: Option<std::net::Ipv4Addr>,
) -> Result<GameProfile, AuthError> {
    let url = if let Some(_) = ip {
        todo!()
    } else {
        MOJANG_AUTH_URL
            .replace("{username}", &username)
            .replace("{hash}", &hash)
    };

    let mut response = ureq::get(url)
        .call()
        .map_err(|_| AuthError::FailedRequest)?;

    response
        .body_mut()
        .read_json::<GameProfile>()
        .map_err(|_| AuthError::MalformedJson)
}
