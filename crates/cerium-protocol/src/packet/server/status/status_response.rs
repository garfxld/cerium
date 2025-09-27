use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("status_response")]
pub struct StatusResponsePacket {
    pub json_response: String,
}

impl Decode for StatusResponsePacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            json_response: r.read_string()?,
        })
    }
}

impl Encode for StatusResponsePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_string(this.json_response)?;
        Ok(())
    }
}
