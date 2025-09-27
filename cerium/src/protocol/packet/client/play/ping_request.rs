use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("ping_request", 0x25)]
pub struct PingRequestPacket {
    pub payload: i64,
}

impl ClientPacket for PingRequestPacket {}

impl Decode for PingRequestPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            payload: r.read_i64()?,
        })
    }
}

impl Encode for PingRequestPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_i64(this.payload)?;
        Ok(())
    }
}
