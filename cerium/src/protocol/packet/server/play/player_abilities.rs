use bitflags::bitflags;

use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct PlayerAbilitiesPacket {
    pub flags: PlayerAbilitiesFlag,
    pub flying_speed: f32,
    pub fov_modifier: f32,
}

impl Packet for PlayerAbilitiesPacket {}
impl ServerPacket for PlayerAbilitiesPacket {}

impl Encode for PlayerAbilitiesPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_u8(this.flags.bits())?;
        w.write_f32(this.flying_speed)?;
        w.write_f32(this.fov_modifier)?;
        Ok(())
    }
}

bitflags! {
    #[derive(Debug, Clone)]
    pub struct PlayerAbilitiesFlag: u8 {
        const INVURNABLE = 0x01;
        const FLYING = 0x02;
        const ALLOW_FLYING = 0x04;
        const CREATIVE_MODE = 0x08;
    }
}
