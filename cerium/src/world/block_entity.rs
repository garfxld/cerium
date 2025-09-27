use simdnbt::owned::Nbt;

#[derive(Debug, Clone)]
pub struct BlockEntity {
    pub packed_xz: u8,
    pub y: i16,
    pub r#type: i32,
    pub data: Nbt,
}
