use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug, Clone)]
#[packet("key")]
pub struct EncryptionResponsePacket {
    pub shared_secret: Box<[u8]>,
    pub verify_token: Box<[u8]>,
}

impl Decode for EncryptionResponsePacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
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
