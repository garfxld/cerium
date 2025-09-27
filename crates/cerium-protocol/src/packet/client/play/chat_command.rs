use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("chat_command", 0x06)]
pub struct ChatCommandPacket {
    pub command: String,
}

impl ClientPacket for ChatCommandPacket {}

impl Decode for ChatCommandPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            command: r.read_string()?,
        })
    }
}

impl Encode for ChatCommandPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_string(this.command)?;
        Ok(())
    }
}
