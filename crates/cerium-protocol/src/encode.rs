use crate::buffer::ByteBuffer;

#[derive(Debug, Clone)]
pub struct EncodeError;

pub trait Encode {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError>;
}
