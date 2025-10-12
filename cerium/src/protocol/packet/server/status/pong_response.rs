use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct PongResponsePacket {
    pub timestamp: i64,
}

impl Packet for PongResponsePacket {}
impl ServerPacket for PongResponsePacket {}

impl Encode for PongResponsePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_i64(this.timestamp)?;
        Ok(())
    }
}
