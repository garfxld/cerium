use crate::{
    network::auth::Property,
    protocol::{
        buffer::ByteBuffer,
        encode::{Encode, EncodeException},
    },
};



impl Encode for Property {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException> {
        buffer.write_string(this.name)?;
        buffer.write_string(this.value)?;
        buffer.write_optional(this.signature, |buffer, value| buffer.write_string(value))?;
        Ok(())
    }
}