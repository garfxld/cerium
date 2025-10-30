use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct AcknowledgeBlockChangePacket {
    pub sequence_id: i32,
}

impl Packet for AcknowledgeBlockChangePacket {}
impl ServerPacket for AcknowledgeBlockChangePacket {}

impl Encode for AcknowledgeBlockChangePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.sequence_id)?;
        Ok(())
    }
}
