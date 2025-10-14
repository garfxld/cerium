use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    registry::DynamicRegistry,
    util::Identifier,
};
use cerium_nbt::{Nbt, to_nbt_compound};

#[derive(Debug, Clone)]
pub struct RegistryDataPacket {
    pub registry_id: Identifier,
    pub entries: Vec<RegistryEntry>,
}

impl Packet for RegistryDataPacket {}
impl ServerPacket for RegistryDataPacket {}

impl Encode for RegistryDataPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_identifier(&this.registry_id)?;
        w.write_array(&this.entries, RegistryEntry::encode)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub entry_id: Identifier,
    pub data: Option<Nbt>,
}

impl Encode for RegistryEntry {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_identifier(&this.entry_id)?;
        w.write_option(&this.data, |w, v| w.write_nbt(v))?;
        Ok(())
    }
}

impl<T> From<&DynamicRegistry<T>> for RegistryDataPacket
where
    T: Serialize + DeserializeOwned + Clone,
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
                    data: Some(to_nbt_compound(value).unwrap().into()),
                })
                .collect(),
        }
    }
}
