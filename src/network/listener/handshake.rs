use crate::{
    network::client::ClientConnection,
    protocol::{buffer::ByteBuffer, decode::Decode, packet::HandshakePacket, ProtcolState},
};

pub(crate) fn handle_packet(client: &mut ClientConnection, id: i32, data: &mut ByteBuffer) {
    match id {
        0x00 => handle_handshake(client, HandshakePacket::decode(data).unwrap()),
        _ => panic!("Unknown packet! ({})", id),
    }
}

fn handle_handshake(client: &mut ClientConnection, packet: HandshakePacket) {
    log::trace!("{:?}", &packet);

    client.state = match packet.intent {
        1 => ProtcolState::Status,
        2 => ProtcolState::Login,
        3 => unimplemented!("Not yet implemented."),
        _ => panic!("Invalid next intent"),
    };
}
