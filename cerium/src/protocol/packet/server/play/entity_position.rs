use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct EntityPositionPacket {
    pub entitiy_id: i32,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub on_ground: bool,
}

impl Packet for EntityPositionPacket {}
impl ServerPacket for EntityPositionPacket {}

impl Encode for EntityPositionPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.entitiy_id)?;
        w.write_i16(this.delta_x)?;
        w.write_i16(this.delta_y)?;
        w.write_i16(this.delta_z)?;
        w.write_bool(this.on_ground)?;
        Ok(())
    }
}
