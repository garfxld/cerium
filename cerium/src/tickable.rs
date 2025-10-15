use std::{sync::Arc, time::Duration};
use tokio::time::{Interval, interval};

use crate::Server;

pub trait Tickable {
    fn tick(self: &Arc<Self>);
}

pub struct Ticker {
    server: Arc<Server>,
    interval: Interval,
}

impl Ticker {
    pub fn new(server: Arc<Server>) -> Self {
        Self {
            server,
            interval: interval(Duration::from_millis(50)),
        }
    }

    pub async fn tick(&mut self) {
        self.interval.tick().await;

        let server = Arc::clone(&self.server);

        for player in &*server.players.lock() {
            player.tick();
        }
    }
}
