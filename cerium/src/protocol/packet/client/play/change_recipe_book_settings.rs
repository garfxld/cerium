use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct ChangeRecipeBookSettingsPacket {
    pub book_id: i32,
    pub book_open: bool,
    pub filter_active: bool,
}

impl Packet for ChangeRecipeBookSettingsPacket {}
impl ClientPacket for ChangeRecipeBookSettingsPacket {}

impl Decode for ChangeRecipeBookSettingsPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            book_id: r.read_varint()?,
            book_open: r.read_bool()?,
            filter_active: r.read_bool()?,
        })
    }
}
