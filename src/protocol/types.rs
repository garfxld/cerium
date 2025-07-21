use crate::{
    network::auth::Property,
    protocol::{
        buffer::ByteBuffer,
        encode::{Encode, EncodeError},
    },
};

impl Encode for Property {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_string(this.name)?;
        buffer.write_string(this.value)?;
        buffer.write_optional(this.signature, |buffer, value| buffer.write_string(value))?;
        Ok(())
    }
}
