use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct ConfirmTeleportationPacket {
    pub teleport_id: i32,
}

impl Packet for ConfirmTeleportationPacket {}
impl ClientPacket for ConfirmTeleportationPacket {}

impl Decode for ConfirmTeleportationPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            teleport_id: r.read_varint()?,
        })
    }
}
