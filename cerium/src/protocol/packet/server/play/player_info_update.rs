use bitflags::bitflags;
use uuid::Uuid;

use crate::{
    auth::Property,
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
};

#[derive(Debug, Clone)]
pub struct PlayerInfoUpdatePacket {
    pub actions: u8,
    pub players: Vec<PlayerEntry>,
}

impl Packet for PlayerInfoUpdatePacket {}
impl ServerPacket for PlayerInfoUpdatePacket {}

impl Encode for PlayerInfoUpdatePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_u8(this.actions)?;
        w.write_array(&this.players, PlayerEntry::encode)?;
        Ok(())
    }
}

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
pub struct PlayerEntry {
    pub uuid: Uuid,
    pub player_actions: Vec<PlayerAction>,
}

impl Encode for PlayerEntry {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_uuid(&this.uuid)?;
        w.write_unprefixed_array(&this.player_actions, |buffer, value| match value {
            PlayerAction::AddPlayer { name, properties } => {
                buffer.write_string(name)?;
                buffer.write_array(properties, Property::encode)?;
                Ok(())
            }
            PlayerAction::UpdateListed { listed } => buffer.write_bool(*listed),
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
