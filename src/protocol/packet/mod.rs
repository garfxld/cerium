#![allow(unused)] // todo: remove
#![cfg_attr(rustfmt, rustfmt_skip)] // prevent re-ordering packet defenitions

pub mod client {

    pub mod handshake {
        pub mod handshake_packet;

        pub use handshake_packet::HandshakePacket;
    }

    pub mod status {
        pub mod status_request_packet;
        pub mod ping_request_packet;

        pub use status_request_packet::StatusRequestPacket;
        pub use ping_request_packet::PingRequestPacket;
    }

    pub mod login {}

    pub mod config {}

    pub mod play {}

    pub use handshake::*;
    pub use status::*;
    pub use login::*;
    pub use config::*;
    pub use play::*;
}

pub mod server {

    pub mod handshake {
        // Empty
    }

    pub mod status {
        pub mod status_response_packet;
        pub mod pong_response_packet;

        pub use status_response_packet::StatusResponsePacket;
        pub use pong_response_packet::PongResponsePacket;
    }

    pub mod login {}

    pub mod config {}

    pub mod play {}

    pub use handshake::*;
    pub use status::*;
    pub use login::*;
    pub use config::*;
    pub use play::*;
}

pub use client::*;
pub use server::*;
