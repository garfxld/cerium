use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("game_event", 0x22)]
pub struct GameEventPacket {
    pub event: u8,
    pub value: f32,
}

impl Decode for GameEventPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            event: r.read_u8()?,
            value: r.read_f32()?,
        })
    }
}

impl Encode for GameEventPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_u8(this.event)?;
        w.write_f32(this.value)?;
        Ok(())
    }
}
