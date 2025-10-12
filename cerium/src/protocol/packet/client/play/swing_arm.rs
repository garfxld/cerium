use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct SwingArmPacket {
    pub hand: i32,
}

impl Packet for SwingArmPacket {}
impl ClientPacket for SwingArmPacket {}

impl Decode for SwingArmPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            hand: r.read_varint()?,
        })
    }
}
