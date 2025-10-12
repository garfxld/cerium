use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PickItemFromBlockPacket {
    pub position: i64,
    pub include_data: bool,
}

impl Packet for PickItemFromBlockPacket {}
impl ClientPacket for PickItemFromBlockPacket {}

impl Decode for PickItemFromBlockPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            position:     r.read_i64()?,
            include_data: r.read_bool()?,
        })
    }
}
