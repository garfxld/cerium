use std::{collections::HashMap, fmt::Debug};

use cerium_protocol_macros::packet;
use cerium_registry::DynamicRegistry;
use cerium_util::identifier::Identifier;
use simdnbt::owned::Nbt;

use crate::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug, Clone)]
#[packet("registry_data")]
pub struct RegistryDataPacket {
    pub registry_id: Identifier,
    pub entries: Vec<RegistryEntry>,
}

impl Encode for RegistryDataPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_identifier(this.registry_id)?;
        buffer.write_array(this.entries, |buffer, value| {
            RegistryEntry::encode(buffer, value)
        })?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub entry_id: Identifier,
    pub data: Option<Nbt>,
}

impl Encode for RegistryEntry {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_identifier(this.entry_id)?;
        buffer.write_optional(this.data, |buffer, value| {
            let mut data: Vec<u8> = Vec::new();
            value.write_unnamed(&mut data);
            buffer.put(&*data);
            Ok(())
        })?;

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
