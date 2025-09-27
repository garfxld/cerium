use bytes::{Buf, BufMut};
use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("keep_alive", 0x36)]
pub struct KeepAlivePacket {
    pub keep_alive_id: i64,
}

impl Decode for KeepAlivePacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            keep_alive_id: r.read_i64()?,
        })
    }
}

impl Encode for KeepAlivePacket {
    fn encode<W: PacketWrite>(buffer: &mut W, this: Self) -> Result<(), EncodeError> {
        buffer.write_i64(this.keep_alive_id)?;
        Ok(())
    }
}
