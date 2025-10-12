use crate::{
    inventory::Slot,
    protocol::{
        decode::{Decode, DecodeError, PacketRead},
        packet::{ClientPacket, Packet},
    },
};

#[derive(Debug, Clone)]
pub struct SetCreativeModeSlotPacket {
    pub slot: i16,
    pub clicked_item: Slot,
}

impl Packet for SetCreativeModeSlotPacket {}
impl ClientPacket for SetCreativeModeSlotPacket {}

impl Decode for SetCreativeModeSlotPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            slot:         r.read_i16()?,
            clicked_item: Slot::decode(r)?,
        })
    }
}
