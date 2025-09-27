use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("player_loaded")]
pub struct PlayerLoadedPacket {
    // Empty
}

impl ClientPacket for PlayerLoadedPacket {}

impl Decode for PlayerLoadedPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}

impl Encode for PlayerLoadedPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        Ok(())
    }
}
