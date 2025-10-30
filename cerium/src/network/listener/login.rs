use std::{io::Cursor, sync::Arc};

use crate::{
    auth::{self, GameProfile},
    network::client::Connection,
    protocol::{
        ProtocolState,
        decode::{Decode as _, DecodeError},
        packet::{
            EncryptionRequestPacket, EncryptionResponsePacket, LoginAcknowledgePacket,
            LoginStartPacket, LoginSuccessPacket, SetCompressionPacket,
        },
    },
};

#[rustfmt::skip]
pub async fn handle_packet(client: Arc<Connection>, id: i32, data: &mut Cursor<&[u8]>) -> Result<(), DecodeError> {
    match id {
        0x00 => handle_login_start(client, LoginStartPacket::decode(data)?).await,
        0x01 => handle_encryption_response(client, EncryptionResponsePacket::decode(data)?).await,
        0x02 => handle_plugin_response(client),
        0x03 => handle_login_acknowledged(client, LoginAcknowledgePacket::decode(data)?),
        0x04 => handle_cookie_response(client),
        _ => return Err(DecodeError::UnkownPacket(id)),
    };
    Ok(())
}

async fn handle_login_start(client: Arc<Connection>, packet: LoginStartPacket) {
    *client.game_profile.lock() = Some(GameProfile {
        uuid: packet.uuid,
        name: packet.name,
        properties: vec![],
    });

    let threshold = 256;

    if true {
        client
            .send_packet_now(SetCompressionPacket { threshold })
            .await;
        client.set_compression(threshold).await;
    }

    // todo: check for online mode
    if false {
        // online mode
        let verify_token: [u8; 4] = rand::random();
        *client.verify_token.lock() = verify_token;

        client.send_packet(EncryptionRequestPacket {
            server_id: "".to_owned(),
            public_key: client.key_store.public_key_der.clone(),
            verify_token: Box::new(verify_token),
            should_authenticate: true,
        });
    } else {
        // offline mode
        client.send_packet(LoginSuccessPacket::from(
            client.game_profile.lock().clone().unwrap(),
        ));
    }
}

async fn handle_encryption_response(client: Arc<Connection>, packet: EncryptionResponsePacket) {
    let shared_secret = client.key_store.decrypt(&packet.shared_secret).unwrap();

    // enable encryption
    client.set_encryption(&shared_secret).await;

    let mut client_game_profile = client.game_profile.lock();

    let username = &client_game_profile.clone().unwrap().name;
    let hash = &client.key_store.digest_secret(&shared_secret);

    let game_profile = auth::authenthicate(username, hash, None).unwrap();

    *client_game_profile = Some(game_profile.clone());

    client.send_packet(LoginSuccessPacket::from(game_profile.clone()));
}

fn handle_plugin_response(client: Arc<Connection>) {
    let _ = client;
}

fn handle_login_acknowledged(client: Arc<Connection>, packet: LoginAcknowledgePacket) {
    let _ = packet;
    client.set_state(ProtocolState::Config);
}

fn handle_cookie_response(client: Arc<Connection>) {
    let _ = client;
}
