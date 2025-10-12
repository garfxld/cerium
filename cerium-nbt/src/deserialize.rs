use std::io::Read;

use serde::{
    Deserialize,
    de::{self, DeserializeSeed, IntoDeserializer as _, MapAccess, SeqAccess, Visitor},
};

use crate::{
    BYTE_ARRAY_ID, BYTE_ID, COMPOUND_ID, END_ID, Error, INT_ARRAY_ID, INT_ID, LIST_ID,
    LONG_ARRAY_ID, LONG_ID, NbtTag, Result,
};

#[allow(unused)]
pub fn from_bytes_named<'a, T: Deserialize<'a>>(r: impl Read) -> Result<T> {
    let mut deserializer = Deserializer::new(r, true);
    T::deserialize(&mut deserializer)
}

#[allow(unused)]
pub fn from_bytes_unnamed<'a, T: Deserialize<'a>>(r: impl Read) -> Result<T> {
    let mut deserializer = Deserializer::new(r, false);
    T::deserialize(&mut deserializer)
}

pub struct Deserializer<R: Read> {
    reader: R,
    named: bool,
    tag_to_deserialize_stack: Option<u8>,
    in_list: bool,
}

impl<R: Read> Deserializer<R> {
    pub fn new(input: R, named: bool) -> Self {
        Deserializer {
            reader: input,
            named,
            tag_to_deserialize_stack: None,
            in_list: false,
        }
    }
}

macro_rules! read_number_be {
    ($name:ident, $type:ty) => {
        fn $name(&mut self) -> Result<$type> {
            let mut buf = [0u8; std::mem::size_of::<$type>()];
            self.read_exact(&mut buf).map_err(Error::Incomplete)?;

            Ok(<$type>::from_be_bytes(buf))
        }
    };
}

#[allow(unused)]
pub trait ReadExt
where
    Self: Read,
{
    read_number_be!(get_u8_be, u8);
    read_number_be!(get_i8_be, i8);
    read_number_be!(get_u16_be, u16);
    read_number_be!(get_i16_be, i16);
    read_number_be!(get_u32_be, u32);
    read_number_be!(get_i32_be, i32);
    read_number_be!(get_u64_be, u64);
    read_number_be!(get_i64_be, i64);
    read_number_be!(get_f32_be, f32);
    read_number_be!(get_f64_be, f64);

    fn read_boxed_slice(&mut self, count: usize) -> Result<Box<[u8]>> {
        let mut buf = vec![0u8; count];
        self.read_exact(&mut buf).map_err(Error::Incomplete)?;
        Ok(buf.into())
    }
}

impl<R: Read> ReadExt for R {}

macro_rules! unsupported_type {
    ($ty:ty) => {
        paste::paste! {
            fn [<deserialize_ $ty>]<V>(self, _: V) -> Result<V::Value>
            where
                V: de::Visitor<'de>,
            {
                Err(Error::UnsupportedType(
                    format!("{}; NBT only supports signed values", stringify!($ty)),
                ))
            }
        }
    };
}

impl<'de, R: Read> de::Deserializer<'de> for &mut Deserializer<R> {
    type Error = Error;

    serde::forward_to_deserialize_any! { i8 i16 i32 i64 f32 f64 char str string unit unit_struct seq tuple tuple_struct bytes newtype_struct byte_buf }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let Some(tag_to_deserialize) = self.tag_to_deserialize_stack else {
            return Err(Error::SerdeError(
                "The top level must be a component (e.g. a struct)".to_string(),
            ));
        };

