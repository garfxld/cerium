use crate::{protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
}, util::{BlockPosition, Identifier}};

#[derive(Debug, Clone)]
pub struct LoginPacket {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_names: Vec<Identifier>,
    pub max_players: i32,
    pub view_distance: i32,
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: i32,
    pub dimension_name: Identifier,
    pub hashed_seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<DeathLocation>,
    pub portal_cooldown: i32,
    pub sea_level: i32,
    pub enforces_secure_chat: bool,
}

impl Packet for LoginPacket {}
impl ServerPacket for LoginPacket {}

impl Encode for LoginPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_i32(this.entity_id)?;
        w.write_bool(this.is_hardcore)?;
        w.write_array(&this.dimension_names, |w, v| w.write_identifier(v))?;
        w.write_varint(this.max_players)?;
        w.write_varint(this.view_distance)?;
        w.write_varint(this.simulation_distance)?;
        w.write_bool(this.reduced_debug_info)?;
        w.write_bool(this.enable_respawn_screen)?;
        w.write_bool(this.do_limited_crafting)?;
        w.write_varint(this.dimension_type)?;
        w.write_identifier(&this.dimension_name)?;
        w.write_i64(this.hashed_seed)?;
        w.write_u8(this.game_mode)?;
        w.write_u8(this.previous_game_mode as u8)?;
        w.write_bool(this.is_debug)?;
        w.write_bool(this.is_flat)?;
        w.write_option(&this.death_location, DeathLocation::encode)?;
        w.write_varint(this.portal_cooldown)?;
        w.write_varint(this.sea_level)?;
        w.write_bool(this.enforces_secure_chat)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DeathLocation {
    dimension_name: String,
    location: BlockPosition,
}

impl Encode for DeathLocation {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_string(&this.dimension_name)?;
        w.write_position(&this.location)?;
        Ok(())
    }
}
