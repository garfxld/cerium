use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    util::Position,
};

#[derive(Debug, Clone)]
pub struct EntityRotationPacket {
    pub entity_id: i32,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}

impl EntityRotationPacket {
    pub fn new(
        entity_id: i32,
        new_position: Position,
        old_position: Position,
        on_ground: bool,
    ) -> Self {
        let _ = old_position;
        Self {
            entity_id,
            yaw: (new_position.yaw() * 256. / 360.) as i8,
            pitch: (new_position.pitch() * 256. / 360.) as i8,
            on_ground,
        }
    }
}

impl Packet for EntityRotationPacket {}
impl ServerPacket for EntityRotationPacket {}

impl Encode for EntityRotationPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.entity_id)?;
        w.write_i8(this.yaw)?;
        w.write_i8(this.pitch)?;
        w.write_bool(this.on_ground)?;
        Ok(())
    }
}
