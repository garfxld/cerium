use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct EntityPositionRotationPacket {
    pub entitiy_id: i32,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub yaw: u8,   // Angle (1/256)
    pub pitch: u8, // Angle (1/256)
    pub on_ground: bool,
}

impl Packet for EntityPositionRotationPacket {}
impl ServerPacket for EntityPositionRotationPacket {}

impl Encode for EntityPositionRotationPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.entitiy_id)?;
        w.write_i16(this.delta_x)?;
        w.write_i16(this.delta_y)?;
        w.write_i16(this.delta_z)?;
        w.write_u8(this.yaw)?;
        w.write_u8(this.pitch)?;
        w.write_bool(this.on_ground)?;
        Ok(())
    }
}
