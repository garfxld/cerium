use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeException},
};

#[derive(Debug)]
#[packet("swing")]
pub struct SwingArmPacket {
    pub hand: i32, // VarInt Enum (Hand)
}

impl Decode for SwingArmPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {
            hand: buffer.read_varint()?,
        })
    }
}
