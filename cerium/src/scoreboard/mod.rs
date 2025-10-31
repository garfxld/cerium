use crate::{
    entity::Player,
    protocol::packet::{DisplayObjectivePacket, UpdateObjectivesPacket},
    text::TextComponent,
    util::{Viewable, Viewers},
};

pub mod team;

pub struct Objective {
    name: String,
    display_name: TextComponent,
    display_slot: DisplaySlot,
    viewers: Viewers,
}

impl Objective {
    pub fn new(name: impl Into<String>, display_name: impl Into<TextComponent>) -> Self {
        Self {
            name: name.into(),
            display_name: display_name.into(),
            display_slot: DisplaySlot::Sidebar,
            viewers: Viewers::new(),
        }
    }
}

impl Viewable for Objective {
    fn add_viewer(&self, player: Player) {
        self.viewers.add_viewer(player.clone());

        player.send_packet(&UpdateObjectivesPacket {
            objective_name: self.name.clone(),
            mode: 0,
            objective_value: Some(self.display_name.clone()),
            ty: Some(1),
            has_number_format: Some(false),
            number_format: None,
        });
        player.send_packet(&DisplayObjectivePacket {
            position: self.display_slot as i32,
            score_name: self.name.clone(),
        });
    }

    fn remove_viewer(&self, player: Player) {
        self.viewers.remove_viewer(player);
    }

    fn viewers(&self) -> &Viewers {
        &self.viewers
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DisplaySlot {
    List,
    Sidebar,
    BelowName,
}
