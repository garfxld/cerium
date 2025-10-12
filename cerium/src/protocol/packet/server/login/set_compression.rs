use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct SetCompressionPacket {
    pub threshold: i32,
}

impl Packet for SetCompressionPacket {}
impl ServerPacket for SetCompressionPacket {}

impl Encode for SetCompressionPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.threshold)?;
        Ok(())
    }
}
