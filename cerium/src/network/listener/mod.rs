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
        let state = self.state().await;
        match state {
            ProtocolState::Handshake => handshake::handle_packet(self, id, data).await,
            ProtocolState::Status => status::handle_packet(self, id, data).await,
            ProtocolState::Login => login::handle_packet(self, id, data).await,
            ProtocolState::Config => config::handle_packet(self, id, data).await,
            ProtocolState::Play => {
                let player = self.player.lock().await;
                let player = player.clone().unwrap();

                play::handle_packet(player, id, data).await
            }
        }
    }
}
