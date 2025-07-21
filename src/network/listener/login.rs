use std::sync::Arc;

use crate::{
    network::{
        auth::{self, CryptContext, GameProfile},
        client::ClientConnection,
    },
    protocol::{
        buffer::ByteBuffer,
        decode::{Decode as _, DecodeError},
        packet::{
            EncryptionRequestPacket, EncryptionResponsePacket, LoginAcknowledgePacket,
            LoginStartPacket, LoginSuccessPacket,
        },
        ProtocolState,
    },
};

pub async fn handle_packet(
    client: Arc<ClientConnection>,
    id: i32,
    data: &mut ByteBuffer,
) -> Result<(), DecodeError> {
    match id {
        0x00 => handle_login_start(client, LoginStartPacket::decode(data)?).await,
        0x01 => handle_encryption_response(client, EncryptionResponsePacket::decode(data)?).await,
        0x02 => handle_plugin_response(client).await,
        0x03 => handle_login_acknowledged(client, LoginAcknowledgePacket::decode(data)?).await,
        0x04 => handle_cookie_response(client).await,
        _ => panic!("Unknown packet! ({})", id),
    };
    Ok(())
}

async fn handle_login_start(client: Arc<ClientConnection>, packet: LoginStartPacket) {
    log::trace!("{:?}", &packet);
    *client.game_profile.lock().await = GameProfile {
        uuid: packet.uuid,
        name: packet.name,
        properties: vec![],
    };

    // todo: check for online mode
    if true {
        let verify_token: [u8; 4] = rand::random();
        *client.verify_token.lock().await = verify_token;

        client
            .send_packet(
                0x01,
                EncryptionRequestPacket {
                    server_id: "".to_owned(),
                    public_key: client.key_store.public_key_der.clone(),
                    verify_token: Box::new(verify_token),
                    should_authenticate: true,
                },
            )
            .await;

        return;
    }

    // ofline mode
    client
        .send_packet(
            0x02,
            LoginSuccessPacket::from(client.game_profile.lock().await.clone()),
        )
        .await;
}

async fn handle_encryption_response(client: Arc<ClientConnection>, packet: EncryptionResponsePacket) {
    log::trace!("{:?}", &packet);
    let shared_secret = client.key_store.decrypt(&packet.shared_secret).unwrap();

    // enable encryption
    *client.crypt_context.lock().await = Some(CryptContext::new(&shared_secret));

    let mut client_game_profile = client.game_profile.lock().await;

    let username = &client_game_profile.name;
    let hash = &client.key_store.digest_secret(&shared_secret);

    let game_profile = auth::authenthicate(username, hash, None).unwrap();

    *client_game_profile = game_profile.clone();

    client
        .send_packet(0x02, LoginSuccessPacket::from(game_profile.clone()))
        .await;
}

async fn handle_plugin_response(_client: Arc<ClientConnection>) {
    todo!()
}

async fn handle_login_acknowledged(client: Arc<ClientConnection>, packet: LoginAcknowledgePacket) {
    log::trace!("{:?}", &packet);
    *client.state.lock().await = ProtocolState::Config;
}

async fn handle_cookie_response(_client: Arc<ClientConnection>) {
    todo!()
}
