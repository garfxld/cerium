use cerium_protocol_macros::packet;
use cerium_util::identifier::Identifier;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("custom_payload")]
pub struct ClientPluginMessagePacket {
    pub identifier: Identifier,
    pub data: Vec<u8>,
}

impl ClientPacket for ClientPluginMessagePacket {}

impl Decode for ClientPluginMessagePacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            identifier: r.read_identifier()?,
            data: r.read_array(|r| r.read_u8())?,
        })
    }
}

impl Encode for ClientPluginMessagePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_identifier(this.identifier)?;
        w.write_array(this.data, |w, v| w.write_u8(v))?;
        Ok(())
    }
}
