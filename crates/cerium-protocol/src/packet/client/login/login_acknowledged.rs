use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("login_acknowledged")]
pub struct LoginAcknowledgePacket {
    // Empty
}

impl ClientPacket for LoginAcknowledgePacket {}

impl Decode for LoginAcknowledgePacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}

impl Encode for LoginAcknowledgePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        Ok(())
    }
}
