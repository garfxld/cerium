use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
    entity::Player,
    protocol::packet::{DisplayObjectivePacket, UpdateObjectivesPacket},
    text::Component,
    util::Viewable,
};

pub struct Sidebar {
    objective: String,
    title: Mutex<Component>,
    viewers: Mutex<Vec<Arc<Player>>>,
}

impl Sidebar {
    pub fn new(objective: impl Into<String>, title: impl Into<Component>) -> Self {
        Self {
            objective: objective.into(),
            title: Mutex::new(title.into()),
            viewers: Mutex::new(vec![]),
        }
    }

    pub fn title(&self) -> Component {
        self.title.lock().clone()
    }

    pub fn set_title(&self, title: impl Into<Component>) {
        *self.title.lock() = title.into();
        // todo: refresh title
    }
}

impl Viewable for Sidebar {
    fn add_viewer(&self, player: Arc<Player>) {
        self.viewers.lock().push(player.clone());

        player.send_packet(UpdateObjectivesPacket {
            objective_name: self.objective.clone(),
            mode: 0,
            objective_value: Some(self.title()),
            ty: Some(0),
            has_number_format: Some(false),
            number_format: Some(0),
        });
        player.send_packet(DisplayObjectivePacket {
            position: 1,
            score_name: self.objective.clone(),
        });
    }

    fn remove_viewer(&self, player: Arc<Player>) {
        self.viewers.lock().retain(|other| *other != player);
    }

    fn viewers(&self) -> Vec<Arc<Player>> {
        self.viewers.lock().clone()
    }
}
