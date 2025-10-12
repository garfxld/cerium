use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct CloseContainerPacket {
    pub window_id: i32,
}

impl Packet for CloseContainerPacket {}
impl ClientPacket for CloseContainerPacket {}

impl Decode for CloseContainerPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(CloseContainerPacket {
            window_id: r.read_varint()?,
        })
    }
}
