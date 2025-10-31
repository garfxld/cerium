use std::{any::Any, marker::PhantomData, sync::Arc};

use rustc_hash::FxHashMap;

pub mod entity;

use crate::item::DataType2;
use crate::item::VarInt;

use crate::protocol::decode::DecodeError;
use crate::protocol::decode::PacketRead;
use crate::protocol::encode::EncodeError;
use crate::protocol::encode::PacketWrite;
use crate::text::TextComponent;
use crate::util::EntityPose;

pub struct MetadataHolder {
    pub entries: FxHashMap<i32, AnyValue>, // i32, Ref((i32, dyn Any), default_value)
}

impl MetadataHolder {
    pub fn get<T: Clone + 'static>(&self, r#ref: MetadataRef<T>) -> Option<T> {
        match &r#ref.kind {
            MetadataRefKind::Direct { id } => {
                let Some(entry) = self.entries.get(id) else {
                    return Some(r#ref.default_value());
                };
                entry.downcast_ref::<T>().cloned()
            }
            MetadataRefKind::Bitmask { parent_id, mask } => {
                let Some(entry) = self.entries.get(parent_id) else {
                    return Some(r#ref.default_value());
                };

                let flags = entry.downcast_ref::<u8>()?;
                let result = (flags & mask) != 0;
                // Safe transmute from bool to T (only valid when T is bool)
                unsafe { Some(std::mem::transmute_copy(&result)) }
            }
        }
    }

    pub fn set<T: Send + Sync + 'static>(&mut self, r#ref: MetadataRef<T>, value: T) {
        match &r#ref.kind {
            MetadataRefKind::Direct { id } => {
                let Some(entry) = self.entries.get_mut(id) else {
                    let type_id = r#ref.type_id().id;
                    let value = AnyValue::new(type_id, Arc::new(value));
                    self.entries.insert(*id, value);
                    return;
                };
                entry.set(Arc::new(value));
            }
            MetadataRefKind::Bitmask { parent_id, mask } => {
                // For bool bitmask fields - assumes T is bool
                let enabled = unsafe { *(&value as *const T as *const bool) };

                let entry = self.entries.entry(*parent_id).or_insert_with(|| {
                    AnyValue::new(0, Arc::new(0u8)) // BYTE type id = 0
                });

                if let Some(flags) = entry.downcast_ref::<u8>() {
                    let mut new_flags = *flags;
                    if enabled {
                        new_flags |= mask;
                    } else {
                        new_flags &= !mask;
                    }
                    entry.set(Arc::new(new_flags));
                }
            }
        }
    }
}

pub enum MetadataRefKind {
    Direct { id: i32 },
    Bitmask { parent_id: i32, mask: u8 },
}

pub struct MetadataRef<T> {
    kind: MetadataRefKind,
    type_id: ValueType<T>,
    default_value: T,
}

impl<T> MetadataRef<T> {
    pub const fn new(id: i32, type_id: ValueType<T>, default_value: T) -> Self {
        Self {
            kind: MetadataRefKind::Direct { id },
            type_id,
            default_value,
        }
    }

    pub fn id(&self) -> i32 {
        match &self.kind {
            MetadataRefKind::Direct { id } => *id,
            MetadataRefKind::Bitmask { parent_id, .. } => *parent_id,
        }
    }

    pub fn type_id(&self) -> &ValueType<T> {
        &self.type_id
    }

    pub fn default_value(self) -> T {
        self.default_value
    }
}

