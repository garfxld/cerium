use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    util::{Position, TeleportFlags},
};

#[derive(Debug, Clone)]
pub struct SyncPlayerPositionPacket {
    pub teleport_id: i32,
    pub position: Position,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: TeleportFlags,
}

impl Packet for SyncPlayerPositionPacket {}
impl ServerPacket for SyncPlayerPositionPacket {}

impl Encode for SyncPlayerPositionPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.teleport_id)?;
        w.write_f64(this.position.x())?;
        w.write_f64(this.position.y())?;
        w.write_f64(this.position.z())?;
        w.write_f64(this.velocity_x)?;
        w.write_f64(this.velocity_y)?;
        w.write_f64(this.velocity_z)?;
        w.write_f32(this.yaw)?;
        w.write_f32(this.pitch)?;
        w.write_i32(this.flags.bits())?;
        Ok(())
    }
}
