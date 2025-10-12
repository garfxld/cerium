use std::io::Write;

use serde::{
    Serialize,
    ser::{self, Impossible},
};

use crate::{
    BYTE_ARRAY_ID, BYTE_ID, COMPOUND_ID, DOUBLE_ID, END_ID, Error, FLOAT_ID, INT_ARRAY_ID, INT_ID,
    LIST_ID, LONG_ARRAY_ID, LONG_ID, NbtTag, Result, SHORT_ID, STRING_ID,
};

pub fn to_bytes_unnamed<T: Serialize>(value: &T, w: impl Write) -> Result<()> {
    let mut serializer = Serializer::new(w, None);
    value.serialize(&mut serializer)?;
    Ok(())
}

pub fn to_bytes_named<T: Serialize>(value: &T, name: String, w: impl Write) -> Result<()> {
    let mut serializer = Serializer::new(w, Some(name));
    value.serialize(&mut serializer)?;
    Ok(())
}

macro_rules! write_number_be {
    ($name:ident, $type:ty) => {
        fn $name(&mut self, value: $type) -> Result<()> {
            let buf = value.to_be_bytes();
            self.write_all(&buf).map_err(Error::Incomplete)?;
            Ok(())
        }
    };
}

#[derive(Clone, Debug, PartialEq)]
enum State {
    // In network NBT, the root name is not present.
    Root(Option<String>),
    Named(String),
    // Used by maps to check if key is a `String`.
    MapKey,
    FirstListElement {
        len: i32,
    },
    ListElement,
    CheckedListElement,
    Array {
        name: String,
        array_type: &'static str,
    },
}

#[allow(unused)]
pub trait WriteExt
where
    Self: Write,
{
    write_number_be!(write_u8_be, u8);
    write_number_be!(write_i8_be, i8);
    write_number_be!(write_u16_be, u16);
    write_number_be!(write_i16_be, i16);
    write_number_be!(write_u32_be, u32);
    write_number_be!(write_i32_be, i32);
    write_number_be!(write_u64_be, u64);
    write_number_be!(write_i64_be, i64);
    write_number_be!(write_f32_be, f32);
    write_number_be!(write_f64_be, f64);

    fn write_slice(&mut self, value: &[u8]) -> Result<()> {
        self.write_all(value).map_err(Error::Incomplete)?;
        Ok(())
    }
}

impl<W: Write> WriteExt for W {}

pub struct Serializer<W>
where
    W: Write,
{
    writer: W,
    state: State,
    handled_root: bool,
    expected_list_tag: u8,
}

impl<W> Serializer<W>
where
    W: Write,
{
    pub fn new(writer: W, name: Option<String>) -> Self {
        Self {
            writer,
            state: State::Root(name),
            handled_root: false,
            expected_list_tag: 0,
        }
    }

    fn parse_state(&mut self, tag: u8) -> Result<()> {
        match &mut self.state {
            State::Named(name) | State::Array { name, .. } => {
                self.writer.write_u8_be(tag)?;
                NbtTag::String(name.clone()).serialize_data(&mut self.writer)?;
            }
            State::FirstListElement { len } => {
                self.writer.write_u8_be(tag)?;
                self.writer.write_i32_be(*len)?;
                self.expected_list_tag = tag;
            }
            State::MapKey => {
                if tag != STRING_ID {
                    return Err(Error::SerdeError(format!(
                        "Map key can only be `String`, not {tag}"
                    )));
                }
            }
            State::ListElement => {
                // Rust rules mandate this is all the same type
            }
            State::CheckedListElement => {
                if tag != self.expected_list_tag {
                    return Err(Error::SerdeError(format!(
                        "List values must all be of the same type! Expected {} but found {}!",
                        self.expected_list_tag, tag
                    )));
                }
            }
            State::Root(root_name) => {
                if self.handled_root {
                    return Err(Error::SerdeError(
                        "Invalid state: already handled root component!".to_string(),
                    ));
                } else {
                    if tag != COMPOUND_ID {
                        return Err(Error::SerdeError(format!(
                            "Invalid state: root is not a `Compound`! ({tag})"
                        )));
                    }
                    self.handled_root = true;
                    self.writer.write_u8_be(tag)?;
                    if let Some(root_name) = root_name {
                        NbtTag::String(root_name.clone()).serialize_data(&mut self.writer)?;
                    }
                }
            }
        };
        Ok(())
    }
}

