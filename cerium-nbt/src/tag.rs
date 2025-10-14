use std::io::Write;

use bytes::Buf;
use serde::{
    Serialize,
    ser::{self, SerializeSeq},
};

use crate::{
    BYTE_ARRAY_ID, BYTE_ID, COMPOUND_ID, DOUBLE_ID, END_ID, Error, FLOAT_ID, INT_ARRAY_ID, INT_ID,
    LIST_ID, LONG_ARRAY_ID, LONG_ID, NbtCompound, SHORT_ID, STRING_ID,
    deserialize::{ReadExt, get_nbt_string},
    serialize::WriteExt,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum NbtTag {
    End = END_ID,
    Byte(i8) = BYTE_ID,
    Short(i16) = SHORT_ID,
    Int(i32) = INT_ID,
    Long(i64) = LONG_ID,
    Float(f32) = FLOAT_ID,
    Double(f64) = DOUBLE_ID,
    ByteArray(Box<[u8]>) = BYTE_ARRAY_ID,
    String(String) = STRING_ID,
    List(Vec<NbtTag>) = LIST_ID,
    Compound(NbtCompound) = COMPOUND_ID,
    IntArray(Vec<i32>) = INT_ARRAY_ID,
    LongArray(Vec<i64>) = LONG_ARRAY_ID,
}

impl NbtTag {
    pub const fn id(&self) -> u8 {
        // Safety: Since Self is repr(u8), it is guaranteed to hold the discriminant in the first byte
        // See https://doc.rust-lang.org/reference/items/enumerations.html#pointer-casting
        unsafe { *(self as *const Self as *const u8) }
    }

    pub fn deserialize_data<R: Buf>(reader: &mut R, tag_id: u8) -> Result<NbtTag, Error> {
        match tag_id {
            END_ID => Ok(NbtTag::End),
            BYTE_ID => {
                let byte = reader.try_get_i8().unwrap();
                Ok(NbtTag::Byte(byte))
            }
            SHORT_ID => {
                let short = reader.try_get_i16().unwrap();
                Ok(NbtTag::Short(short))
            }
            INT_ID => {
                let int = reader.try_get_i32().unwrap();
                Ok(NbtTag::Int(int))
            }
            LONG_ID => {
                let long = reader.try_get_i64().unwrap();
                Ok(NbtTag::Long(long))
            }
            FLOAT_ID => {
                let float = reader.try_get_f32().unwrap();
                Ok(NbtTag::Float(float))
            }
            DOUBLE_ID => {
                let double = reader.try_get_f64().unwrap();
                Ok(NbtTag::Double(double))
            }
            BYTE_ARRAY_ID => {
                let len = reader.try_get_i32().unwrap();
                if len < 0 {
                    return Err(Error::NegativeLength(len));
                }

                let byte_array = reader.read_boxed_slice(len as usize)?;
                Ok(NbtTag::ByteArray(byte_array))
            }
            STRING_ID => Ok(NbtTag::String(get_nbt_string(reader)?)),
            LIST_ID => {
                let tag_type_id = reader.try_get_u8().unwrap();
                let len = reader.try_get_i32().unwrap();
                if len < 0 {
                    return Err(Error::NegativeLength(len));
                }

                let mut list = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    let tag = NbtTag::deserialize_data(reader, tag_type_id)?;
                    assert_eq!(tag.id(), tag_type_id);
                    list.push(tag);
                }
                Ok(NbtTag::List(list))
            }
            COMPOUND_ID => Ok(NbtTag::Compound(NbtCompound::deserialize_content(reader)?)),
            INT_ARRAY_ID => {
                let len = reader.try_get_i32().unwrap();
                if len < 0 {
                    return Err(Error::NegativeLength(len));
                }

                let len = len as usize;
                let mut int_array = Vec::with_capacity(len);
                for _ in 0..len {
                    let int = reader.try_get_i32().unwrap();
                    int_array.push(int);
                }
                Ok(NbtTag::IntArray(int_array))
            }
            LONG_ARRAY_ID => {
                let len = reader.try_get_i32().unwrap();
                if len < 0 {
                    return Err(Error::NegativeLength(len));
                }

                let len = len as usize;
                let mut long_array = Vec::with_capacity(len);
                for _ in 0..len {
                    let long = reader.try_get_i64().unwrap();
                    long_array.push(long);
                }
                Ok(NbtTag::LongArray(long_array))
            }
            _ => Err(Error::UnknownTag(tag_id)),
        }
    }

    pub fn write<W: WriteExt>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_u8_be(self.id())?;
        self.serialize_data(writer)?;
        Ok(())
    }

    pub fn serialize_data<W: Write>(&self, w: &mut W) -> Result<(), Error> {
        match self {
            NbtTag::End => (),
            NbtTag::Byte(byte) => w.write_i8_be(*byte)?,
            NbtTag::Short(short) => w.write_i16_be(*short)?,
            NbtTag::Int(int) => w.write_i32_be(*int)?,
            NbtTag::Long(long) => w.write_i64_be(*long)?,
            NbtTag::Float(float) => w.write_f32_be(*float)?,
            NbtTag::Double(double) => w.write_f64_be(*double)?,
            NbtTag::ByteArray(byte_array) => {
                let len = byte_array.len();
                if len > i32::MAX as usize {
                    return Err(Error::LargeLength(len));
                }

                w.write_i32_be(len as i32)?;
                w.write_slice(byte_array)?;
            }
            NbtTag::String(string) => {
                let java_string = string.as_bytes();
                let len = java_string.len();
                if len > u16::MAX as usize {
                    return Err(Error::LargeLength(len));
                }

                w.write_u16_be(len as u16)?;
                w.write_slice(&java_string)?;
            }
            NbtTag::List(list) => {
                let len = list.len();
                if len > i32::MAX as usize {
                    return Err(Error::LargeLength(len));
                }

                w.write_u8_be(list.first().unwrap_or(&NbtTag::End).id())?;
                w.write_i32_be(len as i32)?;
                for nbt_tag in list {
                    nbt_tag.serialize_data(w)?;
                }
            }
            NbtTag::Compound(compound) => {
                compound.serialize_content(w)?;
            }
            NbtTag::IntArray(int_array) => {
                let len = int_array.len();
                if len > i32::MAX as usize {
                    return Err(Error::LargeLength(len));
                }

                w.write_i32_be(len as i32)?;
                for int in int_array {
                    w.write_i32_be(*int)?;
                }
            }
            NbtTag::LongArray(long_array) => {
                let len = long_array.len();
                if len > i32::MAX as usize {
                    return Err(Error::LargeLength(len));
                }

                w.write_i32_be(len as i32)?;

                for long in long_array {
                    w.write_i64_be(*long)?;
                }
            }
        };
        Ok(())
    }
}

