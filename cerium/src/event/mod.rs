mod events;
pub use events::Events;

pub mod inventory;
pub mod player;

pub trait Event: Send {}

pub struct ServerListPingEvent {
    pub(crate) response: String,
}

impl ServerListPingEvent {
    pub fn new(response: String) -> Self {
        Self { response }
    }

    pub fn get_response(&self) -> String {
        self.response.to_owned()
    }

    pub fn set_response(&mut self, response: String) {
        self.response = response;
    }
}

impl Event for ServerListPingEvent {}

pub trait Cancellable: Event {
    fn set_cancelled(&mut self, value: bool);

    fn is_cancelled(&self) -> bool;
}
