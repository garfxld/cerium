use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct GameEventPacket {
    pub event: u8,
    pub value: f32,
}

impl Packet for GameEventPacket {}
impl ServerPacket for GameEventPacket {}

impl Encode for GameEventPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_u8(this.event)?;
        w.write_f32(this.value)?;
        Ok(())
    }
}