impl Serialize for NbtTag {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            NbtTag::End => serializer.serialize_unit(),
            NbtTag::Byte(v) => serializer.serialize_i8(*v),
            NbtTag::Short(v) => serializer.serialize_i16(*v),
            NbtTag::Int(v) => serializer.serialize_i32(*v),
            NbtTag::Long(v) => serializer.serialize_i64(*v),
            NbtTag::Float(v) => serializer.serialize_f32(*v),
            NbtTag::Double(v) => serializer.serialize_f64(*v),
            NbtTag::ByteArray(v) => {
                use serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for byte in v.iter() {
                    seq.serialize_element(byte)?;
                }
                seq.end()
            }
            NbtTag::String(v) => serializer.serialize_str(v),
            NbtTag::List(v) => {
                use serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for item in v.iter() {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            NbtTag::Compound(v) => v.serialize(serializer),
            NbtTag::IntArray(v) => {
                use serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for int in v.iter() {
                    seq.serialize_element(int)?;
                }
                seq.end()
            }
            NbtTag::LongArray(v) => {
                use serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for long in v.iter() {
                    seq.serialize_element(long)?;
                }
                seq.end()
            }
        }
    }
}

impl From<i8> for NbtTag {
    fn from(value: i8) -> Self {
        NbtTag::Byte(value)
    }
}

impl From<bool> for NbtTag {
    fn from(value: bool) -> Self {
        NbtTag::Byte(value as i8)
    }
}

impl From<i16> for NbtTag {
    fn from(value: i16) -> Self {
        NbtTag::Short(value)
    }
}

impl From<i32> for NbtTag {
    fn from(value: i32) -> Self {
        NbtTag::Int(value)
    }
}

impl From<i64> for NbtTag {
    fn from(value: i64) -> Self {
        NbtTag::Long(value)
    }
}

impl From<f32> for NbtTag {
    fn from(value: f32) -> Self {
        NbtTag::Float(value)
    }
}

impl From<f64> for NbtTag {
    fn from(value: f64) -> Self {
        NbtTag::Double(value)
    }
}

impl From<&[u8]> for NbtTag {
    fn from(value: &[u8]) -> Self {
        let mut cloned = Vec::with_capacity(value.len());
        cloned.copy_from_slice(value);
        NbtTag::ByteArray(cloned.into_boxed_slice())
    }
}

impl From<&str> for NbtTag {
    fn from(value: &str) -> Self {
        NbtTag::String(value.to_string())
    }
}

impl From<String> for NbtTag {
    fn from(value: String) -> Self {
        NbtTag::String(value.into())
    }
}

impl From<NbtCompound> for NbtTag {
    fn from(value: NbtCompound) -> Self {
        NbtTag::Compound(value)
    }
}

impl From<Vec<i32>> for NbtTag {
    fn from(value: Vec<i32>) -> Self {
        NbtTag::IntArray(value)
    }
}

