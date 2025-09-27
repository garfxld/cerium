use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    read::PacketRead,
    write::PacketWrite,
};
use cerium_protocol_macros::packet;

#[derive(Debug, Clone)]
#[packet("remove_entities", 0x46)]
pub struct RemoveEntitiesPacket {
    pub entity_ids: Vec<i32>,
}

impl Decode for RemoveEntitiesPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            entity_ids: r.read_array(|r| r.read_varint())?,
        })
    }
}

impl Encode for RemoveEntitiesPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_array(this.entity_ids, |w, v| w.write_varint(v))?;
        Ok(())
    }
}
