use crate::{
    inventory::Slot,
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
};

#[derive(Debug, Clone)]
pub struct SetContainerContentPacket {
    pub window_id: i32,
    pub state_id: i32,
    pub slot_data: Vec<Slot>,
    pub carried_item: Slot,
}

impl Packet for SetContainerContentPacket {}
impl ServerPacket for SetContainerContentPacket {}

impl Encode for SetContainerContentPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.window_id)?;
        w.write_varint(this.state_id)?;
        w.write_array(&this.slot_data, |w, v| Slot::encode(w, v))?;
        Slot::encode(w, &this.carried_item)?;
        Ok(())
    }
}
