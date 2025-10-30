use bytes::BufMut;
use std::any::TypeId;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    protocol::{
        ProtocolState,
        packet::{
            AcknowledgeBlockChangePacket, BlockUpdatePacket, ChunkBatchFinishedPacket, ChunkBatchStartPacket, ChunkDataAndUpdateLightPacket, DisconnectPacket, DisplayObjectivePacket, EncryptionRequestPacket, EntityAnimationPacket, EntityPositionPacket, EntityPositionRotationPacket, EntityRotationPacket, FeatureFlagsPacket, FinishConfigPacket, GameEventPacket, LoginDisconnectPacket, LoginPacket, LoginSuccessPacket, OpenScreenPacket, Packet, PlayerInfoRemovePacket, PlayerInfoUpdatePacket, PluginMessagePacket, PongResponsePacket, RegistryDataPacket, RemoveEntitiesPacket, ServerPacket, SetBlockDestroyStagePacket, SetCenterChunkPacket, SetCompressionPacket, SetContainerContentPacket, SetContainerSlotPacket, SetEntityMetadataPacket, SetHeadRotationPacket, SetTablistHeaderFooterPacket, SpawnEntityPacket, StatusResponsePacket, SyncPlayerPositionPacket, SystemChatMessagePacket, UnloadChunkPacket, UpdateObjectivesPacket, UpdateScorePacket, WorldEventPacket, server::{
                CloseContainerPacket, KeepAlivePacket, KnownPacksPacket, PlayerAbilitiesPacket,
            }
        },
    },
    text::Component,
    util::{BlockPosition, Identifier},
};
use cerium_nbt::{Nbt, NbtTag};

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("{0}")]
    Encode(String),
    #[error("std::io::Error - {0}")]
    IoError(std::io::Error),
}

pub trait Encode
where
    Self: Sized,
{
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<()>;
}

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

    fn write_string(&mut self, value: &String) -> Result<()>;

    fn write_identifier(&mut self, value: &Identifier) -> Result<()>;

    fn write_uuid(&mut self, value: &Uuid) -> Result<()>;

    fn write_nbt(&mut self, value: &Nbt) -> Result<()>;

    fn write_nbt_tag(&mut self, value: &NbtTag) -> Result<()>;

    fn write_component(&mut self, value: &Component) -> Result<()>;

    fn write_position(&mut self, value: &BlockPosition) -> Result<()>;

    fn write_option<T, F>(&mut self, value: &Option<T>, f: F) -> Result<()>
    where
        F: FnMut(&mut Self, &T) -> Result<()>;

    fn write_array<T, F>(&mut self, value: &Vec<T>, f: F) -> Result<()>
    where
        F: FnMut(&mut Self, &T) -> Result<()>;

    fn write_boxed_slice(&mut self, value: &Box<[u8]>) -> Result<()>;

    fn write_unprefixed_array<T, F>(&mut self, value: &Vec<T>, f: F) -> Result<()>
    where
        F: FnMut(&mut Self, &T) -> Result<()>;
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

impl<B: BufMut> PacketWrite for B {
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

    fn write_string(&mut self, value: &String) -> Result<()> {
        self.write_varint(value.len() as i32)?;
        self.put(value.as_bytes());
        Ok(())
    }

    fn write_identifier(&mut self, value: &Identifier) -> Result<()> {
        self.write_string(&value.to_string())
    }

    fn write_uuid(&mut self, value: &Uuid) -> Result<()> {
        self.write_u128(value.as_u128())
    }

    fn write_nbt(&mut self, value: &Nbt) -> Result<()> {
        let mut data: Vec<u8> = Vec::new();
        value.write_unnamed(&mut data).unwrap();
        self.put(&*data);
        Ok(())
    }

    fn write_nbt_tag(&mut self, value: &NbtTag) -> Result<()> {
        let mut data: Vec<u8> = Vec::new();
        value.write(&mut data).unwrap();
        self.put(&*data);
        Ok(())
    }

    fn write_component(&mut self, value: &Component) -> Result<()> {
        let mut data: Vec<u8> = Vec::new();
        cerium_nbt::to_bytes_unnamed(&value, &mut data).unwrap();
        self.put(&*data);
        Ok(())
    }

    fn write_position(&mut self, value: &BlockPosition) -> Result<()> {
        let encoded =
            ((value.x() & 0x3FFFFFF) << 38) | ((value.z() & 0x3FFFFFF) << 12) | (value.y() & 0xFFF);

        self.write_i64(encoded)?;
        Ok(())
    }

