use std::any::Any;
use std::sync::Arc;
use std::{any::TypeId, marker::PhantomData};

use crate::protocol::decode::{DecodeError, PacketRead};
use crate::protocol::encode::{EncodeError, PacketWrite};

use crate::{
    item::ItemStack,
    registry::{
        CatVariant, ChickenVariant, CowVariant, FrogVariant, PaintingVariant, PigVariant,
        WolfSoundVariant, WolfVariant,
    },
    text::TextComponent,
    util::{DyeColor, Identifier},
};
use cerium_nbt::Nbt;

mod armor_trim;
mod consumable;
mod custom_model_data;
mod equippable;
mod food;
mod tool;
mod tooltip_display;
mod weapon;

pub use armor_trim::ArmorTrim;
pub use consumable::{Consumable, ConsumeEffect};
pub use custom_model_data::CustomModelData;
pub use equippable::Equippable;
pub use food::Food;
use rustc_hash::{FxBuildHasher, FxHashMap};
pub use tool::Tool;
pub use tooltip_display::TooltipDisplay;
pub use weapon::Weapon;

pub type ComponentData = Arc<dyn Any + Send + Sync>;
pub type ComponentMap = FxHashMap<i32, ComponentData>;

// ===== AnyDataComponent =====

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnyDataComponent {
    id: i32,
    key: &'static str,
    type_id: TypeId,
}

impl AnyDataComponent {
    pub fn downcast<T: 'static>(self) -> Result<DataComponent<T>, AnyDataComponent> {
        if TypeId::of::<T>() == self.type_id {
            Ok(DataComponent {
                inner: self,
                __phantom: PhantomData,
            })
        } else {
            Err(self)
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn key(&self) -> &'static str {
        self.key
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
}

// ===== DataComponent =====

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataComponent<T> {
    inner: AnyDataComponent,
    __phantom: PhantomData<T>,
}

impl<T> DataComponent<T> {
    const fn new(id: i32, key: &'static str) -> Self
    where
        T: 'static,
    {
        Self {
            inner: AnyDataComponent {
                id,
                key,
                type_id: TypeId::of::<T>(),
            },
            __phantom: PhantomData,
        }
    }

    pub fn id(&self) -> i32 {
        self.inner.id()
    }

    pub fn key(&self) -> &'static str {
        self.inner.key()
    }

    pub fn into_any(self) -> AnyDataComponent {
        self.inner
    }
}

impl DataComponent<()> {
    pub fn from_key(key: Identifier) -> Option<&'static AnyDataComponent> {
        BY_KEY.get(&key.to_string())
    }

    pub fn from_id(id: i32) -> Option<&'static AnyDataComponent> {
        BY_ID.get(&id)
    }

    pub fn values() -> Vec<&'static AnyDataComponent> {
        BY_ID.values().collect()
    }
}

// ===== DataComponent Impls =====

