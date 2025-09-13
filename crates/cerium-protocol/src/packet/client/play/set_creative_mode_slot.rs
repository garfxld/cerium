use std::{any::Any, collections::HashMap, io::Cursor, ops::Deref, sync::Arc};

use cerium_inventory::{
    Slot,
    item::{AnyDataComponent, DataComponent},
};
use cerium_protocol_macros::packet;
use rustc_hash::FxHashMap;
use simdnbt::owned::{BaseNbt, Nbt, NbtTag};

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("set_creative_mode_slot")]
pub struct SetCreativeModeSlotPacket {
    pub slot: i16,
    pub clicked_item: Slot,
}

impl Decode for SetCreativeModeSlotPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            slot: buffer.read_i16()?,
            clicked_item: Slot::decode(buffer)?,
        })
    }
}

impl Decode for Slot {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        let item_count = buffer.read_varint()?;
        let item_id = if item_count > 0 {
            Some(buffer.read_varint()?)
        } else {
            None
        };

        fn test(a: i32) {}

        let mut to_add: FxHashMap<i32, Arc<dyn Any + Send + Sync>> =
            FxHashMap::with_hasher(Default::default());
        let mut to_remove: Vec<i32> = vec![];
        if item_count > 0 {
            let n1 = buffer.read_varint()?;
            let n2 = buffer.read_varint()?;

            for _ in (0..n1) {
                let component = AnyDataComponent::decode(buffer)?;

                let id = component.id();
                let value = component.decode_to(buffer);
                to_add.insert(id, value);
            }

            for _ in (0..n2) {
                to_remove.push(buffer.read_varint()?);
            }
        }

        Ok(Self {
            item_count,
            item_id,
            to_add,
            to_remove,
        })
    }
}

pub trait DecodeTo<T>: Decode {
    fn decode_to(self, buffer: &mut ByteBuffer) -> T;
}

impl Decode for AnyDataComponent {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self::from_id(buffer.read_varint()?).unwrap())
    }
}
macro_rules! match_component {
    ($component:expr, {
        $(DataComponent::$const_name:ident=> $decode_expr:expr,)*
        _ => $default:expr $(,)?
    }) => {
        {
            match $component.id() {
                $(
                    id if id == DataComponent::$const_name.id() => {
                        Arc::new($decode_expr) as Arc<dyn Any + Send + Sync>
                    }
                )*
                _ => $default,
            }
        }
    };
}

impl DecodeTo<Arc<dyn Any + Send + Sync>> for AnyDataComponent {
    fn decode_to(self, buffer: &mut ByteBuffer) -> Arc<dyn Any + Send + Sync> {
        match_component!(self, {
            DataComponent::CUSTOM_DATA => {
               simdnbt::owned::read(&mut Cursor::new(
            &buffer
                .read_list(|buffer: &mut ByteBuffer| Ok(buffer.read_u8().unwrap()))
                .unwrap(),
        ))
            },
            DataComponent::MAX_STACK_SIZE => buffer.read_varint().unwrap(),
            DataComponent::MAX_DAMAGE => buffer.read_varint().unwrap(),
            DataComponent::DAMAGE => buffer.read_varint().unwrap(),
            DataComponent::UNBREAKABLE => (),
            _ => todo!()
        })
    }
}