    fn write_option<T, F>(&mut self, value: &Option<T>, mut f: F) -> Result<()>
    where
        F: FnMut(&mut Self, &T) -> Result<()>,
    {
        self.write_bool(value.is_some())?;
        if let Some(value) = value {
            f(self, &value)?;
        }
        Ok(())
    }

    fn write_array<T, F>(&mut self, value: &Vec<T>, mut f: F) -> Result<()>
    where
        F: FnMut(&mut Self, &T) -> Result<()>,
    {
        self.write_varint(value.len() as i32)?;
        for element in value {
            f(self, element)?;
        }
        Ok(())
    }

    fn write_boxed_slice(&mut self, value: &Box<[u8]>) -> Result<()> {
        self.write_varint(value.len() as i32)?;
        for element in value {
            self.write_u8(*element)?;
        }
        Ok(())
    }

    fn write_unprefixed_array<T, F>(&mut self, value: &Vec<T>, mut f: F) -> Result<()>
    where
        F: FnMut(&mut Self, &T) -> Result<()>,
    {
        for element in value {
            f(self, element)?;
        }
        Ok(())
    }
}

pub fn packet_id<P>(state: &ProtocolState) -> Option<i32>
where
    P: Packet + ServerPacket + 'static,
{
    let type_id = TypeId::of::<P>();
    match state {
        ProtocolState::Handshake => None,
        ProtocolState::Status => status::<P>(type_id),
        ProtocolState::Login => login::<P>(type_id),
        ProtocolState::Config => config::<P>(type_id),
        ProtocolState::Play => play::<P>(type_id),
    }
}

fn status<P>(type_id: TypeId) -> Option<i32>
where
    P: Packet + ServerPacket + 'static,
{
    Some(match () {
        _ if type_id == TypeId::of::<StatusResponsePacket>() => 0x00,
        _ if type_id == TypeId::of::<PongResponsePacket>() => 0x01,
        _ => return None,
    })
}

fn login<P>(type_id: TypeId) -> Option<i32>
where
    P: Packet + ServerPacket + 'static,
{
    Some(match () {
        _ if type_id == TypeId::of::<LoginDisconnectPacket>() => 0x00,
        _ if type_id == TypeId::of::<EncryptionRequestPacket>() => 0x01,
        _ if type_id == TypeId::of::<LoginSuccessPacket>() => 0x02,
        _ if type_id == TypeId::of::<SetCompressionPacket>() => 0x03,
        _ if type_id == TypeId::of::<PluginMessagePacket>() => 0x04,
        // _ if type_id == TypeId::of::<CookieRequestPacket>() => 0x05,
        _ => return None,
    })
}

fn config<P>(type_id: TypeId) -> Option<i32>
where
    P: Packet + ServerPacket + 'static,
{
    Some(match () {
        // _ if type_id == TypeId::of::<CookieRequestPacket>() => 0x00,
        _ if type_id == TypeId::of::<PluginMessagePacket>() => 0x01,
        _ if type_id == TypeId::of::<DisconnectPacket>() => 0x02,
        _ if type_id == TypeId::of::<FinishConfigPacket>() => 0x03,
        _ if type_id == TypeId::of::<KeepAlivePacket>() => 0x04,
        // _ if type_id == TypeId::of::<PingPacket>() => 0x05,
        // _ if type_id == TypeId::of::<ResetChatPacket>() => 0x06,
        _ if type_id == TypeId::of::<RegistryDataPacket>() => 0x07,
        // _ if type_id == TypeId::of::<RemoveResourcePackPacket>() => 0x08,
        // _ if type_id == TypeId::of::<AddResourcePackPacket>() => 0x09,
        // _ if type_id == TypeId::of::<StoreCookiePacket>() => 0x0A,
        // _ if type_id == TypeId::of::<TransferPacket>() => 0x0B,
        _ if type_id == TypeId::of::<FeatureFlagsPacket>() => 0x0C,
        // _ if type_id == TypeId::of::<UpdateTagsPacket>() => 0x0D,
        _ if type_id == TypeId::of::<KnownPacksPacket>() => 0x0E,
        // _ if type_id == TypeId::of::<CustomReportDetailsPacket>() => 0x0F,
        // _ if type_id == TypeId::of::<ServerLinksPacket>() => 0x10,
        // _ if type_id == TypeId::of::<ClearDialogPacket>() => 0x11,
        // _ if type_id == TypeId::of::<ShowDialogPacket>() => 0x12,
        _ => return None,
    })
}

