use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct StatusResponsePacket {
    pub json_response: String,
}

impl Packet for StatusResponsePacket {}
impl ServerPacket for StatusResponsePacket {}

impl Encode for StatusResponsePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_string(&this.json_response)?;
        Ok(())
    }
}
