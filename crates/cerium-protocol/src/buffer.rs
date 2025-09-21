use bytes::{Buf, BufMut as _, BytesMut, buf::Take};
use uuid::Uuid;

use crate::{decode::DecodeError, encode::EncodeError};
use cerium_util::identifier::Identifier;

#[derive(Clone)]
pub struct ByteBuffer {
    inner: bytes::BytesMut,
}

impl From<BytesMut> for ByteBuffer {
    fn from(value: BytesMut) -> Self {
        Self { inner: value }
    }
}

impl Into<BytesMut> for ByteBuffer {
    fn into(self) -> BytesMut {
        self.inner
    }
}

macro_rules! read_impl {
    ($type:ty) => {
        paste::paste! {
            pub fn [<read_ $type>](&mut self) -> Result<$type, DecodeError> {
                Ok(self.inner.[<get_ $type>]())
            }
        }
    };
}

macro_rules! write_impl {
    ($type:ty) => {
        paste::paste! {
            pub fn [<write_ $type>](&mut self, value: $type) -> Result<(), EncodeError> {
                Ok(self.inner.[<put_ $type>](value))
            }
        }
    };
}

#[allow(unused)]
impl ByteBuffer {
    pub fn new() -> Self {
        Self {
            inner: bytes::BytesMut::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: bytes::BytesMut::with_capacity(capacity),
        }
    }

    //
    // READER
    //

    read_impl!(u8);
    read_impl!(i8);
    read_impl!(u16);
    read_impl!(i16);
    read_impl!(u32);
    read_impl!(i32);
    read_impl!(u64);
    read_impl!(i64);
    read_impl!(u128);
    read_impl!(i128);

    read_impl!(f32);
    read_impl!(f64);

    pub fn read_bool(&mut self) -> Result<bool, DecodeError> {
        Ok(self.read_u8()? == 1)
    }

    pub fn read_option<T, F>(&mut self, f: F) -> Result<Option<T>, DecodeError>
    where
        F: Fn(&mut Self) -> Result<T, DecodeError>,
    {
        let value = if self.read_bool()? {
            Some(f(self)?)
        } else {
            None
        };
        Ok(value)
    }

    pub fn read_string(&mut self) -> Result<String, DecodeError> {
        let length = self.read_varint()? as usize;
        let bytes = self.split_to(length);

        String::from_utf8(bytes.to_vec()).map_err(|e| DecodeError::Decode(e.to_string()))
    }

    pub fn read_uuid(&mut self) -> Result<Uuid, DecodeError> {
        Ok(Uuid::from_u128(self.read_u128()?))
    }

    pub fn read_varint(&mut self) -> Result<i32, DecodeError> {
        let mut value = 0;
        for i in 0..5 {
            let byte = self.read_u8()?;
            value |= (i32::from(byte) & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok(value);
            }
        }
        return Err(DecodeError::Decode("VarInt too large".to_string())); // VarInt is too large.
    }

    pub fn read_list<T, F>(&mut self, mut read: F) -> Result<Vec<T>, DecodeError>
    where
        F: FnMut(&mut Self) -> Result<T, DecodeError>,
    {
        let length = self.read_varint()? as usize;
        let mut list = Vec::with_capacity(length);
        for _ in 0..length {
            list.push(read(self)?);
        }
        Ok(list)
    }

    pub fn read_identifier(&mut self) -> Result<Identifier, DecodeError> {
        let identifier = self.read_string()?;

        match identifier.split_once(":") {
            Some((namespace, path)) => Ok(Identifier::new(namespace, path)),
            None => Err(DecodeError::Decode("Identifier read".to_string())),
        }
    }

    //
    // WRITER
    //

    write_impl!(u8);
    write_impl!(i8);
    write_impl!(u16);
    write_impl!(i16);
    write_impl!(u32);
    write_impl!(i32);
    write_impl!(u64);
    write_impl!(i64);
    write_impl!(u128);
    write_impl!(i128);

    write_impl!(f32);
    write_impl!(f64);

    pub fn write_bool(&mut self, value: bool) -> Result<(), EncodeError> {
        self.write_u8(value as u8)?;
        Ok(())
    }

