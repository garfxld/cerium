use crate::protocol::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeException},
};

#[derive(Debug, Clone)]
pub struct LightData {}

impl Encode for LightData {
    fn encode(buffer: &mut ByteBuffer, _this: Self) -> Result<(), EncodeException> {
        buffer.write_varint(0)?; // sky_light
        buffer.write_varint(0)?; // block_light
        buffer.write_varint(0)?; // empty_sky_light
        buffer.write_varint(0)?; // empty_block_light

        buffer.write_varint(0)?;
        buffer.write_varint(0)?;
        Ok(())
    }
}
