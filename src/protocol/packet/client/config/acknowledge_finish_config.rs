use crate::protocol::{buffer::ByteBuffer, decode::{Decode, DecodeError}};

#[derive(Debug)]
pub struct AcknowledgeFinishConfigPacket {
    // Empty
}

impl Decode for AcknowledgeFinishConfigPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}
