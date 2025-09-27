use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("login_disconnect")]
pub struct LoginDisconnectPacket {
    pub reason: String,
}

impl Decode for LoginDisconnectPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            reason: r.read_string()?,
        })
    }
}

impl Encode for LoginDisconnectPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_string(this.reason)?;
        Ok(())
    }
}
