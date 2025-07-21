use crate::protocol::buffer::ByteBuffer;

#[derive(Debug, Clone)]
pub struct DecodeError;

pub trait Decode
where
    Self: Sized,
{
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError>;
}
