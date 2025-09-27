use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("player_abilities")]
pub struct PlayerAbilitiesPacket {
    pub flags: i8,
}

impl ClientPacket for PlayerAbilitiesPacket {}

impl Decode for PlayerAbilitiesPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            flags: r.read_i8()?,
        })
    }
}

impl Encode for PlayerAbilitiesPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_i8(this.flags)?;
        Ok(())
    }
}
