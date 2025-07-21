use bytes::BufMut;
use macros::packet;
use simdnbt::owned::Nbt;

use crate::{
    identifier::Identifier,
    protocol::{
        buffer::ByteBuffer,
        encode::{Encode, EncodeError},
    },
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
