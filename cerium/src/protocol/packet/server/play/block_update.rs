use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    util::BlockPosition,
};

#[derive(Debug, Clone)]
pub struct BlockUpdatePacket {
    pub position: BlockPosition,
    pub block_id: i32,
}

impl Packet for BlockUpdatePacket {}
impl ServerPacket for BlockUpdatePacket {}

impl Encode for BlockUpdatePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_position(&this.position)?;
        w.write_varint(this.block_id)?;
        Ok(())
    }
}
