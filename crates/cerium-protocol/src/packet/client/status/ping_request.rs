use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("ping_request")]
pub struct PingRequestPacket {
    pub timestamp: i64,
}

impl ClientPacket for PingRequestPacket {}

impl Decode for PingRequestPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            timestamp: r.read_i64()?,
        })
    }
}

impl Encode for PingRequestPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_i64(this.timestamp)?;
        Ok(())
    }
}
