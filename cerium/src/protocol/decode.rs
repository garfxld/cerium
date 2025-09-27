use thiserror::Error;

use crate::protocol::read::PacketRead;

pub trait Decode
where
    Self: Sized,
{
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError>;
}

#[derive(Debug, Clone, Error)]
pub enum DecodeError {
    #[error("{0}")]
    Decode(String),
    #[error("Unknown Packet: {0}")]
    UnkownPacket(i32),
}
