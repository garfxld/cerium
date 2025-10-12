use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    text::Component,
};

#[derive(Debug, Clone)]
pub struct DisconnectPacket {
    pub reason: Component,
}

impl Packet for DisconnectPacket {}
impl ServerPacket for DisconnectPacket {}

impl Encode for DisconnectPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_component(&this.reason)?;
        Ok(())
    }
}
