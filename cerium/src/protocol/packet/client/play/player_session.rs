use uuid::Uuid;

use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PlayerSessionPacket {
    pub session_id: Uuid,
    pub expires_at: i64,
    pub public_key: Vec<u8>,
    pub key_signature: Vec<u8>,
}

impl Packet for PlayerSessionPacket {}
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
