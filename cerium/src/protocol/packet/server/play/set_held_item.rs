use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct SetHeldItemPacket {
    pub slot: i32,
}

impl Packet for SetHeldItemPacket {}
impl ServerPacket for SetHeldItemPacket {}

impl Encode for SetHeldItemPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.slot)?;
        Ok(())
    }
}
