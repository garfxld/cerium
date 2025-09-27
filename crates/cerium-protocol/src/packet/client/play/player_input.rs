use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("player_input", 0x2A)]
pub struct PlayerInputPacket {
    pub flags: u8,
}

impl ClientPacket for PlayerInputPacket {}

impl Decode for PlayerInputPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            flags: r.read_u8()?,
        })
    }
}

impl Encode for PlayerInputPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_u8(this.flags)?;
        Ok(())
    }
}
