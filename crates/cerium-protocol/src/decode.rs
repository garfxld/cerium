use thiserror::Error;

use crate::buffer::ByteBuffer;

#[derive(Debug, Clone, Error)]
pub enum DecodeError {
    #[error("{0}")]
    Decode(String),
    #[error("Unknown Packet: {0}")]
    UnkownPacket(i32),
}

pub trait Decode
where
    Self: Sized,
{
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError>;
}
