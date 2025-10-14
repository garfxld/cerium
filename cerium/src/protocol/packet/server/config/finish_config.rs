use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct FinishConfigPacket {
    // Empty
}

impl Packet for FinishConfigPacket {}
impl ServerPacket for FinishConfigPacket {}

impl Encode for FinishConfigPacket {
    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        Ok(())
    }
}
