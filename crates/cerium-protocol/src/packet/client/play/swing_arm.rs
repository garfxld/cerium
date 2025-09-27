use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("swing")]
pub struct SwingArmPacket {
    pub hand: i32, // VarInt Enum (Hand)
}

impl ClientPacket for SwingArmPacket {}

impl Decode for SwingArmPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            hand: r.read_varint()?,
        })
    }
}

impl Encode for SwingArmPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_varint(this.hand)?;
        Ok(())
    }
}
