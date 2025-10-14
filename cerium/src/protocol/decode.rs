use bytes::Buf;
use thiserror::Error;
use uuid::Uuid;

use crate::{text::Component, util::Identifier};
use cerium_nbt::Nbt;

pub trait Decode
where
    Self: Sized,
{
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self>;
}

#[derive(Error, Debug, Clone)]
pub enum DecodeError {
    #[error("{0}")]
    Decode(String),
    #[error("Not enough bytes to read. (Requested: {1}, Available: {0})")]
    NotEnoughBytes(usize, usize),
    #[error("Unknown Packet: {0}")]
    UnkownPacket(i32),
}

type Result<T> = core::result::Result<T, DecodeError>;

pub trait PacketRead {
    fn read_u8(&mut self) -> Result<u8>;

    fn read_i8(&mut self) -> Result<i8>;

    fn read_u16(&mut self) -> Result<u16>;

    fn read_i16(&mut self) -> Result<i16>;

    fn read_u32(&mut self) -> Result<u32>;

    fn read_i32(&mut self) -> Result<i32>;

    fn read_u64(&mut self) -> Result<u64>;

    fn read_i64(&mut self) -> Result<i64>;

    fn read_u128(&mut self) -> Result<u128>;

    fn read_i128(&mut self) -> Result<i128>;

    fn read_f32(&mut self) -> Result<f32>;

    fn read_f64(&mut self) -> Result<f64>;

    fn read_bool(&mut self) -> Result<bool>;

    fn read_string(&mut self) -> Result<String>;

    fn read_string_limited<const MAX: usize>(&mut self) -> Result<String>;

    fn read_varint(&mut self) -> Result<i32>;

    fn read_uuid(&mut self) -> Result<Uuid>;

    fn read_identifier(&mut self) -> Result<Identifier>;

    fn read_nbt(&mut self) -> Result<Nbt>;

    fn read_option<T, F>(&mut self, f: F) -> Result<Option<T>>
    where
        F: Fn(&mut Self) -> Result<T>;

    fn read_array<T, F>(&mut self, f: F) -> Result<Vec<T>>
    where
        F: FnMut(&mut Self) -> Result<T>;

    fn read_bytes(&mut self, max: i32) -> Result<Vec<u8>>;

    fn read_boxed_slice(&mut self) -> Result<Box<[u8]>>;

    fn read_component(&mut self) -> Result<Component>;
}

macro_rules! read_impl {
    ($ty:ty) => {
        paste::paste! {
            fn [<read_ $ty>](&mut self) -> Result<$ty> {
                self.[<try_get_ $ty>]()
                    .map_err(|e| DecodeError::NotEnoughBytes(e.available, e.requested))
            }
        }
    };
}

impl<R: Buf> PacketRead for R {
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

    fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_u8()? == 1)
    }

    fn read_string(&mut self) -> Result<String> {
        self.read_string_limited::<{ usize::MAX }>()
    }

    fn read_string_limited<const MAX: usize>(&mut self) -> Result<String> {
        let length = self.read_varint()? as usize;

        let mut buf = vec![0u8; length];
        self.copy_to_slice(&mut buf);

        String::from_utf8(buf).map_err(|e| DecodeError::Decode(e.to_string()))
    }

    fn read_varint(&mut self) -> Result<i32> {
        let mut value = 0;
        for i in 0..5 {
            let byte = self.read_u8()?;
            value |= (i32::from(byte) & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok(value);
            }
        }
        return Err(DecodeError::Decode("VarInt too large".to_string()));
    }

    fn read_uuid(&mut self) -> Result<Uuid> {
        Ok(Uuid::from_u128(self.read_u128()?))
    }

    fn read_identifier(&mut self) -> Result<Identifier> {
        let identifier = self.read_string()?;

        match identifier.split_once(":") {
            Some((namespace, path)) => Ok(Identifier::new(namespace, path)),
            None => Err(DecodeError::Decode("Identifier read".to_string())),
        }
    }

    fn read_nbt(&mut self) -> Result<Nbt> {
        todo!()
    }

    fn read_option<T, F>(&mut self, f: F) -> Result<Option<T>>
    where
        F: Fn(&mut Self) -> Result<T>,
    {
        let value = if self.read_bool()? {
            Some(f(self)?)
        } else {
            None
        };
        Ok(value)
    }

    fn read_array<T, F>(&mut self, mut f: F) -> Result<Vec<T>>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        let length = self.read_varint()? as usize;
        let mut list = Vec::with_capacity(length);
        for _ in 0..length {
            list.push(f(self)?);
        }
        Ok(list)
    }

    fn read_bytes(&mut self, max: i32) -> Result<Vec<u8>> {
        let mut len = self.remaining();
        if max != -1 {
            len = std::cmp::min(len, max as usize);
        }

        let mut buf = vec![0u8; len as usize];
        self.copy_to_slice(&mut buf);
        Ok(buf)
    }

    fn read_boxed_slice(&mut self) -> Result<Box<[u8]>> {
        let length = self.read_varint()?;

        let mut buf = vec![0u8; length as usize];
        self.copy_to_slice(&mut buf);
        Ok(buf.into())
    }

    fn read_component(&mut self) -> Result<Component> {
        cerium_nbt::from_bytes_unnamed(self).map_err(|_| DecodeError::Decode("f".to_string()))
    }
}
