use std::{collections::HashMap, fmt::Debug};

use cerium_protocol_macros::packet;
use cerium_registry::DynamicRegistry;
use cerium_util::identifier::Identifier;
use simdnbt::owned::Nbt;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("registry_data", 0x07)]
pub struct RegistryDataPacket {
    pub registry_id: Identifier,
    pub entries: Vec<RegistryEntry>,
}

impl Decode for RegistryDataPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            registry_id: r.read_identifier()?,
            entries:     r.read_array(|r| RegistryEntry::decode(r))?,
        })
    }
}

impl Encode for RegistryDataPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_identifier(this.registry_id)?;
        w.write_array(this.entries, RegistryEntry::encode)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub entry_id: Identifier,
    pub data: Option<Nbt>,
}

impl Decode for RegistryEntry {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            entry_id: r.read_identifier()?,
            data:     r.read_option(|r| r.read_nbt())?,
        })
    }
}

impl Encode for RegistryEntry {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_identifier(this.entry_id)?;
        w.write_option(this.data, |w, v| w.write_nbt(v))?;
        Ok(())
    }
}

impl<T> From<&DynamicRegistry<T>> for RegistryDataPacket
where
    T: serde::de::DeserializeOwned + simdnbt::Serialize + Clone,
{
    fn from(value: &DynamicRegistry<T>) -> Self {
        let registry_id = value.registry_id().clone();
        let entries = value.entries();

        RegistryDataPacket {
            registry_id,
            entries: entries
                .into_iter()
                .map(|(key, value)| RegistryEntry {
                    entry_id: key.as_key().clone(),
                    data: Some(Nbt::Some(value.clone().to_nbt())),
                })
                .collect(),
        }
    }
}
