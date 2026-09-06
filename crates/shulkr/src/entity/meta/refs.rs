use super::{CopperGolemState, MetadataRef, ValueType, WeatheringCopperState};
use crate::{text::TextComponent, util::EntityPose};

#[rustfmt::skip]
pub mod entity {
    use super::*;

    pub const ENTITY_FLAGS: MetadataRef<u8> = MetadataRef::new(0, ValueType::BYTE, 0);

    pub const ON_FIRE: MetadataRef<bool> = MetadataRef::bitmask(0, 0x01, false);
    pub const SNEAKING: MetadataRef<bool> = MetadataRef::bitmask(0, 0x02, false);
    pub const _UNUSED: MetadataRef<bool> = MetadataRef::bitmask(0, 0x04, false);
    pub const SPRINTING: MetadataRef<bool> = MetadataRef::bitmask(0, 0x08, false);
    pub const SWIMMING: MetadataRef<bool> = MetadataRef::bitmask(0, 0x10, false);
    pub const INVISIBLE: MetadataRef<bool> = MetadataRef::bitmask(0, 0x20, false);
    pub const GLOWING_EFFECT: MetadataRef<bool> = MetadataRef::bitmask(0, 0x40, false);
    pub const FLYING_WITH_ELYTRA: MetadataRef<bool> = MetadataRef::bitmask(0, 0x80, false);

    pub const AIR_TICKS: MetadataRef<i32>                   = MetadataRef::new(1, ValueType::VAR_INT, 300);
    pub const CUSTOM_NAME: MetadataRef<Option<TextComponent>> = MetadataRef::new(2, ValueType::OPTIONAL_TEXT_COMPONENT, None);
    pub const CUSTOM_NAME_VISIBLE: MetadataRef<bool>        = MetadataRef::new(3, ValueType::BOOL, false);
    pub const SILENT: MetadataRef<bool>                     = MetadataRef::new(4, ValueType::BOOL, false);
    pub const NO_GRAVITY: MetadataRef<bool>                 = MetadataRef::new(5, ValueType::BOOL, false);
    pub const POSE: MetadataRef<EntityPose>                 = MetadataRef::new(6, ValueType::POSE, EntityPose::Standing);
    pub const TICKS_FROZEN_IN_POWDER_SNOW: MetadataRef<i32> = MetadataRef::new(7, ValueType::VAR_INT, 0);
}

#[rustfmt::skip]
pub mod living_entity {
    use crate::util::BlockPosition;
    use super::*;

    pub const LIVING_ENTITY_FLAGS: MetadataRef<u8> = MetadataRef::new(8, ValueType::BYTE, 0);

    pub const IS_HAND_ACTIVE: MetadataRef<bool>            = MetadataRef::bitmask(8, 0x01, false);
    pub const ACTIVE_HAND: MetadataRef<bool>               = MetadataRef::bitmask(8, 0x02, false);
    pub const IS_IN_RIPTIDE_SPIN_ATTACK: MetadataRef<bool> = MetadataRef::bitmask(8, 0x04, false);

    pub const HEALTH: MetadataRef<f32>                    = MetadataRef::new(9, ValueType::FLOAT, 1.0);
    // pub const PARTICLES: MetadataRef<i32>                = MetadataRef::new(10, ValueType::VAR_INT, 0);
    pub const IS_POTION_EFFECT_AMBIENT: MetadataRef<bool> = MetadataRef::new(11, ValueType::BOOL, false);
    pub const NUMBER_OF_ARROWS: MetadataRef<i32>          = MetadataRef::new(12, ValueType::VAR_INT, 0);
    pub const NUMBER_OF_BEE_STINGERS: MetadataRef<i32>    = MetadataRef::new(13, ValueType::VAR_INT, 0);
    pub const LOCATION_OF_BED: MetadataRef<Option<BlockPosition>> = MetadataRef::new(12, ValueType::OPTIONAL_POSITION, None);
}

#[rustfmt::skip]
pub mod mob {
    use super::*;

