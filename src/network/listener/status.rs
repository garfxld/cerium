use crate::{
    network::client::ClientConnection,
    protocol::{
        buffer::ByteBuffer,
        decode::Decode as _,
        packet::{
            PingRequestPacket, PongResponsePacket, StatusRequestPacket, StatusResponsePacket,
        },
    },
};

pub(crate) fn handle_packet(client: &mut ClientConnection, id: i32, data: &mut ByteBuffer) {
    match id {
        0x00 => handle_status_request(client, StatusRequestPacket::decode(data).unwrap()),
        0x01 => handle_ping_request(client, PingRequestPacket::decode(data).unwrap()),
        _ => panic!("Unknown packet! ({})", id),
    }
}

fn handle_status_request(client: &mut ClientConnection, packet: StatusRequestPacket) {
    log::trace!("{:?}", &packet);
    let _ = packet;

    let response = StatusResponsePacket {
        json_response: r#"
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
                        "favicon": "data:image/png;base64,<data>",
                        "enforcesSecureChat": false
                    }
                    "#
        .to_string(),
    };

    client.send_packet(0x00, response);
}

fn handle_ping_request(client: &mut ClientConnection, packet: PingRequestPacket) {
    log::trace!("{:?}", &packet);
    client.send_packet(
        0x01,
        PongResponsePacket {
            timestamp: packet.timestamp,
        },
    );
}