macro_rules! define_components {
    ($(const $name:ident: DataComponent<$ty:ty$(, $ser:ty)?> = DataComponent::new($id:expr, $key:expr);)*) => {
        impl DataComponent<()> {
            $(pub const $name: DataComponent<$ty> = DataComponent::new($id, $key);)*
        }

        static BY_ID: phf::Map<i32, AnyDataComponent> = phf::phf_map! {
            $($id => DataComponent::$name.inner,)*
        };

        static BY_KEY: phf::Map<&str, AnyDataComponent> = phf::phf_map! {
            $($key => DataComponent::$name.inner,)*
        };

        impl AnyDataComponent {
            pub fn decode_value<R: PacketRead>(self, r: &mut R) -> Result<Arc<dyn Any + Send + Sync>, DecodeError> {
                match self.id() {
                    $($id => decode!(r, $ty $(, $ser)?),)*
                    _ => panic!("error")
                }
            }

            pub fn encode_value<W: PacketWrite>(self, w: &mut W, this: &Arc<dyn Any + Send + Sync>) -> Result<(), EncodeError> {
                match self.id() {
                    $($id => {
                        let value = match this.downcast_ref::<$ty>() {
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

use crate::protocol::DataType;

macro_rules! decode {
    ($reader:expr, $typ:ty) => {
        Ok(Arc::new(<$typ as DataType2<$typ>>::decode($reader)?))
    };
    ($reader:expr, $typ:ty, $ser:ty) => {
        Ok(Arc::new(<$ser as DataType2<$typ>>::decode($reader)?))
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

// See: https://minecraft.wiki/w/Java_Edition_protocol/Slot_data#Structured_components
define_components! {
    const CUSTOM_DATA: DataComponent<Nbt>                               = DataComponent::new(0, "minecraft:custom_data");
    const MAX_STACK_SIZE: DataComponent<i32, VarInt>                    = DataComponent::new(1, "minecraft:max_stack_size");
    const MAX_DAMAGE: DataComponent<i32, VarInt>                        = DataComponent::new(2, "minecraft:max_damage");
    const DAMAGE: DataComponent<i32, VarInt>                            = DataComponent::new(3, "minecraft:damage");
    const UNBREAKABLE: DataComponent<()>                                = DataComponent::new(4, "minecraft:unbreakable");
    const CUSTOM_NAME: DataComponent<TextComponent>                     = DataComponent::new(5, "minecraft:custom_name");
    const ITEM_NAME: DataComponent<TextComponent>                       = DataComponent::new(6, "minecraft:item_name");
    const ITEM_MODEL: DataComponent<TextComponent>                      = DataComponent::new(7, "minecraft:item_model");
    const LORE: DataComponent<Vec<TextComponent>>                       = DataComponent::new(8, "minecraft:lore");
    const RARITY: DataComponent<i32, VarInt>                            = DataComponent::new(9, "minecraft:rarity");
    // const ENCHANTMENTS: DataComponent<Vec<Enchantment>>                = DataComponent::new(10, "minecraft:enchantments");
    // const CAN_PLACE_ON: DataComponent<Vec<BlockPredicate>>             = DataComponent::new(11, "minecraft:can_place_on");
    // const CAN_BREAK: DataComponent<Vec<BlockPredicate>>                = DataComponent::new(12, "minecraft:can_break");
    // const ATTRIBUTE_MODIFIERS: DataComponent<Vec<AttributeModifier>>   = DataComponent::new(13, "minecraft:attribute_modifiers");
    const CUSTOM_MODEL_DATA: DataComponent<CustomModelData>             = DataComponent::new(14, "minecraft:custom_model_data");
    const TOOLTIP_DISPLAY: DataComponent<TooltipDisplay>                = DataComponent::new(15, "minecraft:tooltip_display");
    const REPAIR_COST: DataComponent<i32, VarInt>                       = DataComponent::new(16, "minecraft:repair_cost");
    const CREATIVE_SLOT_LOCK: DataComponent<()>                         = DataComponent::new(17, "minecraft:creative_slot_lock");
    const ENCHANTMENT_GLINT_OVERRIDE: DataComponent<bool>               = DataComponent::new(18, "minecraft:enchantment_glint_override");
    const INTANGIBLE_PROJECTILE: DataComponent<()>                      = DataComponent::new(19, "minecraft:intangible_projectile");
    const FOOD: DataComponent<Food>                                     = DataComponent::new(20, "minecraft:food");
    const CONSUMABLE: DataComponent<Consumable>                         = DataComponent::new(21, "minecraft:consumable");
    // const USE_REMAINDER: DataComponent<Slot>                            = DataComponent::new(22, "minecraft:use_remainder");
    // const USE_COOLDOWN: DataComponent<Cooldown>                         = DataComponent::new(23, "minecraft:use_cooldown");
    const DAMAGE_RESISTANT: DataComponent<Identifier>                   = DataComponent::new(24, "minecraft:damage_resistant");
    const TOOL: DataComponent<Tool>                                     = DataComponent::new(25, "minecraft:tool");
    const WEAPON: DataComponent<Weapon>                                 = DataComponent::new(26, "minecraft:weapon");
    const ENCHANTABLE: DataComponent<i32, VarInt>                       = DataComponent::new(27, "minecraft:enchantable");
    const EQUIPPABLE: DataComponent<Equippable>                         = DataComponent::new(28, "minecraft:equippable");
    // const REPAIRABLE: DataComponent<IdSet>                           = DataComponent::new(29, "minecraft:repairable");
    const GLIDER: DataComponent<()>                                     = DataComponent::new(30, "minecraft:glider");
    const TOOLTIP_STYLE: DataComponent<TextComponent>                   = DataComponent::new(31, "minecraft:tooltip_style");
    const DEATH_PROTECTION: DataComponent<Vec<ConsumeEffect>>           = DataComponent::new(32, "minecraft:death_protection");
    // const BLOCKS_ATTACKS: DataComponent<BlocksAttacks>                 = DataComponent::new(33, "minecraft:blocks_attacks");
    // const STORED_ENCHANTMENTS: DataComponent<Enchantment>              = DataComponent::new(34, "minecraft:stored_enchantments");
    const DYED_COLOR: DataComponent<i32>                                = DataComponent::new(35, "minecraft:dyed_color");
    const MAP_COLOR: DataComponent<i32>                                 = DataComponent::new(36, "minecraft:map_color");
    const MAP_ID: DataComponent<i32, VarInt>                            = DataComponent::new(37, "minecraft:map_id");
    const MAP_DECORATIONS: DataComponent<Nbt>                           = DataComponent::new(38, "minecraft:map_decorations");
    const MAP_POST_PROCESSING: DataComponent<i32, VarInt>               = DataComponent::new(39, "minecraft:map_post_processing");
    // const CHARGED_PROJECTILES: DataComponent<Vec<Slot>>                 = DataComponent::new(40, "minecraft:charged_projectiles");
    // const BUNDLE_CONTENTS: DataComponent<Vec<Slot>>                     = DataComponent::new(41, "minecraft:bundle_contents");
    // const POTION_CONTNETS: DataComponent<PotionContents>                = DataComponent::new(42, "minecraft:potion_contents");
    const POTION_DURATION_SCALE: DataComponent<f32>                     = DataComponent::new(43, "minecraft:potion_duration_scale");
    // const SUSPICIOUS_STEW_EFFECTS: DataComponent<SuspiciousStewEffects> = DataComponent::new(44, "minecraft:suspicious_stew_effects");
    // const WRITABLE_BOOK_CONTENT: DataComponent<WritableBookContent>     = DataComponent::new(45, "minecraft:writable_book_content");
    // const WRITTEN_BOOK_CONTENT: DataComponent<WrittenBookContent>       = DataComponent::new(46, "minecraft:written_book_content");
    const TRIM: DataComponent<ArmorTrim>                                = DataComponent::new(47, "minecraft:trim");
    const DEBUG_STICK_STATE: DataComponent<Nbt>                         = DataComponent::new(48, "minecraft:debug_stick_state");
    const ENTITY_DATA: DataComponent<Nbt>                               = DataComponent::new(49, "minecraft:entity_data");
    const BUCKET_ENTITY_DATA: DataComponent<Nbt>                        = DataComponent::new(50, "minecraft:bucket_entity_data");
    const BLOCK_ENTITY_DATA: DataComponent<Nbt>                         = DataComponent::new(51, "minecraft:block_entity_data");
    // const INSTRUMENT: DataComponent<Instrument>                         = DataComponent::new(52, "minecraft:instrument");
    // const PROVIDES_TRIM_MATERIAL: DataComponent<ProvidesTrimMaterial>   = DataComponent::new(53, "minecraft:provides_trim_material");
    const OMINOUS_BOTTLE_AMPLIFIER: DataComponent<i32, VarInt>          = DataComponent::new(54, "minecraft:ominous_bottle_amplifier");
    // const JUKEBOX_PLAYABLE: DataComponent<Song?>                     = DataComponent::new(55, "minecraft:jukebox_playable");
    const PROVIDES_BANNER_PATTERNS: DataComponent<Identifier>           = DataComponent::new(56, "minecraft:provides_banner_patterns");
    const RECIPES: DataComponent<Nbt>                                   = DataComponent::new(57, "minecraft:recipes");
    // const LODESTONE_TRACKER: DataComponent<LodestoneTracker>            = DataComponent::new(58, "minecraft:lodestone_tracker");
    // const FIREWORK_EXPLOSION: DataComponent<FireworkExplosion>          = DataComponent::new(59, "minecraft:firework_explosion");
    // const FIREWORKS: DataComponent<Fireworks>                           = DataComponent::new(60, "minecraft:fireworks");
    // const PROFILE: DataComponent<GameProfile>                           = DataComponent::new(61, "minecraft:profile");
    const NOTE_BLOCK_SOUND: DataComponent<Identifier>                   = DataComponent::new(62, "minecraft:note_block_sound");
    // const BANNER_PATTERNS: DataComponent<BannerPatterns>                = DataComponent::new(63, "minecraft:banner_patterns");
    const BASE_COLOR: DataComponent<DyeColor>                           = DataComponent::new(64, "minecraft:base_color");
    // const POT_DECORATIONS: DataComponent<PotDecorations>                = DataComponent::new(65, "minecraft:pot_decorations");
    const CONTAINER: DataComponent<Vec<ItemStack>>                      = DataComponent::new(66, "minecraft:container");
    const BLOCK_STATE: DataComponent<ItemBlockState>                    = DataComponent::new(67, "minecraft:block_state");
    // const BEES: DataComponent<Bees>                                     = DataComponent::new(68, "minecraft:bees");
    const LOCK: DataComponent<Nbt>                                      = DataComponent::new(69, "minecraft:lock");
    const CONTAINER_LOOT: DataComponent<Nbt>                            = DataComponent::new(70, "minecraft:container_loot");
    // const BREAK_SOUND: DataComponent<Sound?>                            = DataComponent::new(71, "minecraft:break_sound");
    // const VILLAGER_VARIANT: DataComponent<VillagerVariant>              = DataComponent::new(72, "minecraft:villager/variant");
    const WOLF_VARIANT: DataComponent<WolfVariant>                      = DataComponent::new(73, "minecraft:wolf/variant");
    const WOLF_SOUND_VARIANT: DataComponent<WolfSoundVariant>           = DataComponent::new(74, "minecraft:wolf/sound_variant");
    const WOLF_COLLAR: DataComponent<DyeColor>                          = DataComponent::new(75, "minecraft:wolf/collar");
    // const FOX_VARIANT: DataComponent<FoxVariant>                        = DataComponent::new(76, "minecraft:fox/variant");
    const SALMON_SIZE: DataComponent<Nbt>                               = DataComponent::new(77, "minecraft:salmon/size");
    // const PARROT_VARIANT: DataComponent<ParrotVariant>                  = DataComponent::new(78, "minecraft:parrot/variant");
    // const TROPICAL_FISH_PATTERN: DataComponent<TropicalFishPattern>     = DataComponent::new(79, "minecraft:tropical_fish/pattern");
    // const TROPICAL_FISH_BASE_COLOR: DataComponent<DyeColor>             = DataComponent::new(80, "minecraft:tropical_fish/base_color");
    // const TROPICAL_FISH_PATTERN_COLOR: DataComponent<DyeColor>          = DataComponent::new(81, "minecraft:tropical_fish/pattern_color");
    // const MOOSHROOM_VARIANT: DataComponent<MooshroomVariant>            = DataComponent::new(82, "minecraft:mooshroom/variant");
    // const RABBIT_VARIANT: DataComponent<RabbitVariant>                  = DataComponent::new(83, "minecraft:rabbit/variant");
    const PIG_VARIANT: DataComponent<PigVariant>                        = DataComponent::new(84, "minecraft:pig/variant");
    const COW_VARIANT: DataComponent<CowVariant>                        = DataComponent::new(85, "minecraft:cow/variant");
    const CHICKEN_VARIANT: DataComponent<ChickenVariant>                = DataComponent::new(86, "minecraft:chicken/variant");
    const FROG_VARIANT: DataComponent<FrogVariant>                      = DataComponent::new(87, "minecraft:frog/variant");
    // const HORSE_VARIANT: DataComponent<HorseVariant>                    = DataComponent::new(88, "minecraft:horse/variant");
    const PAINTING_VARIANT: DataComponent<PaintingVariant>              = DataComponent::new(89, "minecraft:painting/variant");
    // const LLAMA_VARIANT: DataComponent<LlamaVariant>                    = DataComponent::new(90, "minecraft:llama/variant");
    // const AXOLOTL_VARIANT: DataComponent<AxolotlVariant>                = DataComponent::new(91, "minecraft:axolotl/variant");
    const CAT_VARIANT: DataComponent<CatVariant>                        = DataComponent::new(92, "minecraft:cat/variant");
    const CAT_COLLAR: DataComponent<DyeColor>                           = DataComponent::new(93, "minecraft:cat/collar");
    const SHEEP_COLOR: DataComponent<DyeColor>                          = DataComponent::new(94, "minecraft:sheep/color");
    const SHULKER_COLOR: DataComponent<DyeColor>                        = DataComponent::new(95, "minecraft:shulker/color");
}

pub struct VarInt;

impl DataType2<i32> for VarInt {
    fn decode<R: PacketRead>(r: &mut R) -> Result<i32, DecodeError> {
        r.read_varint()
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &i32) -> Result<(), EncodeError> {
        w.write_varint(*this)
    }
}

pub trait DataType2<T> {
    fn decode<R: PacketRead>(r: &mut R) -> Result<T, DecodeError>;
    fn encode<W: PacketWrite>(w: &mut W, this: &T) -> Result<(), EncodeError>;
}

impl<D> DataType2<D> for D
where
    D: DataType,
{
    fn decode<R: PacketRead>(r: &mut R) -> Result<D, DecodeError> {
        <D as DataType>::decode(r)
    }
    fn encode<W: PacketWrite>(w: &mut W, this: &D) -> Result<(), EncodeError> {
        <D as DataType>::encode(w, this)
    }
}

pub struct ItemBlockState {
    pub properties: FxHashMap<String, String>,
}

impl ItemBlockState {
    pub const EMPTY: ItemBlockState = ItemBlockState {
        properties: FxHashMap::with_hasher(FxBuildHasher),
    };
}

impl DataType for ItemBlockState {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let properties = r.read_array(|r| Ok((r.read_string()?, r.read_string()?)))?;
        Ok(ItemBlockState {
            properties: properties.into_iter().collect(),
        })
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_array(&this.properties.iter().collect(), |w, (k, v)| {
            w.write_string(k)?;
            w.write_string(v)?;
            Ok(())
        })?;
        Ok(())
    }
}

impl<T> DataType for Vec<T>
where
    T: DataType,
{
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        r.read_array(|r| T::decode(r))
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_array(this, |w, v| T::encode(w, v))
    }
}

impl DataType for Nbt {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        r.read_nbt()
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_nbt(this)
    }
}

impl DataType for Identifier {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        r.read_identifier()
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_identifier(this)
    }
}

impl DataType for TextComponent {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        r.read_component()
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_component(this)
    }
}

impl DataType for () {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        Ok(())
    }

    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        Ok(())
    }
}

impl DataType for f32 {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        r.read_f32()
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_f32(*this)
    }
}

impl DataType for i32 {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        r.read_i32()
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_i32(*this)
    }
}

impl DataType for bool {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        r.read_bool()
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_bool(*this)
    }
}

