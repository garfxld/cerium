use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct EntityAnimationPacket {
    pub entity_id: i32,
    pub animation: u8,
}

impl Packet for EntityAnimationPacket {}
impl ServerPacket for EntityAnimationPacket {}

impl Encode for EntityAnimationPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.entity_id)?;
        w.write_u8(this.animation)?;
        Ok(())
    }
}
