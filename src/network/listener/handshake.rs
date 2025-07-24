use cerium_protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
    packet::HandshakePacket,
    ProtocolState,
};
use std::sync::Arc;

use crate::network::client::ClientConnection;

pub async fn handle_packet(
    client: Arc<ClientConnection>,
    id: i32,
    data: &mut ByteBuffer,
) -> Result<(), DecodeError> {
    match id {
        0x00 => handle_handshake(client, HandshakePacket::decode(data)?).await,
        _ => panic!("Unknown packet! ({})", id),
    };
    Ok(())
}

async fn handle_handshake(client: Arc<ClientConnection>, packet: HandshakePacket) {
    log::trace!("{:?}", &packet);
    let mut state = client.state.lock().await;
    *state = match packet.intent {
        1 => ProtocolState::Status,
        2 => ProtocolState::Login,
        3 => unimplemented!("Not yet implemented."),
        _ => panic!("Invalid next intent"),
    };
}