impl DataType for ItemStack {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        todo!()
    }

    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        todo!()
    }
}

impl DataType for WolfVariant {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        todo!()
    }

    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        todo!()
    }
}

impl DataType for WolfSoundVariant {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        todo!()
    }

    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        todo!()
    }
}

impl DataType for PigVariant {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        todo!()
    }

    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        todo!()
    }
}

impl DataType for CowVariant {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        todo!()
    }

    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        todo!()
    }
}

impl DataType for ChickenVariant {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        todo!()
    }

    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        todo!()
    }
}

impl DataType for FrogVariant {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        todo!()
    }

    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        todo!()
    }
}

impl DataType for PaintingVariant {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        todo!()
    }

    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        todo!()
    }
}

impl DataType for CatVariant {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        todo!()
    }

    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_key() {
        let component1 = DataComponent::from_key("minecraft:custom_data".into()).unwrap();
        let component2 = DataComponent::from_key("minecraft:max_stack_size".into()).unwrap();
        let component3 = DataComponent::from_key("minecraft:shulker/color".into()).unwrap();

        assert_eq!(component1, &DataComponent::CUSTOM_DATA.into_any());
        assert_eq!(component2, &DataComponent::MAX_STACK_SIZE.into_any());
        assert_eq!(component3, &DataComponent::SHULKER_COLOR.into_any());
    }

    #[test]
    fn test_from_id() {
        let component1 = DataComponent::from_id(0).unwrap();
        let component2 = DataComponent::from_id(1).unwrap();
        let component3 = DataComponent::from_id(95).unwrap();

        assert_eq!(component1, &DataComponent::CUSTOM_DATA.into_any());
        assert_eq!(component2, &DataComponent::MAX_STACK_SIZE.into_any());
        assert_eq!(component3, &DataComponent::SHULKER_COLOR.into_any());
    }
}
