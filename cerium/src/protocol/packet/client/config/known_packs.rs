use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct KnownPacksPacket {
    pub known_packs: Vec<KnownPacks>,
}

impl Packet for KnownPacksPacket {}
impl ClientPacket for KnownPacksPacket {}

impl Decode for KnownPacksPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            known_packs: r.read_array(KnownPacks::decode)?,
        })
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
