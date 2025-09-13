use cerium_protocol_macros::packet;

use crate::{
    buffer::ByteBuffer,
    decode::{Decode, DecodeError},
};

#[derive(Debug)]
#[packet("container_click")]
pub struct ClickContainerPacket {
    pub window_id: i32,
    pub state_id: i32,
    pub slot: i16,
    pub button: i8,
    pub mode: i32,
    pub changed_slots: Vec<ChangedSlot>,
    pub carried_item: HashedSlot,
}

#[derive(Debug)]
pub struct ChangedSlot {
    pub slot_number: i16,
    pub slot_data: HashedSlot,
}

#[derive(Debug)]
pub struct HashedSlot {
    pub has_item: bool,
    pub item_id: Option<i32>,
}

impl Decode for ClickContainerPacket {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            window_id: buffer.read_varint()?,
            state_id: buffer.read_varint()?,
            slot: buffer.read_i16()?,
            button: buffer.read_i8()?,
            mode: buffer.read_varint()?,
            changed_slots: buffer.read_list(|buffer| ChangedSlot::decode(buffer))?,
            carried_item: HashedSlot::decode(buffer)?,
        })
    }
}

impl Decode for ChangedSlot {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            slot_number: buffer.read_i16()?,
            slot_data: HashedSlot::decode(buffer)?,
        })
    }
}

impl Decode for HashedSlot {
    fn decode(buffer: &mut ByteBuffer) -> Result<Self, DecodeError> {
        Ok(Self {
            has_item: buffer.read_bool()?,
            item_id: buffer.read_option(|buffer| buffer.read_varint())?,
        })
    }
}
