use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct KnownPacksPacket {
    pub known_packs: Vec<KnownPacks>,
}

impl Packet for KnownPacksPacket {}
impl ServerPacket for KnownPacksPacket {}

impl Encode for KnownPacksPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_array(&this.known_packs, KnownPacks::encode)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct KnownPacks {
    namespace: String,
    id: String,
    version: String,
}

impl Encode for KnownPacks {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_string(&this.namespace)?;
        w.write_string(&this.id)?;
        w.write_string(&this.version)?;
        Ok(())
    }
}
