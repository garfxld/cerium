use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("intention", 0x00)]
pub struct HandshakePacket {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub intent: i32,
}

impl ClientPacket for HandshakePacket {}

impl Decode for HandshakePacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            protocol_version: r.read_varint()?,
            server_address:   r.read_string()?,
            server_port:      r.read_u16()?,
            intent:           r.read_varint()?,
        })
    }
}

impl Encode for HandshakePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_varint(this.protocol_version)?;
        w.write_string(this.server_address)?;
        w.write_u16(this.server_port)?;
        w.write_varint(this.intent)?;
        Ok(())
    }
}
