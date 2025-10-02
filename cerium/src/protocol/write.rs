use crate::util::Identifier;
use bytes::{BufMut, BytesMut};
use simdnbt::owned::{Nbt, NbtTag};
use uuid::Uuid;

use crate::protocol::encode::EncodeError;

type Result<T> = core::result::Result<T, EncodeError>;

pub trait PacketWrite {
    fn write_u8(&mut self, value: u8) -> Result<()>;

    fn write_i8(&mut self, value: i8) -> Result<()>;

    fn write_u16(&mut self, value: u16) -> Result<()>;

    fn write_i16(&mut self, value: i16) -> Result<()>;

    fn write_u32(&mut self, value: u32) -> Result<()>;

    fn write_i32(&mut self, value: i32) -> Result<()>;

    fn write_u64(&mut self, value: u64) -> Result<()>;

    fn write_i64(&mut self, value: i64) -> Result<()>;

    fn write_u128(&mut self, value: u128) -> Result<()>;

    fn write_i128(&mut self, value: i128) -> Result<()>;

    fn write_f32(&mut self, value: f32) -> Result<()>;

    fn write_f64(&mut self, value: f64) -> Result<()>;

    fn write_bool(&mut self, value: bool) -> Result<()>;

    fn write_varint(&mut self, value: i32) -> Result<()>;

    fn write_string(&mut self, value: String) -> Result<()>;

    fn write_identifier(&mut self, value: Identifier) -> Result<()>;

    fn write_uuid(&mut self, value: Uuid) -> Result<()>;

    fn write_nbt(&mut self, value: Nbt) -> Result<()>;

    fn write_nbt_tag(&mut self, value: NbtTag) -> Result<()>;

    fn write_option<T, F>(&mut self, value: Option<T>, f: F) -> Result<()>
    where
        F: FnMut(&mut Self, T) -> Result<()>;

    fn write_array<T, F>(&mut self, value: Vec<T>, f: F) -> Result<()>
    where
        F: FnMut(&mut Self, T) -> Result<()>;

    fn write_unprefixed_array<T, F>(&mut self, value: Vec<T>, f: F) -> Result<()>
    where
        F: FnMut(&mut Self, T) -> Result<()>;
}

macro_rules! write_impl {
    ($type:ty) => {
        paste::paste! {
            fn [<write_ $type>](&mut self, value: $type) -> Result<()> {
                Ok(self.[<put_ $type>](value))
            }
        }
    };
}

impl PacketWrite for BytesMut {
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

    fn write_bool(&mut self, value: bool) -> Result<()> {
        self.write_u8(value as u8)
    }

    fn write_varint(&mut self, value: i32) -> Result<()> {
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

    fn write_string(&mut self, value: String) -> Result<()> {
        self.write_varint(value.len() as i32)?;
        self.put(value.as_bytes());
        Ok(())
    }

    fn write_identifier(&mut self, value: Identifier) -> Result<()> {
        self.write_string(value.to_string())
    }

    fn write_uuid(&mut self, value: Uuid) -> Result<()> {
        self.write_u128(value.as_u128())
    }

    fn write_nbt(&mut self, value: Nbt) -> Result<()> {
        let mut data: Vec<u8> = Vec::new();
        value.write_unnamed(&mut data);
        self.put(&*data);
        Ok(())
    }

    fn write_nbt_tag(&mut self, value: NbtTag) -> Result<()> {
        let mut data: Vec<u8> = Vec::new();
        value.write(&mut data);
        self.put(&*data);
        Ok(())
    }

    fn write_option<T, F>(&mut self, value: Option<T>, mut f: F) -> Result<()>
    where
        F: FnMut(&mut Self, T) -> Result<()>,
    {
        self.write_bool(value.is_some())?;
        if let Some(value) = value {
            f(self, value)?;
        }
        Ok(())
    }

    fn write_array<T, F>(&mut self, value: Vec<T>, mut f: F) -> Result<()>
    where
        F: FnMut(&mut Self, T) -> Result<()>,
    {
        self.write_varint(value.len() as i32)?;
        for element in value {
            f(self, element)?;
        }
        Ok(())
    }

    fn write_unprefixed_array<T, F>(&mut self, value: Vec<T>, mut f: F) -> Result<()>
    where
        F: FnMut(&mut Self, T) -> Result<()>,
    {
        for element in value {
            f(self, element)?;
        }
        Ok(())
    }
}
