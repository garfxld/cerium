use macros::packet;

use crate::protocol::{
    buffer::ByteBuffer,
    encode::{Encode, EncodeError},
};

#[derive(Debug)]
#[packet("login")]
pub struct LoginPacket {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_names: Vec<String>,
    pub max_players: i32,
    pub view_distance: i32,
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: i32,
    pub dimension_name: String,
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

impl Encode for LoginPacket {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_i32(this.entity_id)?;
        buffer.write_bool(this.is_hardcore)?;

        buffer.write_array(this.dimension_names, |buffer, value| {
            buffer.write_string(value)
        })?;

        buffer.write_varint(this.max_players)?;
        buffer.write_varint(this.view_distance)?;
        buffer.write_varint(this.simulation_distance)?;
        buffer.write_bool(this.reduced_debug_info)?;
        buffer.write_bool(this.enable_respawn_screen)?;
        buffer.write_bool(this.do_limited_crafting)?;
        buffer.write_varint(this.dimension_type)?;
        buffer.write_string(this.dimension_name)?;
        buffer.write_i64(this.hashed_seed)?;
        buffer.write_u8(this.game_mode)?;
        buffer.write_u8(this.previous_game_mode as u8)?;
        buffer.write_bool(this.is_debug)?;
        buffer.write_bool(this.is_flat)?;

        buffer.write_optional(this.death_location, |buffer, value| {
            DeathLocation::encode(buffer, value)
        })?;

        buffer.write_varint(this.portal_cooldown)?;
        buffer.write_varint(this.sea_level)?;
        buffer.write_bool(this.enforces_secure_chat)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct DeathLocation {
    dimension_name: String,
    location: i64, // Position
}

impl Encode for DeathLocation {
    fn encode(buffer: &mut ByteBuffer, this: Self) -> Result<(), EncodeError> {
        buffer.write_string(this.dimension_name)?;
        buffer.write_i64(this.location)?;
        Ok(())
    }
}
