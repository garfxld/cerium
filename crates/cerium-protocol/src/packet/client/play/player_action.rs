use cerium_protocol_macros::packet;

use crate::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("player_action")]
pub struct PlayerActionPacket {
    pub status: i32,
    pub position: i64,
    pub face: u8,
    pub sequence: i32,
}

impl ClientPacket for PlayerActionPacket {}

impl Decode for PlayerActionPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            status:   r.read_varint()?,
            position: r.read_i64()?,
            face:     r.read_u8()?,
            sequence: r.read_varint()?,
        })
    }
}

impl Encode for PlayerActionPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_varint(this.status)?;
        w.write_i64(this.position)?;
        w.write_u8(this.face)?;
        w.write_varint(this.sequence)?;
        Ok(())
    }
}
