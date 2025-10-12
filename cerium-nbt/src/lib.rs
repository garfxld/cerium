use std::{fmt::Display, ops::Deref};

mod compound;
mod deserialize;
mod serialize;
mod tag;

pub use compound::*;
use serde::{de, ser};
pub use tag::*;
use thiserror::Error;

pub use deserialize::{from_bytes_named, from_bytes_unnamed};
pub use serialize::{to_bytes_named, to_bytes_unnamed};

use crate::serialize::WriteExt;

pub const END_ID: u8 = 0;
pub const BYTE_ID: u8 = 1;
pub const SHORT_ID: u8 = 2;
pub const INT_ID: u8 = 3;
pub const LONG_ID: u8 = 4;
pub const FLOAT_ID: u8 = 5;
pub const DOUBLE_ID: u8 = 6;
pub const BYTE_ARRAY_ID: u8 = 7;
pub const STRING_ID: u8 = 8;
pub const LIST_ID: u8 = 9;
pub const COMPOUND_ID: u8 = 10;
pub const INT_ARRAY_ID: u8 = 11;
pub const LONG_ARRAY_ID: u8 = 12;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("NBT doesn't support this type: {0}")]
    UnsupportedType(String),
    #[error("NBT reading was cut short: {0}")]
    Incomplete(std::io::Error),
    #[error("Serde error: {0}")]
    SerdeError(String),
    #[error("The root tag of the NBT file is not a compound tag. Received tag id: {0}")]
    MissingRootCompound(u8),
    #[error("Negative list length: {0}")]
    NegativeLength(i32),
    #[error("Length too large: {0}")]
    LargeLength(usize),
    #[error("Encountered an unknown NBT tag id: {0}.")]
    UnknownTag(u8),
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::SerdeError(msg.to_string())
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::SerdeError(msg.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Nbt {
    pub name: String,
    pub root: NbtCompound,
}

impl Nbt {
    pub fn new(name: String, compound: NbtCompound) -> Self {
        Nbt {
            name,
            root: compound,
        }
    }

    pub fn write_unnamed<W: WriteExt>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8_be(COMPOUND_ID)?;
        self.root.serialize_content(writer)?;
        Ok(())
    }
}

impl Deref for Nbt {
    type Target = NbtCompound;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl From<NbtCompound> for Nbt {
    fn from(value: NbtCompound) -> Self {
        Nbt::new(String::new(), value)
    }
}

pub trait ToNbt {
    fn to_nbt(self) -> Nbt;
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Test {
        byte: i8,
        short: i16,
        int: i32,
        long: i64,
        float: f32,
        string: String,
    }

    #[test]
    fn test() {
        let test = Test {
            byte: 123,
            short: 1342,
            int: 4313,
            long: 34,
            float: 1.00,
            string: "Hello test".to_string(),
        };

        let mut bytes = Vec::new();
        to_bytes_unnamed(&test, &mut bytes).unwrap();
        let recreated_struct: Test = from_bytes_unnamed(Cursor::new(bytes)).unwrap();

        assert_eq!(test, recreated_struct);
    }
}
