use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("set_carried_item", 0x34)]
pub struct SetHeldItemPacket {
    pub slot: i16,
}

impl ClientPacket for SetHeldItemPacket {}

impl Decode for SetHeldItemPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            slot: r.read_i16()?,
        })
    }
}

impl Encode for SetHeldItemPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_i16(this.slot)?;
        Ok(())
    }
}
