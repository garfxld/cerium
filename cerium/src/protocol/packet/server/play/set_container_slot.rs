use crate::{
    inventory::Slot,
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
};

#[derive(Debug, Clone)]
pub struct SetContainerSlotPacket {
    pub window_id: i32,
    pub state_id: i32,
    pub slot: i16,
    pub slot_data: Slot,
}

impl Packet for SetContainerSlotPacket {}
impl ServerPacket for SetContainerSlotPacket {}

impl Encode for SetContainerSlotPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.window_id)?;
        w.write_varint(this.state_id)?;
        w.write_i16(this.slot)?;
        Slot::encode(w, &this.slot_data)?;
        Ok(())
    }
}
