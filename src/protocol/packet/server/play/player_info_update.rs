use bitflags::bitflags;
use uuid::Uuid;

use crate::{
    network::auth::Property,
    protocol::{
        buffer::ByteBuffer,
        encode::{Encode, EncodeError},
    },
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PlayerInfoFlags: u8 {
        const ADD_PLAYER            = 0x01;
        const INITIALIZE_CHAT       = 0x02;
        const UPDATE_GAME_MODE      = 0x04;
        const UPDATE_LISTED         = 0x08;
        const UPDATE_LATENCY        = 0x10;
        const UPDATE_DISPLAY_NAME   = 0x20;
        const UPDATE_LIST_PRIORITY  = 0x40;
        const UPDATE_HAT            = 0x80;
    }
}

#[derive(Debug, Clone)]
pub struct PlayerInfoUpdatePacket {
    pub actions: u8,
    pub players: Vec<PlayerEntry>,
}

impl Encode for PlayerInfoUpdatePacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_u8(this.actions)?;
        buffer.write_array(this.players, |buffer, value| {
            PlayerEntry::encode(buffer, value)
        })?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PlayerEntry {
    pub uuid: Uuid,
    pub player_actions: Vec<PlayerAction>,
}

impl Encode for PlayerEntry {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_uuid(this.uuid)?;
        buffer.write_unprefixed_array(this.player_actions, |buffer, value| match value {
            PlayerAction::AddPlayer { name, properties } => {
                buffer.write_string(name)?;
                buffer.write_array(properties, |buffer, value| Property::encode(buffer, value))?;
                Ok(())
            }
            PlayerAction::UpdateListed { listed } => buffer.write_bool(listed),
            _ => todo!(),
        });
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum PlayerAction {
    AddPlayer {
        name: String,
        properties: Vec<Property>,
    },
    InitializeChat {}, // todo
    UpdateGameMode {
        game_mode: i32,
    },
    UpdateListed {
        listed: bool,
    },
    UpdateLatency {
        ping: i32,
    },
    UpdateDisplayName {}, // todo
    UpdateListPriority {
        priority: i32,
    },
    UpdateHat {
        visible: bool,
    },
}
