use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

#[derive(Debug, Clone)]
pub struct DisplayObjectivePacket {
    pub position: i32,
    pub score_name: String,
}

impl Packet for DisplayObjectivePacket {}
impl ServerPacket for DisplayObjectivePacket {}

impl Encode for DisplayObjectivePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.position)?;
        w.write_string(&this.score_name)?;
        Ok(())
    }
}
