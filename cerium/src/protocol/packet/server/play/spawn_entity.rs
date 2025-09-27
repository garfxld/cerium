use cerium_protocol_macros::packet;
use uuid::Uuid;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ServerPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("add_entity", 0x01)]
pub struct SpawnEntityPacket {
    pub id: i32,
    pub uuid: Uuid,
    pub entity_type: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: u8,
    pub yaw: u8,
    pub head_yaw: u8,
    pub data: i32,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

impl ServerPacket for SpawnEntityPacket {}

impl Decode for SpawnEntityPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            id:          r.read_varint()?,
            uuid:        r.read_uuid()?,
            entity_type: r.read_varint()?,
            x:           r.read_f64()?,
            y:           r.read_f64()?,
            z:           r.read_f64()?,
            pitch:       r.read_u8()?,
            yaw:         r.read_u8()?,
            head_yaw:    r.read_u8()?,
            data:        r.read_varint()?,
            velocity_x:  r.read_i16()?,
            velocity_y:  r.read_i16()?,
            velocity_z:  r.read_i16()?,
        })
    }
}

impl Encode for SpawnEntityPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_varint(this.id)?;
        w.write_uuid(this.uuid)?;
        w.write_varint(this.entity_type)?;
        w.write_f64(this.x)?;
        w.write_f64(this.y)?;
        w.write_f64(this.z)?;
        w.write_u8(this.pitch)?;
        w.write_u8(this.yaw)?;
        w.write_u8(this.head_yaw)?;
        w.write_varint(this.data)?;
        w.write_i16(this.velocity_x)?;
        w.write_i16(this.velocity_y)?;
        w.write_i16(this.velocity_z)?;
        Ok(())
    }
}
