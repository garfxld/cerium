use crate::network::client::ClientConnection;
use crate::protocol::{ProtocolState, decode::DecodeError};
use bytes::Bytes;
use std::sync::Arc;

mod config;
mod handshake;
mod login;
mod play;
mod status;

impl ClientConnection {
    pub async fn handle_packet(
        self: Arc<Self>,
        id: i32,
        data: &mut Bytes,
    ) -> Result<(), DecodeError> {
        let state = *self.state.lock().await;
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
