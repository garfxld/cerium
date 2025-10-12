use serde::{Deserialize, Serialize};

use crate::protocol::{
    DataType,
    decode::{DecodeError, PacketRead},
    encode::{EncodeError, PacketWrite},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DyeColor {
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}

impl TryFrom<i32> for DyeColor {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DyeColor::White),
            1 => Ok(DyeColor::Orange),
            2 => Ok(DyeColor::Magenta),
            3 => Ok(DyeColor::LightBlue),
            4 => Ok(DyeColor::Yellow),
            5 => Ok(DyeColor::Lime),
            6 => Ok(DyeColor::Pink),
            7 => Ok(DyeColor::Gray),
            8 => Ok(DyeColor::LightGray),
            9 => Ok(DyeColor::Cyan),
            10 => Ok(DyeColor::Purple),
            11 => Ok(DyeColor::Blue),
            12 => Ok(DyeColor::Brown),
            13 => Ok(DyeColor::Green),
            14 => Ok(DyeColor::Red),
            15 => Ok(DyeColor::Black),
            _ => Err(()),
        }
    }
}

impl DataType for DyeColor {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self::try_from(r.read_varint()?).unwrap())
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)
    }
}
