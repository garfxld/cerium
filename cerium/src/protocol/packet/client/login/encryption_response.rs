use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct EncryptionResponsePacket {
    pub shared_secret: Box<[u8]>,
    pub verify_token: Box<[u8]>,
}

impl Packet for EncryptionResponsePacket {}
impl ClientPacket for EncryptionResponsePacket {}

impl Decode for EncryptionResponsePacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            shared_secret: r.read_boxed_slice()?,
            verify_token:  r.read_boxed_slice()?,
        })
    }
}
