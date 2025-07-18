use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeException},
    encode::Encode,
};

#[derive(Debug)]
pub struct ClientKnownPacksPacket {
    pub known_packs: Vec<KnownPacks>,
}

impl Decode for ClientKnownPacksPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {
            known_packs: buffer.read_list(|buffer| KnownPacks::decode(buffer))?,
        })
    }
}

#[derive(Debug)]
pub struct KnownPacks {
    namespace: String,
    id: String,
    version: String,
}

impl Decode for KnownPacks {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeException> {
        Ok(Self {
            namespace: buffer.read_string()?,
            id: buffer.read_string()?,
            version: buffer.read_string()?,
        })
    }
}
