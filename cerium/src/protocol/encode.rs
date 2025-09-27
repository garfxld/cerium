use crate::protocol::write::PacketWrite;

#[derive(Debug, Clone)]
pub struct EncodeError;

pub trait Encode {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError>;
}
