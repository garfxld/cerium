use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("finish_configuration")]
pub struct FinishConfigPacket {
    // Empty
}

impl Decode for FinishConfigPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}

impl Encode for FinishConfigPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        Ok(())
    }
}
