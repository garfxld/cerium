use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug)]
#[packet("player_position")]
pub struct SyncPlayerPositionPacket {
    pub teleport_id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: i32, // todo: change to flags type (https://minecraft.wiki/w/Java_Edition_protocol/Packets#Teleport_Flags)
}

impl Encode for SyncPlayerPositionPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_varint(this.teleport_id)?;
        buffer.write_f64(this.x)?;
        buffer.write_f64(this.y)?;
        buffer.write_f64(this.z)?;
        buffer.write_f64(this.velocity_x)?;
        buffer.write_f64(this.velocity_y)?;
        buffer.write_f64(this.velocity_z)?;
        buffer.write_f32(this.yaw)?;
        buffer.write_f32(this.pitch)?;
        buffer.write_i32(this.flags)?; // Not a VarInt!
        Ok(())
    }
}
