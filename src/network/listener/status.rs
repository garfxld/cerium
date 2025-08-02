use crate::{event::ServerListPingEvent, network::client::ClientConnection};
use cerium_protocol::{
    buffer::ByteBuffer,
    decode::{Decode as _, DecodeError},
    packet::{PingRequestPacket, PongResponsePacket, StatusRequestPacket, StatusResponsePacket},
};
use std::sync::Arc;

pub async fn handle_packet(
    client: Arc<ClientConnection>,
    id: i32,
    data: &mut ByteBuffer,
) -> Result<(), DecodeError> {
    match id {
        0x00 => handle_status_request(client, StatusRequestPacket::decode(data)?).await,
        0x01 => handle_ping_request(client, PingRequestPacket::decode(data)?).await,
        _ => panic!("Unknown packet! ({})", id),
    };
    Ok(())
}

async fn handle_status_request(client: Arc<ClientConnection>, packet: StatusRequestPacket) {
    log::trace!("{:?}", &packet);
    let _ = packet;

    let mut event = ServerListPingEvent::new(SERVER_LIST_PING.to_owned());
    client.server.events().fire(&mut event).await;

    let response = StatusResponsePacket {
        json_response: event.response,
    };

    client.send_packet(0x00, response).await;
}

async fn handle_ping_request(client: Arc<ClientConnection>, packet: PingRequestPacket) {
    log::trace!("{:?}", &packet);
    client
        .send_packet(
            0x01,
            PongResponsePacket {
                timestamp: packet.timestamp,
            },
        )
        .await;
}

const SERVER_LIST_PING: &'static str = r#"
{
    "version": {
        "name": "1.21.7",
        "protocol": 772
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
