use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("select_known_packs", 0x07)]
pub struct KnownPacksPacket {
    pub known_packs: Vec<KnownPacks>,
}

impl ClientPacket for KnownPacksPacket {}

impl Decode for KnownPacksPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            known_packs: r.read_array(KnownPacks::decode)?,
        })
    }
}

impl Encode for KnownPacksPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_array(this.known_packs, KnownPacks::encode);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct KnownPacks {
    namespace: String,
    id: String,
    version: String,
}

impl Decode for KnownPacks {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            namespace: r.read_string()?,
            id:        r.read_string()?,
            version:   r.read_string()?,
        })
    }
}

impl Encode for KnownPacks {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_string(this.namespace)?;
        w.write_string(this.id)?;
        w.write_string(this.version)?;
        Ok(())
    }
}
