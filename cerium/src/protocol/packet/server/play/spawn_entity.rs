use uuid::Uuid;

use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    util::Position,
};

#[derive(Debug, Clone)]
pub struct SpawnEntityPacket {
    pub id: i32,
    pub uuid: Uuid,
    pub entity_type: i32,
    pub position: Position,
    pub head_yaw: f32,
    pub data: i32,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

impl Packet for SpawnEntityPacket {}
impl ServerPacket for SpawnEntityPacket {}

impl Encode for SpawnEntityPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.id)?;
        w.write_uuid(&this.uuid)?;
        w.write_varint(this.entity_type)?;
        w.write_f64(this.position.x())?;
        w.write_f64(this.position.y())?;
        w.write_f64(this.position.z())?;
        w.write_u8(0)?; // Velocity
        w.write_i8((this.position.yaw() * 256. / 360.) as i8)?;
        w.write_i8((this.position.pitch() * 256. / 360.) as i8)?;
        w.write_i8((this.head_yaw * 256. / 360.) as i8)?;
        w.write_varint(this.data)?;
        Ok(())
    }
}
