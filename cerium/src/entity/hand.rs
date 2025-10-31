use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    encode::{Encode, EncodeError, PacketWrite},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hand {
    MainHand,
    OffHand,
}

impl TryFrom<i32> for Hand {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::MainHand),
            1 => Ok(Self::OffHand),
            _ => Err(()),
        }
    }
}

impl Decode for Hand {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Hand::try_from(r.read_varint()?).map_err(|_| DecodeError::Decode("Invalid Hand"))
    }
}

impl Encode for Hand {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)?;
        Ok(())
    }
}
