use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("login", 0x2B)]
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

impl Decode for LoginPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            entity_id:             r.read_i32()?,
            is_hardcore:           r.read_bool()?,
            dimension_names:       r.read_array(|r| r.read_string())?,
            max_players:           r.read_varint()?,
            view_distance:         r.read_varint()?,
            simulation_distance:   r.read_varint()?,
            reduced_debug_info:    r.read_bool()?,
            enable_respawn_screen: r.read_bool()?,
            do_limited_crafting:   r.read_bool()?,
            dimension_type:        r.read_varint()?,
            dimension_name:        r.read_string()?,
            hashed_seed:           r.read_i64()?,
            game_mode:             r.read_u8()?,
            previous_game_mode:    r.read_u8()? as i8,
            is_debug:              r.read_bool()?,
            is_flat:               r.read_bool()?,
            death_location:        r.read_option(DeathLocation::decode)?,
            portal_cooldown:       r.read_varint()?,
            sea_level:             r.read_varint()?,
            enforces_secure_chat:  r.read_bool()?,
        })
    }
}

impl Encode for LoginPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_i32(this.entity_id)?;
        w.write_bool(this.is_hardcore)?;
        w.write_array(this.dimension_names, |w, v| w.write_string(v))?;
        w.write_varint(this.max_players)?;
        w.write_varint(this.view_distance)?;
        w.write_varint(this.simulation_distance)?;
        w.write_bool(this.reduced_debug_info)?;
        w.write_bool(this.enable_respawn_screen)?;
        w.write_bool(this.do_limited_crafting)?;
        w.write_varint(this.dimension_type)?;
        w.write_string(this.dimension_name)?;
        w.write_i64(this.hashed_seed)?;
        w.write_u8(this.game_mode)?;
        w.write_u8(this.previous_game_mode as u8)?;
        w.write_bool(this.is_debug)?;
        w.write_bool(this.is_flat)?;
        w.write_option(this.death_location, DeathLocation::encode)?;
        w.write_varint(this.portal_cooldown)?;
        w.write_varint(this.sea_level)?;
        w.write_bool(this.enforces_secure_chat)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DeathLocation {
    dimension_name: String,
    location: i64, // Position
}

impl Decode for DeathLocation {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            dimension_name: r.read_string()?,
            location:       r.read_i64()?,
        })
    }
}

impl Encode for DeathLocation {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_string(this.dimension_name)?;
        w.write_i64(this.location)?;
        Ok(())
    }
}
