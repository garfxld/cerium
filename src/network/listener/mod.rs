use crate::{
    network::client::ClientConnection,
    protocol::{buffer::ByteBuffer, ProtcolState},
};

mod config;
mod handshake;
mod login;
mod play;
mod status;

pub trait PacketHandler {
    fn handle_packet(&mut self, id: i32, data: &mut ByteBuffer);
}

impl PacketHandler for ClientConnection {
    fn handle_packet(&mut self, id: i32, data: &mut ByteBuffer) {
        match self.state {
            ProtcolState::Handshake => handshake::handle_packet(self, id, data),
            ProtcolState::Status => status::handle_packet(self, id, data),
            ProtcolState::Login => login::handle_packet(self, id, data),
            ProtcolState::Config => config::handle_packet(self, id, data),
            ProtcolState::Play => play::handle_packet(self, id, data),
        }
    }
}
