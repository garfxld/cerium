use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug)]
#[packet("move_entity_pos")]
pub struct EntityPositionPacket {
    pub entitiy_id: i32,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub on_ground: bool,
}

impl Encode for EntityPositionPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_varint(this.entitiy_id)?;
        buffer.write_i16(this.delta_x)?;
        buffer.write_i16(this.delta_y)?;
        buffer.write_i16(this.delta_z)?;
        buffer.write_bool(this.on_ground)?;
        Ok(())
    }
}
