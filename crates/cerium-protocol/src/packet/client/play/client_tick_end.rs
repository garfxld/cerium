use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("client_tick_end")]
pub struct ClientTickEndPacket {
    // Empty
}

impl ClientPacket for ClientTickEndPacket {}

impl Decode for ClientTickEndPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}

impl Encode for ClientTickEndPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        Ok(())
    }
}