impl From<Vec<i64>> for NbtTag {
    fn from(value: Vec<i64>) -> Self {
        NbtTag::LongArray(value)
    }
}

pub fn to_nbt_compound<T: Serialize>(value: &T) -> Result<NbtCompound, Error> {
    match value.serialize(Serializer)? {
        NbtTag::Compound(compound) => Ok(compound),
        _ => Err(Error::SerdeError("Expected a compound tag".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestStruct {
        name: String,
        age: i32,
        scores: Vec<f32>,
    }

    #[test]
    fn test_serialization() {
        let test = TestStruct {
            name: "Alice".to_string(),
            age: 30,
            scores: vec![95.5, 87.3, 92.1],
        };

        let nbt = to_nbt_compound(&test).unwrap();
        println!("{:?}", nbt);
    }
}

pub struct Serializer;

impl ser::Serializer for Serializer {
    type Ok = NbtTag;
    type Error = Error;
    type SerializeSeq = SeqSerializer;
    type SerializeTuple = SeqSerializer;
    type SerializeTupleStruct = SeqSerializer;
    type SerializeTupleVariant = TupleVariantSerializer;
    type SerializeMap = MapSerializer;
    type SerializeStruct = MapSerializer;
    type SerializeStructVariant = StructVariantSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Byte(if v { 1 } else { 0 }))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Byte(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Short(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Int(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Long(v))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        unimplemented!("only signed variants supported");
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        unimplemented!("only signed variants supported");
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        unimplemented!("only signed variants supported");
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        unimplemented!("only signed variants supported");
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Float(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Double(v))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!("unsupported")
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::String(v.to_string()))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
        //Ok(NbtTag::ByteArray(v.iter().map(|&b| b as i8).collect()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Compound(NbtCompound { children: vec![] }))
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Compound(NbtCompound { children: vec![] }))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::String(variant.to_string()))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        let mut compound = NbtCompound { children: vec![] };
        compound
            .children
            .push((variant.to_string(), value.serialize(Serializer)?));
        Ok(NbtTag::Compound(compound))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqSerializer { items: vec![] })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(TupleVariantSerializer {
            variant: variant.to_string(),
            items: vec![],
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer {
            children: vec![],
            current_key: None,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(StructVariantSerializer {
            variant: variant.to_string(),
            children: vec![],
            // current_key: None,
        })
    }
}

// Sequence serializer
pub struct SeqSerializer {
    items: Vec<NbtTag>,
}

impl ser::SerializeSeq for SeqSerializer {
    type Ok = NbtTag;
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.items.push(value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::List(self.items))
    }
}

impl ser::SerializeTuple for SeqSerializer {
    type Ok = NbtTag;
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for SeqSerializer {
    type Ok = NbtTag;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

// Tuple variant serializer
pub struct TupleVariantSerializer {
    variant: String,
    items: Vec<NbtTag>,
}

impl ser::SerializeTupleVariant for TupleVariantSerializer {
    type Ok = NbtTag;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.items.push(value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut compound = NbtCompound { children: vec![] };
        compound
            .children
            .push((self.variant, NbtTag::List(self.items)));
        Ok(NbtTag::Compound(compound))
    }
}

// Map serializer
pub struct MapSerializer {
    children: Vec<(String, NbtTag)>,
    current_key: Option<String>,
}

impl ser::SerializeMap for MapSerializer {
    type Ok = NbtTag;
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        let key_tag = key.serialize(Serializer)?;
        self.current_key = Some(match key_tag {
            NbtTag::String(s) => s,
            _ => return Err(Error::SerdeError("Map keys must be strings".to_string())),
        });
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let key = self.current_key.take().ok_or_else(|| {
            Error::SerdeError("serialize_value called before serialize_key".to_string())
        })?;
        self.children.push((key, value.serialize(Serializer)?));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Compound(NbtCompound {
            children: self.children,
        }))
    }
}

impl ser::SerializeStruct for MapSerializer {
    type Ok = NbtTag;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.children
            .push((key.to_string(), value.serialize(Serializer)?));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(NbtTag::Compound(NbtCompound {
            children: self.children,
        }))
    }
}

// Struct variant serializer
pub struct StructVariantSerializer {
    variant: String,
    children: Vec<(String, NbtTag)>,
    // current_key: Option<String>,
}

impl ser::SerializeStructVariant for StructVariantSerializer {
    type Ok = NbtTag;
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.children
            .push((key.to_string(), value.serialize(Serializer)?));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let inner_compound = NbtCompound {
            children: self.children,
        };
        let mut outer_compound = NbtCompound { children: vec![] };
        outer_compound
            .children
            .push((self.variant, NbtTag::Compound(inner_compound)));
        Ok(NbtTag::Compound(outer_compound))
    }
}
