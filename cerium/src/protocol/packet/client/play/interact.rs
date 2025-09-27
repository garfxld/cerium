use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("interact", 0x19)]
pub struct InteractPacket {
    entity_id: i32,
    r#type: i32,
    target_x: Option<f32>,
    target_y: Option<f32>,
    target_z: Option<f32>,
    hand: Option<i32>,
    sneak_key_pressed: bool,
}

impl ClientPacket for InteractPacket {}

impl Decode for InteractPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let entity_id = r.read_varint()?;
        let r#type = r.read_varint()?;

        let (target_x, target_y, target_z) = if r#type == 2 {
            (
                Some(r.read_f32()?),
                Some(r.read_f32()?),
                Some(r.read_f32()?),
            )
        } else {
            (None, None, None)
        };

        let hand = if r#type == 0 || r#type == 2 {
            Some(r.read_varint()?)
        } else {
            None
        };

        Ok(Self {
            entity_id,
            r#type,
            target_x,
            target_y,
            target_z,
            hand,
            sneak_key_pressed: r.read_bool()?,
        })
    }
}

impl Encode for InteractPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        todo!()
    }
}
