use bytes::{Buf, BufMut};
use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeException},
    encode::{Encode, EncodeException},
};

#[derive(Debug)]
#[packet("keep_alive")]
pub struct KeepAlivePacket {
    pub keep_alive_id: i64,
}

impl Decode for KeepAlivePacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {
            keep_alive_id: buffer.read_i64()?,
        })
    }
}

impl Encode for KeepAlivePacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException> {
        buffer.write_i64(this.keep_alive_id)?;
        Ok(())
    }
}
