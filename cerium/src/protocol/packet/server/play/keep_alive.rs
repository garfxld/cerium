use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct KeepAlivePacket {
    pub keep_alive_id: i64,
}

impl Packet for KeepAlivePacket {}
impl ServerPacket for KeepAlivePacket {}

impl Encode for KeepAlivePacket {
    fn encode<W: PacketWrite>(buffer: &mut W, this: &Self) -> Result<(), EncodeError> {
        buffer.write_i64(this.keep_alive_id)?;
        Ok(())
    }
}
