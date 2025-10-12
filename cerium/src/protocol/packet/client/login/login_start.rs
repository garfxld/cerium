use uuid::Uuid;

use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: Uuid,
}

impl Packet for LoginStartPacket {}
impl ClientPacket for LoginStartPacket {}

impl Decode for LoginStartPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            name: r.read_string()?,
            uuid: r.read_uuid()?,
        })
    }
}
