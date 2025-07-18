use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeException},
};

#[derive(Debug)]
#[packet("game_event")]
pub struct GameEventPacket {
    pub event: u8,
    pub value: f32,
}

impl Encode for GameEventPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException> {
        buffer.write_u8(this.event)?;
        buffer.write_f32(this.value)?;
        Ok(())
    }
}
