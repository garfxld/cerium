use crate::protocol::buffer::ByteBuffer;

#[derive(Debug, Clone)]
pub struct DecodeException;

pub trait Decode
where
    Self: Sized,
{
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException>;
}
