#![allow(unused)] // todo: remove

pub mod packet;

pub mod decoder;
pub mod encoder;
pub mod types;



#[derive(Debug)]
pub enum ProtcolState {
    Handshake,
    Status,
    Login,
    // Transfer,
    Config,
    Play,
}

impl ProtcolState {
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
