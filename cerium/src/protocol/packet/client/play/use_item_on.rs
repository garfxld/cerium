use crate::{
    entity::Hand,
    protocol::{
        decode::{Decode, DecodeError, PacketRead},
        packet::{ClientPacket, Packet},
    },
    util::BlockPosition,
    world::BlockFace,
};

#[derive(Debug, Clone)]
pub struct UseItemOnPacket {
    pub hand: Hand,
    pub position: BlockPosition,
    pub face: BlockFace,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub cursor_z: f32,
    pub inside_block: bool,
    pub world_border_hit: bool,
    pub sequence: i32,
}

impl Packet for UseItemOnPacket {}
impl ClientPacket for UseItemOnPacket {}

impl Decode for UseItemOnPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            hand:             Hand::decode(r)?,
            position:         r.read_position()?,
            face:             BlockFace::try_from(r.read_varint()?).unwrap(),
            cursor_x:         r.read_f32()?,
            cursor_y:         r.read_f32()?,
            cursor_z:         r.read_f32()?,
            inside_block:     r.read_bool()?,
            world_border_hit: r.read_bool()?,
            sequence:         r.read_varint()?,
        })
    }
}
