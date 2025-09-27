use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("client_information", 0x0D)]
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

impl Encode for ClientInfoPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_string(this.locale)?;
        w.write_u8(this.view_distance)?;
        w.write_varint(this.chat_mode)?;
        w.write_u8(this.displayed_skin_parts)?;
        w.write_varint(this.main_hand)?;
        w.write_bool(this.enable_text_filtering)?;
        w.write_bool(this.allow_server_listings)?;
        w.write_varint(this.particle_status)?;
        Ok(())
    }
}
