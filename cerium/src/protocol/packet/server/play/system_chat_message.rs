use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    text::Component,
};

#[derive(Debug, Clone)]
pub struct SystemChatMessagePacket {
    pub content: Component,
    pub overlay: bool,
}

impl Packet for SystemChatMessagePacket {}
impl ServerPacket for SystemChatMessagePacket {}

impl Encode for SystemChatMessagePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_component(&this.content)?;
        w.write_bool(this.overlay)?;
        Ok(())
    }
}
