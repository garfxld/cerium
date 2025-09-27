use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("move_player_status_only", 0x20)]
pub struct PlayerMovementFlagsPacket {
    pub flags: u8,
}

impl ClientPacket for PlayerMovementFlagsPacket {}

impl Decode for PlayerMovementFlagsPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            flags: r.read_u8()?,
        })
    }
}

impl Encode for PlayerMovementFlagsPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_u8(this.flags)?;
        Ok(())
    }
}
