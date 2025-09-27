use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("pong_response", 0x01)]
pub struct PongResponsePacket {
    pub timestamp: i64,
}

impl Decode for PongResponsePacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            timestamp: r.read_i64()?,
        })
    }
}

impl Encode for PongResponsePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_i64(this.timestamp)?;
        Ok(())
    }
}
