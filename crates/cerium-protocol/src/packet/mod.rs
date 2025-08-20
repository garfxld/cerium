#![allow(unused, ambiguous_glob_reexports)]
#![cfg_attr(rustfmt, rustfmt_skip)] // prevent re-ordering packet defenitions

pub mod client {

    pub mod handshake {
        mod handshake;

        pub use handshake::HandshakePacket;
    }

    pub mod status {
        mod status_request;
        mod ping_request;

        pub use status_request::StatusRequestPacket;
        pub use ping_request::PingRequestPacket;
    }

    pub mod login {
        mod login_start;
        mod login_acknowledged;
        mod encryption_response;

        pub use login_start::LoginStartPacket;
        pub use login_acknowledged::LoginAcknowledgePacket;
        pub use encryption_response::EncryptionResponsePacket;
    }

    pub mod config {
        mod client_info;
        mod client_known_packs;
        mod acknowledge_finish_config;

        pub use client_info::ClientInfoPacket;
        pub use client_known_packs::ClientKnownPacksPacket;
        pub use acknowledge_finish_config::AcknowledgeFinishConfigPacket;
    }

    pub mod play {
        mod client_tick_end;
        mod confirm_teleportation;
        mod player_position_and_rotation;
        mod player_position;
        mod player_input;
        mod player_loaded;
        mod close_container;
        mod click_container;
        mod player_rotation;
        mod player_action;
        mod swing_arm;
        mod use_item_on;
        mod player_abilities;
        mod pick_item_from_block;
        mod set_held_item;
        mod player_command;
        mod chat_command;
        mod player_session;
        mod player_movement_flags;
        mod chunk_batch_received;

        pub use client_tick_end::ClientTickEndPacket;
        pub use confirm_teleportation::ConfirmTeleportationPacket;
        pub use player_position_and_rotation::PlayerPositionAndRotationPacket;
        pub use player_position::PlayerPositionPacket;
        pub use player_input::PlayerInputPacket;
        pub use player_loaded::PlayerLoadedPacket;
        pub use close_container::CloseContainerPacket;
        pub use click_container::ClickContainerPacket;
        pub use player_rotation::PlayerRotationPacket;
        pub use player_action::PlayerActionPacket;
        pub use swing_arm::SwingArmPacket;
        pub use use_item_on::UseItemOnPacket;
        pub use player_abilities::PlayerAbilitiesPacket;
        pub use pick_item_from_block::PickItemFromBlockPacket;
        pub use set_held_item::SetHeldItemPacket;
        pub use player_command::PlayerCommandPacket;
        pub use chat_command::ChatCommandPacket;
        pub use player_session::PlayerSessionPacket;
        pub use player_movement_flags::PlayerMovementFlagsPacket;
        pub use chunk_batch_received::ChunkBatchReceivedPacket;
    }

    pub mod common {
        mod client_plugin_message;

        pub use client_plugin_message::ClientPluginMessagePacket;
    }
 
    pub use handshake::*;
    pub use status::*;
    pub use login::*;
    pub use config::*;
    pub use play::*;
    pub use common::*;
}

pub mod server {

    pub mod handshake {
        // Empty
    }

    pub mod status {
        mod status_response;
        mod pong_response;

        pub use status_response::StatusResponsePacket;
        pub use pong_response::PongResponsePacket;
    }

    pub mod login {
        mod login_disconnect;
        mod login_success;
        mod encryption_request;
        mod set_compression;

        pub use login_disconnect::LoginDisconnectPacket;
        pub use login_success::*;
        pub use encryption_request::*;
        pub use set_compression::SetCompressionPacket;
    }

    pub mod config {
        mod server_known_packs;
        mod registry_data;
        mod finish_config;

        pub use server_known_packs::ServerKnownPacksPacket;
        pub use registry_data::*;
        pub use finish_config::FinishConfigPacket;
    }

    pub mod play {
        mod login;
        mod sync_player_position;
        mod game_event;
        mod set_center_chunk;
        mod chunk_data_and_update_light;
        mod player_info_update;
        mod chunk_batch_start;
        mod chunk_batch_finished;
        mod unload_chunk;
        mod entity_position;
        mod entity_position_rotation;

        pub use login::LoginPacket;
        pub use sync_player_position::SyncPlayerPositionPacket;
        pub use game_event::GameEventPacket;
        pub use set_center_chunk::SetCenterChunkPacket;
        pub use chunk_data_and_update_light::*;
        pub use player_info_update::*;
        pub use chunk_batch_start::ChunkBatchStartPacket;
        pub use chunk_batch_finished::ChunkBatchFinishedPacket;
        pub use unload_chunk::UnloadChunkPacket;
        pub use entity_position::EntityPositionPacket;
        pub use entity_position_rotation::EntityPositionRotationPacket;
    }

    pub mod common {}

    pub use handshake::*;
    pub use status::*;
    pub use login::*;
    pub use config::*;
    pub use play::*;
    pub use common::*;
}

pub use client::*;
pub use server::*;

pub mod keep_alive;
pub use keep_alive::KeepAlivePacket;


pub trait Packet {
}

#[derive(Debug, Clone)]
pub struct RawPacket {
    id: i32,
    data: Vec<u8>
}

impl RawPacket {
    pub fn new(id: i32, data: Vec<u8>) -> Self {
        Self { id, data }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