    pub const MOB_FLAGS: MetadataRef<u8> = MetadataRef::new(15, ValueType::BYTE, 0);

    pub const NO_AI: MetadataRef<bool>          = MetadataRef::bitmask(15, 0x01, false);
    pub const IS_LEFT_HANDED: MetadataRef<bool> = MetadataRef::bitmask(15, 0x02, false);
    pub const IS_AGGRESSIVE: MetadataRef<bool>  = MetadataRef::bitmask(15, 0x04, false);
}

#[rustfmt::skip]
pub mod ageable_mob {
    use super::*;

    pub const IS_BABY: MetadataRef<bool>    = MetadataRef::new(16, ValueType::BOOL, false);
    pub const AGE_LOCKED: MetadataRef<bool> = MetadataRef::new(17, ValueType::BOOL, false);
}

#[rustfmt::skip]
pub mod axolotl {
    use super::*;

    pub const VARIANT: MetadataRef<i32>              = MetadataRef::new(18, ValueType::VAR_INT, 0);
    pub const PLAYING_DEAD: MetadataRef<bool>        = MetadataRef::new(19, ValueType::BOOL, false);
    pub const SPAWNED_FROM_BUCKET: MetadataRef<bool> = MetadataRef::new(20, ValueType::BOOL, false);
}

#[rustfmt::skip]
pub mod fox {
    use super::*;
    use uuid::Uuid;

    pub const VARIANT: MetadataRef<i32>  = MetadataRef::new(18, ValueType::VAR_INT, 0);
    pub const FOX_FLAGS: MetadataRef<u8>        = MetadataRef::new(19, ValueType::BYTE, 0);

    pub const IS_SITTING: MetadataRef<bool>     = MetadataRef::bitmask(19, 0x01, false);
    pub const _UNUSED: MetadataRef<bool>        = MetadataRef::bitmask(19, 0x02, false);
    pub const IS_CROUCHING: MetadataRef<bool>   = MetadataRef::bitmask(19, 0x04, false);
    pub const IS_INTERESTED: MetadataRef<bool>  = MetadataRef::bitmask(19, 0x08, false);
    pub const IS_POUNCING: MetadataRef<bool>    = MetadataRef::bitmask(19, 0x10, false);
    pub const IS_SLEEPING: MetadataRef<bool>    = MetadataRef::bitmask(19, 0x20, false);
    pub const IS_FACEPLANTED: MetadataRef<bool> = MetadataRef::bitmask(19, 0x40, false);
    pub const IS_DEFENDING: MetadataRef<bool>   = MetadataRef::bitmask(19, 0x80, false);

    pub const FIRST_UUID: MetadataRef<Option<Uuid>>  = MetadataRef::new(20, ValueType::OPTIONAL_LIVING_ENTITY, None);
    pub const SECOND_UUID: MetadataRef<Option<Uuid>> = MetadataRef::new(21, ValueType::OPTIONAL_LIVING_ENTITY, None);
}

#[rustfmt::skip]
pub mod rabbit {
    use super::*;

    pub const VARIANT: MetadataRef<i32> = MetadataRef::new(18, ValueType::VAR_INT, 0);
}

#[rustfmt::skip]
pub mod mooshroom {
    use super::*;

    pub const VARIANT: MetadataRef<i32> = MetadataRef::new(18, ValueType::VAR_INT, 0);
}

#[rustfmt::skip]
pub mod parrot {
    use super::*;

    pub const VARIANT: MetadataRef<i32> = MetadataRef::new(20, ValueType::VAR_INT, 0);
}

#[rustfmt::skip]
pub mod horse {
    use super::*;

    pub const VARIANT: MetadataRef<i32> = MetadataRef::new(19, ValueType::VAR_INT, 0);
}

#[rustfmt::skip]
pub mod llama {
    use super::*;

    pub const STRENGTH: MetadataRef<i32> = MetadataRef::new(20, ValueType::VAR_INT, 0);
    pub const VARIANT: MetadataRef<i32> = MetadataRef::new(21, ValueType::VAR_INT, 0);
}

