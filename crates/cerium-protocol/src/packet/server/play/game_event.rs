use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug)]
#[packet("game_event")]
pub struct GameEventPacket {
    pub event: u8,
    pub value: f32,
}

impl Encode for GameEventPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_u8(this.event)?;
        buffer.write_f32(this.value)?;
        Ok(())
    }
}
