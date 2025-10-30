use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    util::BlockPosition,
};

#[derive(Debug, Clone)]
pub struct SetBlockDestroyStagePacket {
    pub entitiy_id: i32,
    pub location: BlockPosition,
    pub destroy_stage: u8,
}

impl Packet for SetBlockDestroyStagePacket {}
impl ServerPacket for SetBlockDestroyStagePacket {}

impl Encode for SetBlockDestroyStagePacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.entitiy_id)?;
        w.write_position(&this.location)?;
        w.write_u8(this.destroy_stage)?;
        Ok(())
    }
}
