use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("finish_configuration", 0x03)]
pub struct AcknowledgeFinishConfigPacket {
    // Empty
}

impl ClientPacket for AcknowledgeFinishConfigPacket {}

impl Decode for AcknowledgeFinishConfigPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}

impl Encode for AcknowledgeFinishConfigPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        Ok(())
    }
}
