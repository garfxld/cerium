use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct SetHeldItemPacket {
    pub slot: i16,
}

impl Packet for SetHeldItemPacket {}
impl ClientPacket for SetHeldItemPacket {}

impl Decode for SetHeldItemPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            slot: r.read_i16()?,
        })
    }
}
