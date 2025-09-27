use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("hello", 0x01)]
pub struct EncryptionRequestPacket {
    pub server_id: String,
    pub public_key: Box<[u8]>,
    pub verify_token: Box<[u8]>,
    pub should_authenticate: bool,
}

impl Decode for EncryptionRequestPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            server_id:           r.read_string()?,
            public_key:          r.read_array(|r| r.read_u8())?.into_boxed_slice(),
            verify_token:        r.read_array(|r| r.read_u8())?.into_boxed_slice(),
            should_authenticate: r.read_bool()?,
        })
    }
}

impl Encode for EncryptionRequestPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_string(this.server_id)?;
        w.write_array(this.public_key.to_vec(), |w, v| w.write_u8(v))?;
        w.write_array(this.verify_token.to_vec(), |w, v| w.write_u8(v))?;
        w.write_bool(this.should_authenticate)?;
        Ok(())
    }
}