pub(crate) const NBT_ARRAY_TAG: &str = "__nbt_array";
pub(crate) const NBT_INT_ARRAY_TAG: &str = "__nbt_int_array";
pub(crate) const NBT_LONG_ARRAY_TAG: &str = "__nbt_long_array";
pub(crate) const NBT_BYTE_ARRAY_TAG: &str = "__nbt_byte_array";

macro_rules! unsupported {
    ($ty:expr) => {
        return Err(Error::UnsupportedType(format!(
            "{}; NBT only supports signed values",
            stringify!($ty)
        )))
    };
}

impl<W> ser::Serializer for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.serialize_i8(v as i8)?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.parse_state(BYTE_ID)?;
        self.writer.write_i8_be(v)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.parse_state(SHORT_ID)?;
        self.writer.write_i16_be(v)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.parse_state(INT_ID)?;
        self.writer.write_i32_be(v)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.parse_state(LONG_ID)?;
        self.writer.write_i64_be(v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        if let State::Named(_) = &self.state {
            unsupported!("u8");
        }

        self.parse_state(BYTE_ID)?;
        self.writer.write_u8_be(v)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.parse_state(FLOAT_ID)?;
        self.writer.write_f32_be(v)?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.parse_state(DOUBLE_ID)?;
        self.writer.write_f64_be(v)?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.parse_state(STRING_ID)?;

        if self.state == State::MapKey {
            self.state = State::Named(v.to_string());
        } else {
            NbtTag::String(v.to_string()).serialize_data(&mut self.writer)?;
        }
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.parse_state(LIST_ID)?;
        self.writer.write_u8_be(BYTE_ID)?;

        let len = v.len();
        if len > i32::MAX as usize {
            return Err(Error::LargeLength(len));
        }

        self.writer.write_i32_be(len as i32)?;
        self.writer.write_slice(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)?;
        Ok(())
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        if name == NBT_ARRAY_TAG {
            let name = match self.state {
                State::Named(ref name) => name.clone(),
                _ => return Err(Error::SerdeError("Invalid `Serializer` state!".to_string())),
            };

            self.state = State::Array {
                name,
                array_type: variant,
            };
        } else {
            unsupported!("newtype variant");
        }
        value.serialize(self)?;

        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        let Some(len) = len else {
            return Err(Error::SerdeError(
                "The length of the sequence must be known first!".to_string(),
            ));
        };
        if len > i32::MAX as usize {
            return Err(Error::LargeLength(len));
        }

        match &mut self.state {
            State::Array { array_type, .. } => {
                let (id, expected_tag) = match *array_type {
                    NBT_BYTE_ARRAY_TAG => (BYTE_ARRAY_ID, BYTE_ID),
                    NBT_INT_ARRAY_TAG => (INT_ARRAY_ID, INT_ID),
                    NBT_LONG_ARRAY_TAG => (LONG_ARRAY_ID, LONG_ID),
                    _ => {
                        return Err(Error::SerdeError(
                            "Array supports only `byte`, `int`, and `long`".to_string(),
                        ));
                    }
                };

                self.parse_state(id)?;
                self.writer.write_i32_be(len as i32)?;

                // We can mark anything as an NBT array list, so mark as needed to be checked.
                self.expected_list_tag = expected_tag;
                self.state = State::CheckedListElement;
            }
            _ => {
                self.parse_state(LIST_ID)?;
                self.state = State::FirstListElement { len: len as i32 };
                if len == 0 {
                    // If we have no elements, the `FirstListElement` state will never be invoked, so
                    // write the (unknown) list type and length here.
                    self.writer.write_u8_be(END_ID)?;
                    self.writer.write_i32_be(0)?;
                }
            }
        }

        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.parse_state(COMPOUND_ID)?;
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.parse_state(COMPOUND_ID)?;
        Ok(self)
    }

    fn is_human_readable(&self) -> bool {
        false
    }

    // Unsupported Types

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        unsupported!("u16");
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        unsupported!("u32");
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        unsupported!("u64");
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        unsupported!("char");
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        unsupported!("unit struct");
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        unsupported!("newtype struct");
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unsupported!("tuple struct");
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unsupported!("tuple variant");
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unsupported!("struct variant");
    }
}

impl<W> ser::SerializeTuple for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        self.state = State::CheckedListElement;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<W> ser::SerializeSeq for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        self.state = State::ListElement;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<W> ser::SerializeStruct for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        self.state = State::Named(key.to_string());
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.writer.write_u8_be(END_ID)?;
        Ok(())
    }
}

impl<W> ser::SerializeMap for &mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        self.state = State::MapKey;
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.writer.write_u8_be(END_ID)?;
        Ok(())
    }
}