        match tag_to_deserialize {
            END_ID => Err(Error::SerdeError(
                "Trying to deserialize an END tag!".to_string(),
            )),
            LIST_ID | INT_ARRAY_ID | LONG_ARRAY_ID | BYTE_ARRAY_ID => {
                let list_type = match tag_to_deserialize {
                    LIST_ID => self.reader.get_u8_be()?,
                    INT_ARRAY_ID => INT_ID,
                    LONG_ARRAY_ID => LONG_ID,
                    BYTE_ARRAY_ID => BYTE_ID,
                    _ => unreachable!(),
                };

                let remaining_values = self.reader.get_i32_be()?;
                if remaining_values < 0 {
                    return Err(Error::NegativeLength(remaining_values));
                }

                let result = visitor.visit_seq(ListAccess {
                    de: self,
                    list_type,
                    remaining_values: remaining_values as usize,
                })?;
                Ok(result)
            }
            COMPOUND_ID => visitor.visit_map(CompoundAccess { de: self }),
            _ => {
                let result = match NbtTag::deserialize_data(&mut self.reader, tag_to_deserialize)? {
                    NbtTag::Byte(value) => visitor.visit_i8::<Error>(value)?,
                    NbtTag::Short(value) => visitor.visit_i16::<Error>(value)?,
                    NbtTag::Int(value) => visitor.visit_i32::<Error>(value)?,
                    NbtTag::Long(value) => visitor.visit_i64::<Error>(value)?,
                    NbtTag::Float(value) => visitor.visit_f32::<Error>(value)?,
                    NbtTag::Double(value) => visitor.visit_f64::<Error>(value)?,
                    NbtTag::String(value) => visitor.visit_string::<Error>(value)?,
                    _ => unreachable!(),
                };
                Ok(result)
            }
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.tag_to_deserialize_stack.unwrap() == BYTE_ID {
            let value = self.reader.get_u8_be()?;
            if value != 0 {
                return visitor.visit_bool(true);
            }
        }
        visitor.visit_bool(false)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.in_list {
            let value = self.reader.get_u8_be()?;
            visitor.visit_u8::<Error>(value)
        } else {
            Err(Error::UnsupportedType(
                "u8; NBT only supports signed values".to_string(),
            ))
        }
    }

    unsupported_type!(u16);
    unsupported_type!(u32);
    unsupported_type!(u64);

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(tag_id) = self.tag_to_deserialize_stack {
            if tag_id != COMPOUND_ID {
                return Err(Error::SerdeError(format!(
                    "Trying to deserialize a map without a compound ID (id {})",
                    tag_id
                )));
            }
        } else {
            let next_byte = self.reader.get_u8_be()?;
            if next_byte != COMPOUND_ID {
                return Err(Error::MissingRootCompound(next_byte));
            }

            if self.named {
                // Consume struct name, similar to get_nbt_string but without cesu8::from_java_cesu8
                let length = self.reader.get_u16_be()? as usize;
                let _ = self.reader.read_boxed_slice(length)?;
            }
        }

        let value = visitor.visit_map(CompoundAccess { de: &mut self })?;
        Ok(value)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let variant = get_nbt_string(&mut self.reader)?;
        visitor.visit_enum(variant.into_deserializer())
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let str = get_nbt_string(&mut self.reader)?;
        visitor.visit_string(str)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let Some(tag) = self.tag_to_deserialize_stack else {
            return Err(Error::SerdeError("Ignoring nothing!".to_string()));
        };

        // NbtTag::skip_data(&mut self.input, tag)?;
        visitor.visit_unit()
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

pub fn get_nbt_string<R: Read>(bytes: &mut R) -> Result<String> {
    let length = bytes.get_u16_be()? as usize;
    let bytes = bytes.read_boxed_slice(length)?;
    Ok(String::from_utf8(bytes.to_vec()).unwrap())
}

struct CompoundAccess<'a, R: Read> {
    de: &'a mut Deserializer<R>,
}

impl<'de, R: Read> MapAccess<'de> for CompoundAccess<'_, R> {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        let tag = self.de.reader.get_u8_be()?;
        self.de.tag_to_deserialize_stack = Some(tag);

        if tag == END_ID {
            return Ok(None);
        }

        seed.deserialize(MapKey { de: self.de }).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

struct MapKey<'a, R: Read> {
    de: &'a mut Deserializer<R>,
}

impl<'de, R: Read> de::Deserializer<'de> for MapKey<'_, R> {
    type Error = Error;

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit unit_struct seq tuple tuple_struct map
        struct identifier ignored_any bytes enum newtype_struct byte_buf option
    }

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let key = get_nbt_string(&mut self.de.reader)?;
        visitor.visit_string(key)
    }
}

struct ListAccess<'a, R: Read> {
    de: &'a mut Deserializer<R>,
    remaining_values: usize,
    list_type: u8,
}

impl<'de, R: Read> SeqAccess<'de> for ListAccess<'_, R> {
    type Error = Error;

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining_values)
    }

    fn next_element_seed<E: DeserializeSeed<'de>>(&mut self, seed: E) -> Result<Option<E::Value>> {
        if self.remaining_values == 0 {
            return Ok(None);
        }

        self.remaining_values -= 1;
        self.de.tag_to_deserialize_stack = Some(self.list_type);
        self.de.in_list = true;
        let result = seed.deserialize(&mut *self.de).map(Some);
        self.de.in_list = false;

        result
    }
}
