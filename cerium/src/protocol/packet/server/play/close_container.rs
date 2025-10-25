use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct CloseContainerPacket {
    pub window_id: i32,
}

impl Packet for CloseContainerPacket {}
impl ServerPacket for CloseContainerPacket {}

impl Encode for CloseContainerPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.window_id)?;
        Ok(())
    }
}
