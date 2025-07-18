use crate::{
    network::{
        auth::{self, CryptContext, GameProfile},
        client::ClientConnection,
    },
    protocol::{
        buffer::ByteBuffer,
        decode::Decode as _,
        packet::{
            EncryptionRequestPacket, EncryptionResponsePacket, LoginAcknowledgePacket,
            LoginStartPacket, LoginSuccessPacket,
        },
        ProtcolState,
    },
};

pub(crate) fn handle_packet(client: &mut ClientConnection, id: i32, data: &mut ByteBuffer) {
    match id {
        0x00 => handle_login_start(client, LoginStartPacket::decode(data).unwrap()),
        0x01 => handle_encryption_response(client, EncryptionResponsePacket::decode(data).unwrap()),
        0x02 => handle_plugin_response(client),
        0x03 => handle_login_acknowledged(client, LoginAcknowledgePacket::decode(data).unwrap()),
        0x04 => handle_cookie_response(client),
        _ => panic!("Unknown packet! ({})", id),
    }
}

fn handle_login_start(client: &mut ClientConnection, packet: LoginStartPacket) {
    log::trace!("{:?}", &packet);
    client.game_profile = GameProfile {
        uuid: packet.uuid,
        name: packet.name,
        properties: vec![],
    };

    // todo: check for online mode
    if true {
        client.verify_token = rand::random();

        client.send_packet(
            0x01,
            EncryptionRequestPacket {
                server_id: "".to_owned(),
                public_key: client.key_store.public_key_der.clone(),
                verify_token: Box::new(client.verify_token),
                should_authenticate: true,
            },
        );

        return;
    }

    // ofline mode
    client.send_packet(0x02, LoginSuccessPacket::from(client.game_profile.clone()));
}

fn handle_encryption_response(client: &mut ClientConnection, packet: EncryptionResponsePacket) {
    log::trace!("{:?}", &packet);
    let shared_secret = client.key_store.decrypt(&packet.shared_secret).unwrap();

    // enable encryption
    client.crypt_context = Some(CryptContext::new(&shared_secret));

    let username = &client.game_profile.name;
    let hash = &client.key_store.digest_secret(&shared_secret);

    client.game_profile = auth::authenthicate(username, hash, None).unwrap();
    client.send_packet(0x02, LoginSuccessPacket::from(client.game_profile.clone()));
}

fn handle_plugin_response(_client: &mut ClientConnection) {
    todo!()
}

fn handle_login_acknowledged(client: &mut ClientConnection, packet: LoginAcknowledgePacket) {
    log::trace!("{:?}", &packet);
    client.state = ProtcolState::Config;
}

fn handle_cookie_response(_client: &mut ClientConnection) {
    todo!()
}
