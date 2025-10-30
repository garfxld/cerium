use crate::network::client::Connection;
use crate::protocol::{ProtocolState, decode::DecodeError};
use std::io::Cursor;
use std::sync::Arc;

mod config;
mod handshake;
mod login;
mod play;
mod status;

impl Connection {
    pub async fn handle_packet(
        self: Arc<Self>,
        id: i32,
        data: &mut Cursor<&[u8]>,
    ) -> Result<(), DecodeError> {
        match self.state() {
            ProtocolState::Handshake => handshake::handle_packet(self, id, data),
            ProtocolState::Status => status::handle_packet(self, id, data),
            ProtocolState::Login => login::handle_packet(self, id, data).await,
            ProtocolState::Config => config::handle_packet(self, id, data),
            ProtocolState::Play => {
                let player = {
                    let player = self.player.lock();
                    player.clone().unwrap()
                };

                play::handle_packet(player, id, data)
            }
        }
    }
}
