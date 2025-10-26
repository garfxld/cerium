use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    text::Component,
};

#[derive(Debug, Clone)]
pub struct SetTablistHeaderFooterPacket {
    pub header: Component,
    pub footer: Component,
}

impl Packet for SetTablistHeaderFooterPacket {}
impl ServerPacket for SetTablistHeaderFooterPacket {}

impl Encode for SetTablistHeaderFooterPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_component(&this.header)?;
        w.write_component(&this.footer)?;
        Ok(())
    }
}
