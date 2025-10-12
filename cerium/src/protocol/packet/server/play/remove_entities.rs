use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct RemoveEntitiesPacket {
    pub entity_ids: Vec<i32>,
}

impl Packet for RemoveEntitiesPacket {}
impl ServerPacket for RemoveEntitiesPacket {}

impl Encode for RemoveEntitiesPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_array(&this.entity_ids, |w, v| w.write_varint(*v))?;
        Ok(())
    }
}
