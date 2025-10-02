use cerium_protocol_macros::packet;
use simdnbt::ToNbtTag;

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
#[packet("system_chat", 0x72)]
pub struct SystemChatMessagePacket {
    pub content: Component,
    pub overlay: bool,
}

impl Decode for SystemChatMessagePacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        todo!()
    }
}

impl Encode for SystemChatMessagePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_nbt_tag(this.content.to_nbt_tag())?;
        w.write_bool(this.overlay)?;
        Ok(())
    }
}