impl MetadataRef<bool> {
    pub const fn bitmask(parent_id: i32, mask: u8, default_value: bool) -> Self {
        Self {
            kind: MetadataRefKind::Bitmask { parent_id, mask },
            type_id: ValueType::BOOL,
            default_value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnyValue {
    type_id: i32,
    value: Arc<dyn Any + Send + Sync>,
}

impl AnyValue {
    fn new(id: i32, value: Arc<dyn Any + Send + Sync>) -> Self {
        Self { type_id: id, value }
    }

    pub fn type_id(&self) -> i32 {
        self.type_id
    }

    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.value.downcast_ref()
    }

    pub fn set(&mut self, value: Arc<dyn Any + Send + Sync>) {
        self.value = value;
    }
}

#[rustfmt::skip]
impl MetadataRef<()> {
    pub const ENTITY_FLAGS: MetadataRef<u8>                 = MetadataRef::new(0, ValueType::BYTE, 0);

    pub const ON_FIRE: MetadataRef<bool>                    = MetadataRef::bitmask(0, 0x01, false);
    pub const SNEAKING: MetadataRef<bool>                   = MetadataRef::bitmask(0, 0x02, false);
    pub const SPRINTING: MetadataRef<bool>                  = MetadataRef::bitmask(0, 0x08, false);
    pub const SWIMMING: MetadataRef<bool>                   = MetadataRef::bitmask(0, 0x10, false);
    pub const INVISIBLE: MetadataRef<bool>                  = MetadataRef::bitmask(0, 0x20, false);
    pub const GLOWING_EFFECT: MetadataRef<bool>             = MetadataRef::bitmask(0, 0x40, false);
    pub const FLYING_WITH_ELYTRA: MetadataRef<bool>         = MetadataRef::bitmask(0, 0x80, false);

    pub const AIR_TICKS: MetadataRef<i32>                   = MetadataRef::new(1, ValueType::VAR_INT, 300);
    pub const CUSTOM_NAME: MetadataRef<Option<TextComponent>> = MetadataRef::new(2, ValueType::OPTIONAL_TEXT_COMPONENT, None);
    pub const CUSTOM_NAME_VISIBLE: MetadataRef<bool>        = MetadataRef::new(3, ValueType::BOOL, false);
    pub const SILENT: MetadataRef<bool>                     = MetadataRef::new(4, ValueType::BOOL, false);
    pub const NO_GRAVITY: MetadataRef<bool>                 = MetadataRef::new(5, ValueType::BOOL, false);
    pub const POSE: MetadataRef<EntityPose>                 = MetadataRef::new(6, ValueType::POSE, EntityPose::Standing);
    pub const TICKS_FROZEN_IN_POWDER_SNOW: MetadataRef<i32> = MetadataRef::new(7, ValueType::VAR_INT, 0);

    pub const WEATHERING_COPPER_STATE: MetadataRef<WeatheringCopperState> = MetadataRef::new(
        16,
        ValueType::WEATHERING_COPPER_STATE,
        WeatheringCopperState::Unaffected,
    );
    pub const COPPER_GOLEM_STATE: MetadataRef<CopperGolemState> =
        MetadataRef::new(17, ValueType::COPPER_GOLEM_STATE, CopperGolemState::Idle);
}

#[derive(Debug, Clone, Copy)]
pub struct ValueType<T> {
    id: i32,
    __phantom: PhantomData<T>,
}

impl<T> ValueType<T> {
    pub const fn new(id: i32) -> Self {
        Self {
            id,
            __phantom: PhantomData,
        }
    }
}

impl ValueType<()> {}

macro_rules! define_types {
    ($(const $name:ident: ValueType<$ty:ty$(, $ser:ty)?> = ValueType::new($id:expr);)*) => {
        impl ValueType<()> {
            $(pub const $name: ValueType<$ty> = ValueType::new($id);)*
        }

        // static BY_ID: phf::Map<i32, AnyDataComponent> = phf::phf_map! {
        //     $($id => DataComponent::$name.inner,)*
        // };

        // static BY_KEY: phf::Map<&str, AnyDataComponent> = phf::phf_map! {
        //     $($key => DataComponent::$name.inner,)*
        // };

        impl AnyValue {
            pub fn decode_value<R: PacketRead>(r: &mut R) -> Result<AnyValue, DecodeError> {
                let type_id = r.read_varint()?;
                let value: Arc<dyn Any + Send + Sync> = match type_id {
                    $($id => decode!(r, $ty $(, $ser)?),)*
                    _ => panic!("error")
                };
                Ok(AnyValue {
                    type_id,
                    value
                })
            }

            pub fn encode_value<W: PacketWrite>(&self, w: &mut W) -> Result<(), EncodeError> {
                w.write_varint(self.type_id())?;
                match self.type_id() {
                    $($id => {
                        let value = match self.value.downcast_ref::<$ty>() {
                            Some(v) => v,
                            None => panic!("error"),
                        };

                        encode!(w, value, $ty $(, $ser)?)
                    },)*
                    _ => panic!("error")
                }
                Ok(())
            }
        }
    };
}

macro_rules! decode {
    ($reader:expr, $typ:ty) => {
        Arc::new(<$typ as DataType2<$typ>>::decode($reader)?)
    };
    ($reader:expr, $typ:ty, $ser:ty) => {
        Arc::new(<$ser as DataType2<$typ>>::decode($reader)?)
    };
}

macro_rules! encode {
    ($writer:expr, $value:expr, $typ:ty) => {
        <$typ as DataType2<$typ>>::encode($writer, $value)?
    };
    ($writer:expr, $value:expr, $typ:ty, $ser:ty) => {
        <$ser as DataType2<$typ>>::encode($writer, $value)?
    };
}

define_types! {
    const BOOL: ValueType<bool> = ValueType::new(-1);

    const BYTE: ValueType<u8> = ValueType::new(0);
    const VAR_INT: ValueType<i32, VarInt> = ValueType::new(1);

    const OPTIONAL_TEXT_COMPONENT: ValueType<Option<TextComponent>> = ValueType::new(6);
    const POSE: ValueType<EntityPose> = ValueType::new(20);

    const COPPER_GOLEM_STATE: ValueType<CopperGolemState> = ValueType::new(32);
    const WEATHERING_COPPER_STATE: ValueType<WeatheringCopperState> = ValueType::new(33);
}

impl DataType2<u8> for u8 {
    fn decode<R: PacketRead>(r: &mut R) -> Result<u8, DecodeError> {
        r.read_u8()
    }
    fn encode<W: PacketWrite>(w: &mut W, this: &u8) -> Result<(), EncodeError> {
        w.write_u8(*this)
    }
}

impl<T> DataType2<Option<T>> for Option<T>
where
    T: DataType2<T>,
{
    fn decode<R: PacketRead>(r: &mut R) -> Result<Option<T>, DecodeError> {
        if r.read_bool()? {
            Ok(Some(T::decode(r)?))
        } else {
            Ok(None)
        }
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Option<T>) -> Result<(), EncodeError> {
        w.write_bool(this.is_some())?;
        if let Some(this) = this {
            T::encode(w, this)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CopperGolemState {
    Idle,
    GettingItem,
    GettingNoItem,
    DroppingItem,
    DroppingNoItem,
}

impl TryFrom<i32> for CopperGolemState {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let this = match value {
            0 => CopperGolemState::Idle,
            1 => CopperGolemState::GettingItem,
            2 => CopperGolemState::GettingNoItem,
            3 => CopperGolemState::DroppingItem,
            4 => CopperGolemState::DroppingNoItem,
            _ => return Err(()),
        };
        Ok(this)
    }
}

impl DataType2<CopperGolemState> for CopperGolemState {
    fn decode<R: PacketRead>(r: &mut R) -> Result<CopperGolemState, DecodeError> {
        CopperGolemState::try_from(r.read_varint()?).map_err(|_| todo!())
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &CopperGolemState) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeatheringCopperState {
    Unaffected,
    Exposed,
    Weathered,
    Oxidized,
}

impl TryFrom<i32> for WeatheringCopperState {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let this = match value {
            0 => Self::Unaffected,
            1 => Self::Exposed,
            2 => Self::Weathered,
            3 => Self::Oxidized,
            _ => return Err(()),
        };
        Ok(this)
    }
}

impl DataType2<WeatheringCopperState> for WeatheringCopperState {
    fn decode<R: PacketRead>(r: &mut R) -> Result<WeatheringCopperState, DecodeError> {
        WeatheringCopperState::try_from(r.read_varint()?).map_err(|_| todo!())
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &WeatheringCopperState) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)
    }
}

impl DataType2<EntityPose> for EntityPose {
    fn decode<R: PacketRead>(r: &mut R) -> Result<EntityPose, DecodeError> {
        EntityPose::try_from(r.read_varint()?).map_err(|_| todo!())
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &EntityPose) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)
    }
}
