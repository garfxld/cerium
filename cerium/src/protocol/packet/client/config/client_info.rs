use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct ClientInfoPacket {
    pub locale: String, // 16
    pub view_distance: u8,
    pub chat_mode: i32,
    pub displayed_skin_parts: u8,
    pub main_hand: i32,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
    pub particle_status: i32,
}

impl Packet for ClientInfoPacket {}
impl ClientPacket for ClientInfoPacket {}

impl Decode for ClientInfoPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            locale:                r.read_string()?,
            view_distance:         r.read_u8()?,
            chat_mode:             r.read_varint()?,
            displayed_skin_parts:  r.read_u8()?,
            main_hand:             r.read_varint()?,
            enable_text_filtering: r.read_bool()?,
            allow_server_listings: r.read_bool()?,
            particle_status:       r.read_varint()?,
        })
    }
}
