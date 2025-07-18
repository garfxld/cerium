use crate::protocol::buffer::ByteBuffer;

#[derive(Debug, Clone)]
pub struct EncodeException;

pub trait Encode {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException>;
}
