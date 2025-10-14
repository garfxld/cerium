use crate::protocol::{
    decode::{Decode as _, DecodeError},
    packet::{
        PongResponsePacket, StatusRequestPacket, StatusResponsePacket,
        client::status::PingRequestPacket,
    },
};
use crate::{event::ServerListPingEvent, network::client::Connection};

use std::{io::Cursor, sync::Arc};

#[rustfmt::skip]
pub async fn handle_packet(client: Arc<Connection>, id: i32, data: &mut Cursor<&[u8]>) -> Result<(), DecodeError> {
    match id {
        0x00 => handle_status_request(client, StatusRequestPacket::decode(data)?).await,
        0x01 => handle_ping_request(client, PingRequestPacket::decode(data)?).await,
        _ => return Err(DecodeError::UnkownPacket(id)),
    };
    Ok(())
}

async fn handle_status_request(client: Arc<Connection>, packet: StatusRequestPacket) {
    let _ = packet;

    let mut event = ServerListPingEvent::new(SERVER_LIST_PING.to_owned());
    client.server().events().fire(&mut event);

    let response = StatusResponsePacket {
        json_response: event.response,
    };

    client.send_packet(response);
}

async fn handle_ping_request(client: Arc<Connection>, packet: PingRequestPacket) {
    let packet = PongResponsePacket {
        timestamp: packet.timestamp,
    };
    client.send_packet(packet);
}

const SERVER_LIST_PING: &'static str = r#"
{
    "version": {
        "name": "1.21.10",
        "protocol": 773
    },
    "players": {
        "max": 100,
        "online": 5,
        "sample": [
            {
                "name": "thinkofdeath",
                "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
            }
        ]
    },
    "description": {
        "text": "Hello, world!"
    },
    "enforcesSecureChat": false
}
"#;
