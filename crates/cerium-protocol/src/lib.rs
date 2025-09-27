#![feature(associated_type_defaults)]

pub mod packet;

pub mod decode;
pub mod encode;
pub mod types;

mod chunk;

pub mod read;
pub mod write;

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
