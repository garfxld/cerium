use std::sync::Arc;

use crate::{
    network::client::ClientConnection,
    protocol::{buffer::ByteBuffer, decode::DecodeError, ProtocolState},
};

mod config;
mod handshake;
mod login;
mod play;
mod status;

impl ClientConnection {
    pub async fn handle_packet(
        self: Arc<Self>,
        id: i32,
        data: &mut ByteBuffer,
    ) -> Result<(), DecodeError> {
        let state = *self.state.lock().await;
        match state {
            ProtocolState::Handshake => handshake::handle_packet(self, id, data).await,
            ProtocolState::Status => status::handle_packet(self, id, data).await,
            ProtocolState::Login => login::handle_packet(self, id, data).await,
            ProtocolState::Config => config::handle_packet(self, id, data).await,
            ProtocolState::Play => play::handle_packet(self, id, data).await,
        }
    }
}
