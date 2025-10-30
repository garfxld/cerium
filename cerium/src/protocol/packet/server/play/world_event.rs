use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    util::BlockPosition,
};

#[derive(Debug, Clone)]
pub struct WorldEventPacket {
    pub event: i32,
    pub position: BlockPosition,
    pub data: i32,
    pub disable_relative_volume: bool,
}

impl Packet for WorldEventPacket {}
impl ServerPacket for WorldEventPacket {}

impl Encode for WorldEventPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_i32(this.event)?;
        w.write_position(&this.position)?;
        w.write_i32(this.data)?;
        w.write_bool(this.disable_relative_volume)?;
        Ok(())
    }
}
