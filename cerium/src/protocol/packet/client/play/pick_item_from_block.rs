use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("pick_item_from_block", 0x23)]
pub struct PickItemFromBlockPacket {
    pub position: i64,
    pub include_data: bool,
}

impl ClientPacket for PickItemFromBlockPacket {}

impl Decode for PickItemFromBlockPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            position:     r.read_i64()?,
            include_data: r.read_bool()?,
        })
    }
}

impl Encode for PickItemFromBlockPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        w.write_i64(this.position)?;
        w.write_bool(this.include_data)?;
        Ok(())
    }
}
