use crate::{
    protocol::{
        decode::{Decode, DecodeError, PacketRead},
        packet::{ClientPacket, Packet},
    },
    util::BlockPosition,
    world::BlockFace,
};

#[derive(Debug, Clone)]
pub struct PlayerActionPacket {
    pub status: PlayerDiggingState,
    pub position: BlockPosition,
    pub face: BlockFace,
    pub sequence: i32,
}

impl Packet for PlayerActionPacket {}
impl ClientPacket for PlayerActionPacket {}

impl Decode for PlayerActionPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            status:   PlayerDiggingState::try_from(r.read_varint()?).unwrap(),
            position: r.read_position()?,
            face:     BlockFace::try_from(r.read_u8()? as i32).unwrap(),
            sequence: r.read_varint()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlayerDiggingState {
    StartDigging,
    CancelledDigging,
    FinishedDigging,
    DropItemStack,
    DropItem,
    ItemUpdated,
    SwapItemInHand,
}

impl TryFrom<i32> for PlayerDiggingState {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::StartDigging,
            1 => Self::CancelledDigging,
            2 => Self::FinishedDigging,
            3 => Self::DropItemStack,
            4 => Self::DropItem,
            5 => Self::ItemUpdated,
            6 => Self::SwapItemInHand,
            _ => return Err(()),
        })
    }
}

impl Into<i32> for PlayerDiggingState {
    fn into(self) -> i32 {
        match self {
            Self::StartDigging => 0,
            Self::CancelledDigging => 1,
            Self::FinishedDigging => 2,
            Self::DropItemStack => 3,
            Self::DropItem => 4,
            Self::ItemUpdated => 5,
            Self::SwapItemInHand => 6,
        }
    }
}
