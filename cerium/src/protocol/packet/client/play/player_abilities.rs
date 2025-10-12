use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PlayerAbilitiesPacket {
    pub flags: i8,
}

impl Packet for PlayerAbilitiesPacket {}
impl ClientPacket for PlayerAbilitiesPacket {}

impl Decode for PlayerAbilitiesPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            flags: r.read_i8()?,
        })
    }
}
