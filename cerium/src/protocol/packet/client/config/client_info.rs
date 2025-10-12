use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct ClientInfoPacket {
    locale: String, // 16
    view_distance: u8,
    chat_mode: i32,
    displayed_skin_parts: u8,
    main_hand: i32,
    enable_text_filtering: bool,
    allow_server_listings: bool,
    particle_status: i32,
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
