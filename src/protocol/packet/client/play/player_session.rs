use bytes::Buf;
use macros::packet;
use uuid::Uuid;

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeException},
};

#[derive(Debug, Clone)]
#[packet("chat_session_update")]
pub struct PlayerSessionPacket {
    pub session_id: Uuid,

    // publick key
    pub expires_at: i64,
    pub public_key: Vec<u8>,
    pub key_signature: Vec<u8>,
}

impl Decode for PlayerSessionPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {
            session_id: buffer.read_uuid()?,

            // public key
            expires_at: buffer.read_i64()?,
            public_key: buffer.read_list(|buffer| buffer.read_u8())?,
            key_signature: buffer.read_list(|buffer| buffer.read_u8())?,
        })
    }
}
