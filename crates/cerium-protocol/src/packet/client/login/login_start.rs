use cerium_protocol_macros::packet;
use uuid::Uuid;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("hello")]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: Uuid,
}

impl ClientPacket for LoginStartPacket {}

impl Decode for LoginStartPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            name: r.read_string()?,
            uuid: r.read_uuid()?,
        })
    }
}

impl Encode for LoginStartPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_string(this.name)?;
        w.write_uuid(this.uuid)?;
        Ok(())
    }
}
