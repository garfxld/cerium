use std::{any::Any, collections::HashMap, io::Cursor, ops::Deref, sync::Arc};

use cerium_inventory::{
    Slot,
    item::{AnyDataComponent, DataComponent},
};
use cerium_protocol_macros::packet;
use rustc_hash::FxHashMap;
use simdnbt::owned::{BaseNbt, Nbt, NbtTag};

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::{ClickContainerPacket, ClientPacket},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("set_creative_mode_slot", 0x37)]
pub struct SetCreativeModeSlotPacket {
    pub slot: i16,
    pub clicked_item: Slot,
}

impl ClientPacket for SetCreativeModeSlotPacket {}

impl Decode for SetCreativeModeSlotPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            slot:         r.read_i16()?,
            clicked_item: Slot::decode(r)?,
        })
    }
}

impl Encode for SetCreativeModeSlotPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_i16(this.slot)?;
        Slot::encode(w, this.clicked_item)?;
        Ok(())
    }
}

impl Decode for Slot {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let item_count = r.read_varint()?;
        let item_id = if item_count > 0 {
            Some(r.read_varint()?)
        } else {
            None
        };

        fn test(a: i32) {}

        let mut to_add: FxHashMap<i32, Arc<dyn Any + Send + Sync>> =
            FxHashMap::with_hasher(Default::default());
        let mut to_remove: Vec<i32> = vec![];
        if item_count > 0 {
            let n1 = r.read_varint()?;
            let n2 = r.read_varint()?;

            for _ in (0..n1) {
                let component = AnyDataComponent::decode(r)?;

                let id = component.id();
                let value = component.decode_to(r);
                to_add.insert(id, value);
            }

            for _ in (0..n2) {
                to_remove.push(r.read_varint()?);
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

impl Encode for Slot {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        todo!()
    }
}

pub trait DecodeTo<T>: Decode {
    fn decode_to<R: PacketRead>(self, r: &mut R) -> T;
}

impl Decode for AnyDataComponent {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self::from_id(r.read_varint()?).unwrap())
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
    fn decode_to<R: PacketRead>(self, r: &mut R) -> Arc<dyn Any + Send + Sync> {
        match_component!(self, {
            DataComponent::CUSTOM_DATA => {
               simdnbt::owned::read(&mut Cursor::new(
            &r
                .read_array(|r| r.read_u8())
                .unwrap(),
        ))
            },
            DataComponent::MAX_STACK_SIZE => r.read_varint().unwrap(),
            DataComponent::MAX_DAMAGE => r.read_varint().unwrap(),
            DataComponent::DAMAGE => r.read_varint().unwrap(),
            DataComponent::UNBREAKABLE => (),
            _ => todo!()
        })
    }
}
