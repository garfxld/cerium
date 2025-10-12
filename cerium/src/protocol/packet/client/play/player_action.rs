use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PlayerActionPacket {
    pub status: i32,
    pub position: i64,
    pub face: u8,
    pub sequence: i32,
}

impl Packet for PlayerActionPacket {}
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
