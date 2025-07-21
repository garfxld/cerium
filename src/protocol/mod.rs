pub mod packet;

pub mod buffer;
pub mod decode;
pub mod encode;
pub mod types;

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
    #[allow(unused)] // todo: remove
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
