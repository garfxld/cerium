use crate::protocol::{
    DataType,
    decode::{DecodeError, PacketRead},
    encode::{EncodeError, PacketWrite},
};

#[derive(Debug)]
pub struct Equippable {}

impl DataType for Equippable {
    fn decode<R: PacketRead>(_r: &mut R) -> Result<Self, DecodeError> {
        todo!()
    }

    fn encode<W: PacketWrite>(_w: &mut W, _this: &Self) -> Result<(), EncodeError> {
        todo!()
    }
}
