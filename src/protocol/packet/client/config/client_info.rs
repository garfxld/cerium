use bytes::{buf, Buf};

use crate::protocol::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
pub struct ClientInfoPacket {
    locale: String, // 16
    view_distance: u8,
    chat_mode: i32, // VarInt
    displayed_skin_parts: u8,
    main_hand: i32, // VarInt
    enable_text_filtering: bool,
    allow_server_listings: bool,
    particle_status: i32, // VarInt
}

impl Decode for ClientInfoPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<ClientInfoPacket, DecodeError> {
        Ok(Self {
            locale: buffer.read_string()?,
            view_distance: buffer.read_u8()?,
            chat_mode: buffer.read_varint()?,
            displayed_skin_parts: buffer.read_u8()?,
            main_hand: buffer.read_varint()?,
            enable_text_filtering: buffer.read_bool()?,
            allow_server_listings: buffer.read_bool()?,
            particle_status: buffer.read_varint()?,
        })
    }
}
