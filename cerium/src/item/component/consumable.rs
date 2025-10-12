use serde::{Deserialize, Serialize};

use crate::protocol::{
    DataType,
    decode::{DecodeError, PacketRead},
    encode::{EncodeError, PacketWrite},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consumable {
    consume_seconds: f32,
    animation: i32, // VarInt Enum
    sound: (),      // IdOr SoundEvent
    has_consume_particles: bool,
    effects: Vec<ConsumeEffect>, // Vec<ConsumeEffect>
}

impl DataType for Consumable {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        todo!()
    }

    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConsumeEffect {}

impl DataType for ConsumeEffect {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        todo!()
    }

    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        todo!()
    }
}
