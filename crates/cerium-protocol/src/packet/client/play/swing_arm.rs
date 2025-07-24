use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("swing")]
pub struct SwingArmPacket {
    pub hand: i32, // VarInt Enum (Hand)
}

impl Decode for SwingArmPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            hand: buffer.read_varint()?,
        })
    }
}
