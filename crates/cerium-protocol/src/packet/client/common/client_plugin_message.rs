use cerium_protocol_macros::packet;
use cerium_util::identifier::Identifier;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("custom_payload")]
pub struct ClientPluginMessagePacket {
    pub identifier: Identifier,
    pub data: Vec<u8>,
}

impl Decode for ClientPluginMessagePacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        let identifier = buffer.read_identifier()?;

        let length = buffer.read_varint()?;

        // todo: fix overflow
        // currently when using fabric the client send a "minecraft:register" packet to the server.
        // for some reason the read length is way too big and therefore crashes the server.
        // current output (no typo): "abric:attachment_sync_v1fabric-screen-handler-api-v1:open_screen"
        let buffer = buffer.split_to(std::cmp::min(length as usize, buffer.remaining()));

        Ok(Self {
            identifier,
            data: buffer.to_vec(),
        })
    }
}
