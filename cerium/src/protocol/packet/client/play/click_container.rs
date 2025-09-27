use cerium_protocol_macros::packet;

use crate::protocol::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    packet::ClientPacket,
    read::PacketRead,
    write::PacketWrite,
};

#[derive(Debug, Clone)]
#[packet("container_click", 0x11)]
pub struct ClickContainerPacket {
    pub window_id: i32,
    pub state_id: i32,
    pub slot: i16,
    pub button: i8,
    pub mode: i32,
    pub changed_slots: Vec<ChangedSlot>,
    pub carried_item: HashedSlot,
}

impl ClientPacket for ClickContainerPacket {}

impl Decode for ClickContainerPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            window_id:     r.read_varint()?,
            state_id:      r.read_varint()?,
            slot:          r.read_i16()?,
            button:        r.read_i8()?,
            mode:          r.read_varint()?,
            changed_slots: r.read_array(ChangedSlot::decode)?,
            carried_item:  HashedSlot::decode(r)?,
        })
    }
}

impl Encode for ClickContainerPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: Self) -> Result<(), EncodeError> {
        todo!();
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ChangedSlot {
    pub slot_number: i16,
    pub slot_data: HashedSlot,
}

#[derive(Debug, Clone)]
pub struct HashedSlot {
    pub has_item: bool,
    pub item_id: Option<i32>,
}

impl Decode for ChangedSlot {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            slot_number: r.read_i16()?,
            slot_data:   HashedSlot::decode(r)?,
        })
    }
}

impl Decode for HashedSlot {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            has_item: r.read_bool()?,
            item_id:  r.read_option(|r| r.read_varint())?,
        })
    }
}
