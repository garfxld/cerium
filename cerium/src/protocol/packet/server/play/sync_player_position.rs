use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("player_position", 0x41)]
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

impl Decode for SyncPlayerPositionPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            teleport_id: r.read_varint()?,
            x:           r.read_f64()?,
            y:           r.read_f64()?,
            z:           r.read_f64()?,
            velocity_x:  r.read_f64()?,
            velocity_y:  r.read_f64()?,
            velocity_z:  r.read_f64()?,
            yaw:         r.read_f32()?,
            pitch:       r.read_f32()?,
            flags:       r.read_i32()?,
        })
    }
}

impl Encode for SyncPlayerPositionPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_varint(this.teleport_id)?;
        w.write_f64(this.x)?;
        w.write_f64(this.y)?;
        w.write_f64(this.z)?;
        w.write_f64(this.velocity_x)?;
        w.write_f64(this.velocity_y)?;
        w.write_f64(this.velocity_z)?;
        w.write_f32(this.yaw)?;
        w.write_f32(this.pitch)?;
        w.write_i32(this.flags)?; // Not a VarInt!
        Ok(())
    }
}
