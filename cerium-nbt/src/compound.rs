use std::io::{Read, Write};

use serde::Serialize;

use crate::{
    END_ID, Error, NbtTag,
    deserialize::{ReadExt, get_nbt_string},
    serialize::WriteExt,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NbtCompound {
    pub children: Vec<(String, NbtTag)>,
}

impl NbtCompound {
    pub fn new() -> NbtCompound {
        NbtCompound {
            children: Vec::new(),
        }
    }

    pub fn insert(&mut self, name: &str, value: impl Into<NbtTag>) {
        let name = name.to_string();
        if !self.children.iter().any(|(key, _)| key == &name) {
            self.children.push((name, value.into()));
        }
    }

    pub fn deserialize_content<R: Read>(reader: &mut R) -> Result<NbtCompound, Error> {
        let mut compound = NbtCompound::new();

        loop {
            let tag_id = match reader.get_u8_be() {
                Ok(id) => id,
                Err(err) => match err {
                    Error::Incomplete(err) => match err.kind() {
                        std::io::ErrorKind::UnexpectedEof => {
                            break;
                        }
                        _ => {
                            return Err(Error::Incomplete(err));
                        }
                    },
                    _ => {
                        return Err(err);
                    }
                },
            };
            if tag_id == END_ID {
                break;
            }

            let name = get_nbt_string(reader)?;
            let tag = NbtTag::deserialize_data(reader, tag_id)?;
            compound.insert(&name, tag);
        }

        Ok(compound)
    }

    pub fn serialize_content<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        for (name, tag) in &self.children {
            w.write_u8_be(tag.id())?;
            NbtTag::String(name.clone()).serialize_data(w)?;
            tag.serialize_data(w)?;
        }
        w.write_u8_be(END_ID)?;
        Ok(())
    }
}

impl Serialize for NbtCompound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.children.len()))?;
        for (key, value) in &self.children {
            map.serialize_entry(key, &value)?;
        }
        map.end()
    }
}
