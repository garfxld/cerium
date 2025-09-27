use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("accept_teleportation", 0x00)]
pub struct ConfirmTeleportationPacket {
    teleport_id: i32,
}

impl ClientPacket for ConfirmTeleportationPacket {}

impl Decode for ConfirmTeleportationPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            teleport_id: r.read_varint()?,
        })
    }
}

impl Encode for ConfirmTeleportationPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_varint(this.teleport_id)?;
        Ok(())
    }
}