fn play<P>(type_id: TypeId) -> Option<i32>
where
    P: Packet + ServerPacket + 'static,
{
    Some(match () {
        // _ if type_id == TypeId::of::<BundleDelimiterPacket>() => 0x00,
        _ if type_id == TypeId::of::<SpawnEntityPacket>() => 0x01,
        _ if type_id == TypeId::of::<EntityAnimationPacket>() => 0x02,
        // _ if type_id == TypeId::of::<AwardStatsPacket>() => 0x03,
        _ if type_id == TypeId::of::<AcknowledgeBlockChangePacket>() => 0x04,
        _ if type_id == TypeId::of::<SetBlockDestroyStagePacket>() => 0x05,
        // _ if type_id == TypeId::of::<BlockEntityDataPacket>() => 0x06,
        // _ if type_id == TypeId::of::<BlockEventPacket>() => 0x07,
        _ if type_id == TypeId::of::<BlockUpdatePacket>() => 0x08,
        // _ if type_id == TypeId::of::<BossEventPacket>() => 0x09,
        // _ if type_id == TypeId::of::<ChangeDifficultyPacket>() => 0x0A,
        _ if type_id == TypeId::of::<ChunkBatchFinishedPacket>() => 0x0B,
        _ if type_id == TypeId::of::<ChunkBatchStartPacket>() => 0x0C,
        // _ if type_id == TypeId::of::<ChunksBiomesPacket>() => 0x0D,
        // _ if type_id == TypeId::of::<ClearTitlesPacket>() => 0x0E,
        // _ if type_id == TypeId::of::<CommandSuggestionsPacket>() => 0x0F,
        // _ if type_id == TypeId::of::<CommandsPacket>() => 0x10,
        _ if type_id == TypeId::of::<CloseContainerPacket>() => 0x11,
        _ if type_id == TypeId::of::<SetContainerContentPacket>() => 0x12,
        // _ if type_id == TypeId::of::<SetContainerData() => 0x13,
        _ if type_id == TypeId::of::<SetContainerSlotPacket>() => 0x14,
        // _ if type_id == TypeId::of::<CookieRequestPacket>() => 0x15,
        // _ if type_id == TypeId::of::<CooldownPacket>() => 0x16,
        // _ if type_id == TypeId::of::<CustomChatCompletionsPacket>() => 0x17,
        // _ if type_id == TypeId::of::<CustomPayloadPacket>() => 0x18,
        // _ if type_id == TypeId::of::<DamageEventPacket>() => 0x19,
        // _ if type_id == TypeId::of::<DebugBlockValuePacket>() => 0x1A,
        // _ if type_id == TypeId::of::<DebugChunkValuePacket>() => 0x1B,
        // _ if type_id == TypeId::of::<DebugEntityValuePacket>() => 0x1C,
        // _ if type_id == TypeId::of::<DebugEventPacket>() => 0x1D,
        // _ if type_id == TypeId::of::<DebugSamplePacket>() => 0x1E,
        // _ if type_id == TypeId::of::<DeleteChatPacket>() => 0x1F,
        _ if type_id == TypeId::of::<DisconnectPacket>() => 0x20,
        // _ if type_id == TypeId::of::<DisguisedChatPacket>() => 0x21,
        // _ if type_id == TypeId::of::<EntityEventPacket>() => 0x22,
        // _ if type_id == TypeId::of::<EntityPositionSyncPacket>() => 0x23,
        // _ if type_id == TypeId::of::<ExplodePacket>() => 0x24,
        _ if type_id == TypeId::of::<UnloadChunkPacket>() => 0x25,
        _ if type_id == TypeId::of::<GameEventPacket>() => 0x26,
        // _ if type_id == TypeId::of::<GameTestHighlightPosPacket>() => 0x27,
        // _ if type_id == TypeId::of::<HorseScreenOpenPacket>() => 0x28,
        // _ if type_id == TypeId::of::<HurtAnimationPacket>() => 0x29,
        // _ if type_id == TypeId::of::<InitializeBorderPacket>() => 0x2A,
        _ if type_id == TypeId::of::<KeepAlivePacket>() => 0x2B,
        _ if type_id == TypeId::of::<ChunkDataAndUpdateLightPacket>() => 0x2C,
        _ if type_id == TypeId::of::<WorldEventPacket>() => 0x2D,
        // _ if type_id == TypeId::of::<LevelParticlesPacket>() => 0x2E,
        // _ if type_id == TypeId::of::<LightUpdatePacket>() => 0x2F,
        _ if type_id == TypeId::of::<LoginPacket>() => 0x30,
        // _ if type_id == TypeId::of::<MapItemDataPacket>() => 0x31,
        // _ if type_id == TypeId::of::<MerchantOffersPacket>() => 0x32,
        _ if type_id == TypeId::of::<EntityPositionPacket>() => 0x33,
        _ if type_id == TypeId::of::<EntityPositionRotationPacket>() => 0x34,
        // _ if type_id == TypeId::of::<MoveMinecartAlongTrackPacket>() => 0x35,
        _ if type_id == TypeId::of::<EntityRotationPacket>() => 0x36,
        // _ if type_id == TypeId::of::<MoveVehiclePacket>() => 0x37,
        // _ if type_id == TypeId::of::<OpenBookPacket>() => 0x38,
        _ if type_id == TypeId::of::<OpenScreenPacket>() => 0x39,
        // _ if type_id == TypeId::of::<OpenSignEditorPacket>() => 0x3A,
        // _ if type_id == TypeId::of::<PingPacket>() => 0x3B,
        // _ if type_id == TypeId::of::<PongResponsePacket>() => 0x3C,
        // _ if type_id == TypeId::of::<PlaceGhostRecipePacket>() => 0x3D,
        _ if type_id == TypeId::of::<PlayerAbilitiesPacket>() => 0x3E,
        // _ if type_id == TypeId::of::<PlayerChatPacket>() => 0x3F,
        // _ if type_id == TypeId::of::<PlayerCombatEndPacket>() => 0x40,
        // _ if type_id == TypeId::of::<PlayerCombatEnterPacket>() => 0x41,
        // _ if type_id == TypeId::of::<PlayerCombatKillPacket>() => 0x42,
        _ if type_id == TypeId::of::<PlayerInfoRemovePacket>() => 0x43,
        _ if type_id == TypeId::of::<PlayerInfoUpdatePacket>() => 0x44,
        // _ if type_id == TypeId::of::<PlayerLookAtPacket>() => 0x45,
        _ if type_id == TypeId::of::<SyncPlayerPositionPacket>() => 0x46,
        // _ if type_id == TypeId::of::<PlayerRotationPacket>() => 0x47,
        // _ if type_id == TypeId::of::<RecipeBookAddPacket>() => 0x48,
        // _ if type_id == TypeId::of::<RecipeBookRemovePacket>() => 0x49,
        // _ if type_id == TypeId::of::<RecipeBookSettingsPacket>() => 0x4A,
        _ if type_id == TypeId::of::<RemoveEntitiesPacket>() => 0x4B,
        // _ if type_id == TypeId::of::<RemoveMobEffectPacket>() => 0x4C,
        // _ if type_id == TypeId::of::<ResetScorePacket>() => 0x4D,
        // _ if type_id == TypeId::of::<ResourcePackPopPacket>() => 0x4E,
        // _ if type_id == TypeId::of::<ResourcePackPushPacket>() => 0x4F,
        // _ if type_id == TypeId::of::<RespawnPacket>() => 0x50,
        _ if type_id == TypeId::of::<SetHeadRotationPacket>() => 0x51,
        // _ if type_id == TypeId::of::<SectionBlocksUpdatePacket>() => 0x52,
        // _ if type_id == TypeId::of::<SelectAdvancementsTabPacket>() => 0x53,
        // _ if type_id == TypeId::of::<ServerDataPacket>() => 0x54,
        // _ if type_id == TypeId::of::<SetActionBarTextPacket>() => 0x55,
        // _ if type_id == TypeId::of::<SetBorderCenterPacket>() => 0x56,
        // _ if type_id == TypeId::of::<SetBorderLerpSizePacket>() => 0x57,
        // _ if type_id == TypeId::of::<SetBorderSizePacket>() => 0x58,
        // _ if type_id == TypeId::of::<SetBorderWarningDelayPacket>() => 0x59,
        // _ if type_id == TypeId::of::<SetBorderWarningDistancePacket>() => 0x5A,
        // _ if type_id == TypeId::of::<SetCameraPacket>() => 0x5B,
        _ if type_id == TypeId::of::<SetCenterChunkPacket>() => 0x5C,
        // _ if type_id == TypeId::of::<SetChunkCacheRadiusPacket>() => 0x5D,
        // _ if type_id == TypeId::of::<SetCursorItemPacket>() => 0x5E,
        // _ if type_id == TypeId::of::<SetDefaultSpawnPositionPacket>() => 0x5F,
        _ if type_id == TypeId::of::<DisplayObjectivePacket>() => 0x60,
        _ if type_id == TypeId::of::<SetEntityMetadataPacket>() => 0x61,
        // _ if type_id == TypeId::of::<SetEntityLinkPacket>() => 0x62,
        // _ if type_id == TypeId::of::<SetEntityMotionPacket>() => 0x63,
        // _ if type_id == TypeId::of::<SetEquipmentPacket>() => 0x64,
        // _ if type_id == TypeId::of::<SetExperiencePacket>() => 0x65,
        // _ if type_id == TypeId::of::<SetHealthPacket>() => 0x66,
        // _ if type_id == TypeId::of::<SetHeldSlotPacket>() => 0x67,
        _ if type_id == TypeId::of::<UpdateObjectivesPacket>() => 0x68,
        // _ if type_id == TypeId::of::<SetPassengersPacket>() => 0x69,
        // _ if type_id == TypeId::of::<SetPlayerInventoryPacket>() => 0x6A,
        // _ if type_id == TypeId::of::<SetPlayerTeamPacket>() => 0x6B,
        _ if type_id == TypeId::of::<UpdateScorePacket>() => 0x6C,
        // _ if type_id == TypeId::of::<SetSimulationDistancePacket>() => 0x6D,
        // _ if type_id == TypeId::of::<SetSubtitleTextPacket>() => 0x6E,
        // _ if type_id == TypeId::of::<SetTimePacket>() => 0x6F,
        // _ if type_id == TypeId::of::<SetTitleTextPacket>() => 0x70,
        // _ if type_id == TypeId::of::<SetTitlesAnimationPacket>() => 0x71,
        // _ if type_id == TypeId::of::<SoundEntityPacket>() => 0x72,
        // _ if type_id == TypeId::of::<SoundPacket>() => 0x73,
        // _ if type_id == TypeId::of::<StartConfigurationPacket>() => 0x74,
        // _ if type_id == TypeId::of::<StopSoundPacket>() => 0x75,
        // _ if type_id == TypeId::of::<StoreCookiePacket>() => 0x76,
        _ if type_id == TypeId::of::<SystemChatMessagePacket>() => 0x77,
        _ if type_id == TypeId::of::<SetTablistHeaderFooterPacket>() => 0x78,
        // _ if type_id == TypeId::of::<TagQueryPacket>() => 0x79,
        // _ if type_id == TypeId::of::<TakeItemEntityPacket>() => 0x7A,
        // _ if type_id == TypeId::of::<TeleportEntityPacket>() => 0x7B,
        // _ if type_id == TypeId::of::<TestInstanceBlockStatusPacket>() => 0x7C,
        // _ if type_id == TypeId::of::<TickingStatePacket>() => 0x7D,
        // _ if type_id == TypeId::of::<TickingStepPacket>() => 0x7E,
        // _ if type_id == TypeId::of::<TransferPacket>() => 0x7F,
        // _ if type_id == TypeId::of::<UpdateAdvancementsPacket>() => 0x80,
        // _ if type_id == TypeId::of::<UpdateAttributesPacket>() => 0x81,
        // _ if type_id == TypeId::of::<UpdateMobEffectPacket>() => 0x82,
        // _ if type_id == TypeId::of::<UpdateRecipesPacket>() => 0x83,
        // _ if type_id == TypeId::of::<UpdateTagsPacket>() => 0x84,
        // _ if type_id == TypeId::of::<ProjectilePowerPacket>() => 0x85,
        // _ if type_id == TypeId::of::<CustomReportDetailsPacket>() => 0x86,
        // _ if type_id == TypeId::of::<ServerLinksPacket>() => 0x87,
        // _ if type_id == TypeId::of::<WaypointPacket>() => 0x88,
        // _ if type_id == TypeId::of::<ClearDialogPacket>() => 0x89,
        // _ if type_id == TypeId::of::<ShowDialogPacket>() => 0x8A,
        _ => return None,
    })
}
