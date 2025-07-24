use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("accept_teleportation")]
pub struct ConfirmTeleportationPacket {
    teleport_id: i32,
}

impl Decode for ConfirmTeleportationPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            teleport_id: buffer.read_varint()?,
        })
    }
}
