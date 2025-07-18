use crate::protocol::{buffer::ByteBuffer, decode::{Decode, DecodeException}};

#[derive(Debug)]
pub struct AcknowledgeFinishConfigPacket {
    // Empty
}

impl Decode for AcknowledgeFinishConfigPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {})
    }
}
