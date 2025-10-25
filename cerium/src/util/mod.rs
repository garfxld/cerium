mod position;
use std::sync::Arc;

use crate::{
    entity::Player,
    protocol::packet::{Packet, ServerPacket},
};

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
    fn add_viewer(&self, player: Arc<Player>);
    fn remove_viewer(&self, player: Arc<Player>);
    fn viewers(&self) -> Vec<Arc<Player>>;

    fn send_packet_to_viewers<P>(&self, packet: P)
    where
        P: Packet + ServerPacket + 'static,
    {
        for viewer in self.viewers() {
            viewer.send_packet(packet.clone());
        }
    }
}
