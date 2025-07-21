use crate::protocol::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug)]
pub struct Heightmap {
    kind: i32,
    data: Vec<i64>,
}

impl Encode for Heightmap {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_varint(this.kind)?;
        buffer.write_array(this.data, |buffer, value| buffer.write_i64(value))?;
        Ok(())
    }
}
