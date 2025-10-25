use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct SetHeadRotationPacket {
    pub entity_id: i32,
    pub angle: i8,
}

impl SetHeadRotationPacket {
    pub fn new(entity_id: i32, head_rotation: f32) -> Self {
        Self {
            entity_id,
            angle: (head_rotation * 256. / 360.) as i8,
        }
    }
}

impl Packet for SetHeadRotationPacket {}
impl ServerPacket for SetHeadRotationPacket {}

impl Encode for SetHeadRotationPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.entity_id)?;
        w.write_i8(this.angle)?;
        Ok(())
    }
}
