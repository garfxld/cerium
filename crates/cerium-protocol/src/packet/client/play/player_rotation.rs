use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("move_player_rot", 0x1F)]
pub struct PlayerRotationPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
}

impl ClientPacket for PlayerRotationPacket {}

impl Decode for PlayerRotationPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            yaw: r.read_f32()?,
            pitch: r.read_f32()?,
            flags: r.read_u8()?,
        })
    }
}

impl Encode for PlayerRotationPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_f32(this.yaw)?;
        w.write_f32(this.pitch)?;
        w.write_u8(this.flags)?;
        Ok(())
    }
}
