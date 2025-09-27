use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("player_command", 0x29)]
pub struct PlayerCommandPacket {
    pub entity_id: i32,
    pub action_id: i32,
    pub jump_boost: i32,
}

impl ClientPacket for PlayerCommandPacket {}

impl Decode for PlayerCommandPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            entity_id: r.read_varint()?,
            action_id: r.read_varint()?,
            jump_boost: r.read_varint()?,
        })
    }
}

impl Encode for PlayerCommandPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_varint(this.entity_id)?;
        w.write_varint(this.action_id)?;
        w.write_varint(this.jump_boost)?;
        Ok(())
    }
}