    pub fn write_varint(&mut self, value: i32) -> Result<(), EncodeError> {
        let x = value as u64;
        let stage1 = (x & 0x000000000000007f)
            | ((x & 0x0000000000003f80) << 1)
            | ((x & 0x00000000001fc000) << 2)
            | ((x & 0x000000000fe00000) << 3)
            | ((x & 0x00000000f0000000) << 4);

        let leading = stage1.leading_zeros();

        let unused_bytes = (leading - 1) >> 3;
        let bytes_needed = 8 - unused_bytes;

        // set all but the last MSBs
        let msbs = 0x8080808080808080;
        let msbmask = 0xffffffffffffffff >> (((8 - bytes_needed + 1) << 3) - 1);

        let merged = stage1 | (msbs & msbmask);
        let bytes = merged.to_le_bytes();

        self.put(unsafe { bytes.get_unchecked(..bytes_needed as usize) });
        Ok(())
    }

    pub fn write_varint3(&mut self, value: i32) -> Result<(), EncodeError> {
        self.write_u8((value & 0x7F | 0x80) as u8)?;
        self.write_u8((((value >> 7) & 0x7F) | 0x80) as u8)?;
        self.write_u8((value >> 14) as u8)?;
        Ok(())
    }

    pub fn write_string(&mut self, value: String) -> Result<(), EncodeError> {
        self.write_varint(value.len() as i32)?;
        self.put(value.as_bytes());
        Ok(())
    }

    pub fn write_identifier(&mut self, value: Identifier) -> Result<(), EncodeError> {
        self.write_string(value.to_string())?;
        Ok(())
    }

    pub fn write_uuid(&mut self, value: Uuid) -> Result<(), EncodeError> {
        self.write_u128(value.as_u128())?;
        Ok(())
    }

    pub fn write_optional<T, F>(
        &mut self,
        value: Option<T>,
        mut write: F,
    ) -> Result<(), EncodeError>
    where
        F: FnMut(&mut Self, T) -> Result<(), EncodeError>,
    {
        self.write_bool(value.is_some())?;
        if let Some(value) = value {
            write(self, value)?;
        }
        Ok(())
    }

    pub fn write_byte_array(&mut self, value: &Vec<u8>) -> Result<(), EncodeError> {
        self.write_varint(value.len() as i32);
        self.inner.put_slice(value.as_slice());
        Ok(())
    }

    pub fn write_array<T, F>(&mut self, value: Vec<T>, mut write: F) -> Result<(), EncodeError>
    where
        F: FnMut(&mut Self, T) -> Result<(), EncodeError>,
    {
        self.write_varint(value.len() as i32)?;
        for element in value {
            write(self, element)?;
        }
        Ok(())
    }

    pub fn write_slice(&mut self, slice: &[u8]) -> Result<(), EncodeError> {
        self.inner.put_slice(slice);
        Ok(())
    }

    pub fn write_unprefixed_array<T, F>(
        &mut self,
        value: Vec<T>,
        mut write: F,
    ) -> Result<(), EncodeError>
    where
        F: FnMut(&mut Self, T) -> Result<(), EncodeError>,
    {
        for element in value {
            write(self, element)?;
        }
        Ok(())
    }

    pub fn freeze(&mut self) {
        self.inner = self.inner.clone();
    }

    pub fn put<T: Buf>(&mut self, src: T) {
        self.inner.put(src);
    }

    pub fn split_to(&mut self, at: usize) -> Self {
        Self {
            inner: self.inner.split_to(at),
        }
    }

    pub fn split_off(&mut self, at: usize) -> Self {
        Self {
            inner: self.inner.split_off(at),
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.inner.to_vec()
    }

    pub fn remaining(&self) -> usize {
        self.inner.remaining()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn advance(&mut self, cnt: usize) {
        self.inner.advance(cnt);
    }

    pub fn set_position(&mut self, position: usize) {
        self.inner = self.inner.clone();
        self.inner.advance(position);
    }

    pub fn set_len(&mut self, length: usize) {
        unsafe {
            self.inner.set_len(length);
        }
    }

    pub fn take(&mut self, limit: usize) -> Take<BytesMut> {
        self.inner.clone().take(limit)
    }
}
