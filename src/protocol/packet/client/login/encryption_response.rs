use bytes::Buf;
use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeException},
};

#[derive(Debug, Clone)]
#[packet("key")]
pub struct EncryptionResponsePacket {
    pub shared_secret: Box<[u8]>,
    pub verify_token: Box<[u8]>,
}

impl Decode for EncryptionResponsePacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {
            shared_secret: buffer
                .read_list(|buffer| buffer.read_u8())?
                .into_boxed_slice(),
            verify_token: buffer
                .read_list(|buffer| buffer.read_u8())?
                .into_boxed_slice(),
        })
    }
}
