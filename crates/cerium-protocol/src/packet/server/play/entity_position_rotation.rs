use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug)]
#[packet("move_entity_pos_rot")]
pub struct EntityPositionRotationPacket {
    pub entitiy_id: i32,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub yaw: u8,   // Angle (1/256)
    pub pitch: u8, // Angle (1/256)
    pub on_ground: bool,
}

impl Encode for EntityPositionRotationPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_varint(this.entitiy_id)?;
        buffer.write_i16(this.delta_x)?;
        buffer.write_i16(this.delta_y)?;
        buffer.write_i16(this.delta_z)?;
        buffer.write_u8(this.yaw)?;
        buffer.write_u8(this.pitch)?;
        buffer.write_bool(this.on_ground)?;
        Ok(())
    }
}
