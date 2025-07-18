use crate::protocol::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeException},
};

#[derive(Debug)]
pub struct ServerKnownPacksPacket {
    pub known_packs: Vec<KnownPacks>,
}

impl Encode for ServerKnownPacksPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException> {
        buffer.write_array(this.known_packs, |buffer, value| {
            KnownPacks::encode(buffer, value)
        })?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct KnownPacks {
    namespace: String,
    id: String,
    version: String,
}

impl Encode for KnownPacks {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeException> {
        buffer.write_string(this.namespace)?;
        buffer.write_string(this.id)?;
        buffer.write_string(this.version)?;
        Ok(())
    }
}
