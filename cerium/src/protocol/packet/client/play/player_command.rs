use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct PlayerCommandPacket {
    pub entity_id: i32,
    pub action_id: PlayerCommand,
    pub jump_boost: i32,
}

impl Packet for PlayerCommandPacket {}
impl ClientPacket for PlayerCommandPacket {}

impl Decode for PlayerCommandPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            entity_id: r.read_varint()?,
            action_id: PlayerCommand::try_from(r.read_varint()?).unwrap(),
            jump_boost: r.read_varint()?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum PlayerCommand {
    LeaveBed,
    StartSprinting,
    StopSprinting,
    StartJumpWithHorse,
    StopJumpWithHorse,
    OpenVehicleInventory,
    StartFlyingWithElytra,
}

impl TryFrom<i32> for PlayerCommand {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let this = match value {
            0 => Self::LeaveBed,
            1 => Self::StartSprinting,
            2 => Self::StopSprinting,
            3 => Self::StartJumpWithHorse,
            4 => Self::StopJumpWithHorse,
            5 => Self::OpenVehicleInventory,
            6 => Self::StartFlyingWithElytra,
            _ => return Err(()),
        };
        Ok(this)
    }
}
