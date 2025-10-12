use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    encode::{Encode, EncodeError, PacketWrite},
};

pub mod packet;

pub mod decode;
pub mod encode;
pub mod types;

mod chunk;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProtocolState {
    Handshake,
    Status,
    Login,
    // Transfer,
    Config,
    Play,
}

impl ProtocolState {
    pub fn from_id(id: i32) -> Self {
        match id {
            0 => Self::Handshake,
            1 => Self::Status,
            2 => Self::Login,
            // 3 => Self::Transfer,
            4 => Self::Config,
            5 => Self::Play,
            _ => panic!("protocol with id {} does not exist!", id),
        }
    }
}

pub trait DataType
where
    Self: Sized,
{
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError>;
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError>;
}

impl<T: DataType> Encode for T {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        <Self as DataType>::encode(w, this)
    }
}

impl<T: DataType> Decode for T {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        <Self as DataType>::decode(r)
    }
}
