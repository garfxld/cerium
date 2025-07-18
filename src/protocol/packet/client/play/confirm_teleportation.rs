use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeException},
};

#[derive(Debug)]
#[packet("accept_teleportation")]
pub struct ConfirmTeleportationPacket {
    teleport_id: i32,
}

impl Decode for ConfirmTeleportationPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {
            teleport_id: buffer.read_varint()?,
        })
    }
}
