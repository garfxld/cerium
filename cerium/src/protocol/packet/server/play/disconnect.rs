use cerium_protocol_macros::packet;
use simdnbt::{Serialize, ToNbtTag};

use crate::{
    protocol::{
        decode::{Decode, DecodeError},
        encode::{Encode, EncodeError},
        read::PacketRead,
        write::PacketWrite,
    },
    text::Component,
};

#[derive(Debug, Clone)]
#[packet("disconnect", 0x1C)]
pub struct DisconnectPacket {
    pub reason: Component,
}

impl Decode for DisconnectPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self { reason: todo!() })
    }
}

impl Encode for DisconnectPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_nbt_tag(this.reason.to_nbt_tag())?;
        Ok(())
    }
}