#[rustfmt::skip]
pub mod tropical_fish {
    use super::*;

    pub const VARIANT: MetadataRef<i32> = MetadataRef::new(17, ValueType::VAR_INT, 0);
}

#[rustfmt::skip]
pub mod villager {
    use crate::entity::meta::VillagerData;
    use super::*;

    pub const DATA: MetadataRef<VillagerData> = MetadataRef::new(19, ValueType::VILLAGER_DATA, VillagerData::new());
    pub const DATA_FINALIZED: MetadataRef<bool> = MetadataRef::new(20, ValueType::BOOL, false);
}

#[rustfmt::skip]
pub mod copper_golem {
    use super::*;

    pub const WEATHERING_COPPER_STATE: MetadataRef<WeatheringCopperState> = MetadataRef::new(16, ValueType::WEATHERING_COPPER_STATE, WeatheringCopperState::Unaffected);
    pub const COPPER_GOLEM_STATE: MetadataRef<CopperGolemState> = MetadataRef::new(17, ValueType::COPPER_GOLEM_STATE, CopperGolemState::Idle);
}

#[rustfmt::skip]
pub mod piglin {
    use super::*;

    pub const IS_BABY: MetadataRef<bool> = MetadataRef::new(17, ValueType::BOOL, false);
    pub const IS_CHARGING_CROSSBOW: MetadataRef<bool> = MetadataRef::new(18, ValueType::BOOL, false);
    pub const IS_DANCING: MetadataRef<bool> = MetadataRef::new(19, ValueType::BOOL, false);
}

#[rustfmt::skip]
pub mod strider {
    use super::*;

    pub const TIME_TO_BOOST: MetadataRef<i32> = MetadataRef::new(18, ValueType::VAR_INT, 0);
    pub const IS_SHAKING: MetadataRef<bool> = MetadataRef::new(19, ValueType::BOOL, false);
}

#[rustfmt::skip]
pub mod blaze {
    use super::*;

    pub const BLAZE_FLAGS: MetadataRef<u8>        = MetadataRef::new(16, ValueType::BYTE, 0);

    pub const IS_ON_FIRE: MetadataRef<bool>     = MetadataRef::bitmask(16, 0x01, false);
}

#[rustfmt::skip]
pub mod ghast {
    use super::*;

    pub const IS_ATTACKING: MetadataRef<bool> = MetadataRef::new(16, ValueType::BOOL, false);
}

#[rustfmt::skip]
pub mod wither {
    use super::*;

    pub const CENTER_TARGET: MetadataRef<i32> = MetadataRef::new(16, ValueType::VAR_INT, 0);
    pub const LEFT_TARGET: MetadataRef<i32> = MetadataRef::new(17, ValueType::VAR_INT, 0);
    pub const RIGHT_TARGET: MetadataRef<i32> = MetadataRef::new(18, ValueType::VAR_INT, 0);
    pub const INVULNERABLE_TIME: MetadataRef<i32> = MetadataRef::new(19, ValueType::VAR_INT, 0);
}

#[rustfmt::skip]
pub mod hoglin {
    use super::*;

    pub const IS_IMMUNE_TO_ZOMBIFICATION: MetadataRef<bool> = MetadataRef::new(18, ValueType::BOOL, false);
}

#[rustfmt::skip]
pub mod zoglin {
    use super::*;

    pub const IS_BABY: MetadataRef<bool> = MetadataRef::new(16, ValueType::BOOL, false);
}

#[rustfmt::skip]
pub mod raider {
    use super::*;

    pub const IS_CELEBRATING: MetadataRef<bool> = MetadataRef::new(16, ValueType::BOOL, false);
}

#[rustfmt::skip]
pub mod pillager {
    use super::*;

    pub const IS_CHARGING: MetadataRef<bool> = MetadataRef::new(17, ValueType::BOOL, false);
}

