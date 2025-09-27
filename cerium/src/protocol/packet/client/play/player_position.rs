use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("move_player_pos", 0x1D)]
pub struct PlayerPositionPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub flags: u8, // 0x01: on ground, 0x02: pushing against wall
}

impl ClientPacket for PlayerPositionPacket {}

impl Decode for PlayerPositionPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            x:      r.read_f64()?,
            feet_y: r.read_f64()?,
            z:      r.read_f64()?,
            flags:  r.read_u8()?,
        })
    }
}

impl Encode for PlayerPositionPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_f64(this.x)?;
        w.write_f64(this.feet_y)?;
        w.write_f64(this.z)?;
        w.write_u8(this.flags)?;
        Ok(())
    }
}
