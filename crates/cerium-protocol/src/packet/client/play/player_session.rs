use cerium_protocol_macros::packet;
use uuid::Uuid;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("chat_session_update", 0x09)]
pub struct PlayerSessionPacket {
    pub session_id: Uuid,
    pub expires_at: i64,
    pub public_key: Vec<u8>,
    pub key_signature: Vec<u8>,
}

impl ClientPacket for PlayerSessionPacket {}

impl Decode for PlayerSessionPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            session_id:    r.read_uuid()?,
            expires_at:    r.read_i64()?,
            public_key:    r.read_array(|r| r.read_u8())?,
            key_signature: r.read_array(|r| r.read_u8())?,
        })
    }
}

impl Encode for PlayerSessionPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_uuid(this.session_id)?;
        w.write_i64(this.expires_at)?;
        w.write_array(this.public_key, |w, v| w.write_u8(v))?;
        w.write_array(this.key_signature, |w, v| w.write_u8(v))?;
        Ok(())
    }
}
