use crate::{
    entity::Hand,
    protocol::{
        decode::{Decode, DecodeError, PacketRead},
        encode::{Encode, EncodeError, PacketWrite},
        packet::{ClientPacket, Packet},
    },
};

#[derive(Debug, Clone)]
pub struct InteractPacket {
    pub entity_id: i32,
    pub r#type: InteractType,
    pub target_x: Option<f32>,
    pub target_y: Option<f32>,
    pub target_z: Option<f32>,
    pub hand: Option<Hand>,
    pub sneak_key_pressed: bool,
}

impl Packet for InteractPacket {}
impl ClientPacket for InteractPacket {}

impl Decode for InteractPacket {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let entity_id = r.read_varint()?;
        let r#type = InteractType::decode(r)?;

        let (target_x, target_y, target_z) = if r#type == InteractType::Interact {
            (
                Some(r.read_f32()?),
                Some(r.read_f32()?),
                Some(r.read_f32()?),
            )
        } else {
            (None, None, None)
        };

        Ok(Self {
            entity_id,
            r#type,
            target_x,
            target_y,
            target_z,
            hand: matches!(r#type, InteractType::Interact | InteractType::Attack)
                .then_some(Hand::decode(r)?),
            sneak_key_pressed: r.read_bool()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InteractType {
    Interact,
    Attack,
    InteractAt,
}

impl TryFrom<i32> for InteractType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Interact),
            1 => Ok(Self::Attack),
            2 => Ok(Self::InteractAt),
            _ => Err(()),
        }
    }
}

impl Decode for InteractType {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        InteractType::try_from(r.read_varint()?)
            .map_err(|_| DecodeError::Decode("Invalid InteractType".to_string()))
    }
}

impl Encode for InteractType {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)?;
        Ok(())
    }
}
