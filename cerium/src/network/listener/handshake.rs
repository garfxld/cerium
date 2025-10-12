use std::{io::Cursor, sync::Arc};

use crate::{
    network::client::ClientConnection,
    protocol::{
        ProtocolState,
        decode::{Decode, DecodeError},
        packet::HandshakePacket,
    },
};

#[rustfmt::skip]
pub async fn handle_packet(client: Arc<ClientConnection>, id: i32, data: &mut Cursor<&[u8]>) -> Result<(), DecodeError> {
    match id {
        0x00 => handle_handshake(client, HandshakePacket::decode(data)?).await,
        _ => return Err(DecodeError::UnkownPacket(id)),
    };
    Ok(())
}

async fn handle_handshake(client: Arc<ClientConnection>, packet: HandshakePacket) {
    let mut state = client.state.lock().await;
    *state = match packet.intent {
        1 => ProtocolState::Status,
        2 => ProtocolState::Login,
        3 => unimplemented!("Not yet implemented."),
        _ => panic!("Invalid next intent"),
    };
}