#[rustfmt::skip]
pub mod witch {
    use super::*;

    pub const IS_DRINKING_POTION: MetadataRef<bool> = MetadataRef::new(17, ValueType::BOOL, false);
}

#[rustfmt::skip]
pub mod spellcaster_illager {
    use super::*;

    pub const SPELL: MetadataRef<u8> = MetadataRef::new(17, ValueType::BYTE, 0);
}

#[rustfmt::skip]
pub mod vex {
    use super::*;

    pub const VEX_FLAGS: MetadataRef<u8>        = MetadataRef::new(16, ValueType::BYTE, 0);

    pub const IS_ATTACKING: MetadataRef<bool>     = MetadataRef::bitmask(16, 0x01, false);
}

#[rustfmt::skip]
pub mod interaction {
    use super::*;

    pub const WIDTH: MetadataRef<f32> = MetadataRef::new(8, ValueType::FLOAT, 0.0);
    pub const HEIGHT: MetadataRef<f32> = MetadataRef::new(9, ValueType::FLOAT, 0.0);
    pub const RESPONSIVE: MetadataRef<bool> = MetadataRef::new(10, ValueType::BOOL, false);
}

#[rustfmt::skip]
pub mod avatar {
    use crate::entity::MainHand;
    use super::*;

    pub const MAIN_HAND: MetadataRef<MainHand>              = MetadataRef::new(15, ValueType::HUMANOID_ARM, MainHand::Right);

    pub const AVATAR_FLAGS: MetadataRef<u8>                 = MetadataRef::new(16, ValueType::BYTE, 0);

    pub const CAPE_ENABLED: MetadataRef<bool>               = MetadataRef::bitmask(16, 0x01, false);
    pub const JACKET_ENABLED: MetadataRef<bool>             = MetadataRef::bitmask(16, 0x02, false);
    pub const LEFT_SLEEVE_ENABLED: MetadataRef<bool>        = MetadataRef::bitmask(16, 0x04, false);
    pub const RIGHT_SLEEVE_ENABLED: MetadataRef<bool>       = MetadataRef::bitmask(16, 0x08, false);
    pub const LEFT_PANTS_LEG_ENABLED: MetadataRef<bool>     = MetadataRef::bitmask(16, 0x10, false);
    pub const RIGHT_PANTS_LEG_ENABLED: MetadataRef<bool>    = MetadataRef::bitmask(16, 0x20, false);
    pub const HAT_ENABLED: MetadataRef<bool>                = MetadataRef::bitmask(16, 0x40, false);
}

#[rustfmt::skip]
pub mod mannequin {
    use crate::auth::ResolvableProfile;
    use super::*;

    pub const PROFILE: MetadataRef<ResolvableProfile>        = MetadataRef::new(17, ValueType::RESOLVABLE_PROFILE, ResolvableProfile::empty());
    pub const IMMOVABLE: MetadataRef<bool>                   = MetadataRef::new(18, ValueType::BOOL, false);
    pub const TEXT_BELOW: MetadataRef<Option<TextComponent>> = MetadataRef::new(19, ValueType::OPTIONAL_TEXT_COMPONENT, None);
}

#[rustfmt::skip]
pub mod player {
    use super::*;

    pub const ADDITIONAL_HEARTS: MetadataRef<f32>                  = MetadataRef::new(17, ValueType::FLOAT, 0.0);
    pub const SCORE: MetadataRef<i32>                              = MetadataRef::new(18, ValueType::VAR_INT, 0);
    pub const LEFT_SHOULDER_ENTITY_DATA: MetadataRef<Option<i32>>  = MetadataRef::new(19, ValueType::OPTIONAL_VAR_INT, None);
    pub const RIGHT_SHOULDER_ENTITY_DATA: MetadataRef<Option<i32>> = MetadataRef::new(20, ValueType::OPTIONAL_VAR_INT, None);
}

#[rustfmt::skip]
pub mod pig {
    use super::*;
    use crate::{entity::meta::{PigSoundVariant, PigVariant}, registry::RegistryKey};

