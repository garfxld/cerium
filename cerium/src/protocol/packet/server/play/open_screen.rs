use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    text::Component,
};

#[derive(Debug, Clone)]
pub struct OpenScreenPacket {
    pub window_id: i32,
    pub window_type: i32,
    pub window_title: Component,
}

impl Packet for OpenScreenPacket {}
impl ServerPacket for OpenScreenPacket {}

impl Encode for OpenScreenPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.window_id)?;
        w.write_varint(this.window_type)?;
        w.write_component(&this.window_title)?;
        Ok(())
    }
}
