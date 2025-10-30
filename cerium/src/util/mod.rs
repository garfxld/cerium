use parking_lot::Mutex;

use crate::{
    entity::Player,
    protocol::packet::{Packet, ServerPacket},
};

mod position;
pub use position::*;

mod identifier;
pub use identifier::*;

mod dye_color;
pub use dye_color::*;

mod direction;
pub use direction::*;

mod pose;
pub use pose::*;

pub trait Viewable {
    fn viewers(&self) -> &Viewers;

    fn add_viewer(&self, player: Player) {
        self.viewers().add_viewer(player);
    }

    fn remove_viewer(&self, player: Player) {
        self.viewers().remove_viewer(player);
    }

    fn send_packet_to_viewers<P>(&self, packet: P)
    where
        P: Packet + ServerPacket + 'static,
    {
        for viewer in self.viewers() {
            viewer.send_packet(packet.clone());
        }
    }
}

pub struct Viewers {
    viewers: Mutex<Vec<Player>>,
}

impl Viewers {
    pub fn new() -> Self {
        Self {
            viewers: Mutex::new(vec![]),
        }
    }

    pub fn add_viewer(&self, player: Player) {
        self.viewers.lock().push(player);
    }

    pub fn remove_viewer(&self, player: Player) {
        self.viewers.lock().retain(|other| *other != player);
    }
    pub fn iter(&self) -> Vec<Player> {
        self.viewers.lock().clone()
    }

    pub fn len(&self) -> usize {
        self.viewers.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.viewers.lock().is_empty()
    }
}

impl Clone for Viewers {
    fn clone(&self) -> Self {
        Self {
            viewers: self.viewers.lock().clone().into(),
        }
    }
}

impl IntoIterator for Viewers {
    type Item = Player;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.viewers.into_inner().into_iter()
    }
}

impl<'a> IntoIterator for &'a Viewers {
    type Item = Player;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.viewers.lock().clone().into_iter()
    }
}
