use cerium_protocol_macros::packet;

#[derive(Debug)]
#[packet("container_click")]
pub struct ClickContainerPacket {
    pub window_id: i32,
    pub state_id: i32,
    pub slot: i16,
    pub button: i8,
    pub mode: i32,
    pub changed_slots: Vec<ChangedSlot>,
    pub carried_item: ChangedSlot,
}

#[derive(Debug)]
pub struct ChangedSlot {
    pub slot_number: i16,
    pub slot_data: i32,
}
