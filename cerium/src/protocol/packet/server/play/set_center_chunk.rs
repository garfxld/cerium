use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct SetCenterChunkPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

impl Packet for SetCenterChunkPacket {}
impl ServerPacket for SetCenterChunkPacket {}

impl Encode for SetCenterChunkPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.chunk_x)?;
        w.write_varint(this.chunk_z)?;
        Ok(())
    }
}
