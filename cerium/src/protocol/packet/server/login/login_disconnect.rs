use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct LoginDisconnectPacket {
    pub reason: String,
}

impl Packet for LoginDisconnectPacket {}
impl ServerPacket for LoginDisconnectPacket {}

impl Encode for LoginDisconnectPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_string(&this.reason)?;
        Ok(())
    }
}
