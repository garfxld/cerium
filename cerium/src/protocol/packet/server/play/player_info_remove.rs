use uuid::Uuid;

use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct PlayerInfoRemovePacket {
    pub uuids: Vec<Uuid>,
}

impl Packet for PlayerInfoRemovePacket {}
impl ServerPacket for PlayerInfoRemovePacket {}

impl Encode for PlayerInfoRemovePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_array(&this.uuids, |w, v| w.write_uuid(v))?;
        Ok(())
    }
}