    pub const BOOST_TIME: MetadataRef<i32>                              = MetadataRef::new(18, ValueType::VAR_INT, 0);
    pub const VARIANT: MetadataRef<RegistryKey<PigVariant>>             = MetadataRef::new(19, ValueType::PIG_VARIANT, PigVariant::TEMPERATE);
    pub const SOUND_VARIANT: MetadataRef<RegistryKey<PigSoundVariant>>  = MetadataRef::new(20, ValueType::PIG_SOUND_VARIANT, PigSoundVariant::CLASSIC);
}

#[rustfmt::skip]
pub mod tamable_animal {
    use super::*;
    use uuid::Uuid;

    pub const TAMABLE_FLAGS: MetadataRef<u8> = MetadataRef::new(18, ValueType::BYTE, 0);

    pub const IS_SITTING: MetadataRef<bool> = MetadataRef::bitmask(18, 0x01, false);
    pub const IS_TAMED: MetadataRef<bool>   = MetadataRef::bitmask(18, 0x04, false);

    pub const OWNER: MetadataRef<Option<Uuid>> = MetadataRef::new(19, ValueType::OPTIONAL_LIVING_ENTITY, None);
}

#[rustfmt::skip]
pub mod chicken {
    use super::*;
    use crate::{entity::meta::{ChickenSoundVariant, ChickenVariant}, registry::RegistryKey};

    pub const VARIANT: MetadataRef<RegistryKey<ChickenVariant>>              = MetadataRef::new(18, ValueType::CHICKEN_VARIANT, ChickenVariant::TEMPERATE);
    pub const SOUND_VARIANT: MetadataRef<RegistryKey<ChickenSoundVariant>>   = MetadataRef::new(19, ValueType::CHICKEN_SOUND_VARIANT, ChickenSoundVariant::CLASSIC);
}

#[rustfmt::skip]
pub mod cow {
    use super::*;
    use crate::{entity::meta::{CowSoundVariant, CowVariant}, registry::RegistryKey};

    pub const VARIANT: MetadataRef<RegistryKey<CowVariant>>              = MetadataRef::new(18, ValueType::COW_VARIANT, CowVariant::TEMPERATE);
    pub const SOUND_VARIANT: MetadataRef<RegistryKey<CowSoundVariant>>   = MetadataRef::new(19, ValueType::COW_SOUND_VARIANT, CowSoundVariant::CLASSIC);
}

#[rustfmt::skip]
pub mod frog {
    use super::*;
    use crate::{entity::meta::FrogVariant, registry::RegistryKey};

    pub const VARIANT: MetadataRef<RegistryKey<FrogVariant>> = MetadataRef::new(18, ValueType::FROG_VARIANT, FrogVariant::TEMPERATE);
    pub const TONGUE_TARGET: MetadataRef<Option<i32>>        = MetadataRef::new(19, ValueType::OPTIONAL_VAR_INT, None);
}

#[rustfmt::skip]
pub mod wolf {
    use super::*;
    use crate::{entity::meta::{WolfSoundVariant, WolfVariant}, registry::RegistryKey};

    pub const IS_BEGGING: MetadataRef<bool>                             = MetadataRef::new(20, ValueType::BOOL, false);
    pub const COLLAR_COLOR: MetadataRef<i32>                            = MetadataRef::new(21, ValueType::VAR_INT, 14);
    pub const ANGER_TIME: MetadataRef<i64>                              = MetadataRef::new(22, ValueType::VAR_LONG, 0);
    pub const VARIANT: MetadataRef<RegistryKey<WolfVariant>>            = MetadataRef::new(23, ValueType::WOLF_VARIANT, WolfVariant::PALE);
    pub const SOUND_VARIANT: MetadataRef<RegistryKey<WolfSoundVariant>> = MetadataRef::new(24, ValueType::WOLF_SOUND_VARIANT, WolfSoundVariant::CLASSIC);
}
