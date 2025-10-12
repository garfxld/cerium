use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct KeepAlivePacket {
    pub keep_alive_id: i64,
}

impl Packet for KeepAlivePacket {}
impl ClientPacket for KeepAlivePacket {}

impl Decode for KeepAlivePacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            keep_alive_id: r.read_i64()?,
        })
    }
}
