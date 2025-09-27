use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("key", 0x01)]
pub struct EncryptionResponsePacket {
    pub shared_secret: Box<[u8]>,
    pub verify_token: Box<[u8]>,
}

impl ClientPacket for EncryptionResponsePacket {}

impl Decode for EncryptionResponsePacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            shared_secret: r.read_array(|r| r.read_u8())?.into_boxed_slice(),
            verify_token:  r.read_array(|r| r.read_u8())?.into_boxed_slice(),
        })
    }
}

impl Encode for EncryptionResponsePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_array(this.shared_secret.to_vec(), |w, v| w.write_u8(v))?;
        w.write_array(this.verify_token.to_vec(), |w, v| w.write_u8(v))?;
        Ok(())
    }
}
