use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
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

impl Packet for SyncPlayerPositionPacket {}
impl ServerPacket for SyncPlayerPositionPacket {}

impl Encode for SyncPlayerPositionPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
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
