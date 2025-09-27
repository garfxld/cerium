use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("container_close", 0x12)]
pub struct CloseContainerPacket {
    pub window_id: i32,
}

impl ClientPacket for CloseContainerPacket {}

impl Decode for CloseContainerPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(CloseContainerPacket {
            window_id: r.read_varint()?,
        })
    }
}

impl Encode for CloseContainerPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_varint(this.window_id)?;
        Ok(())
    }
}
