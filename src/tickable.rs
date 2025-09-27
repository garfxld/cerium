use std::{
    future::Future,
    sync::Arc,
    time::{Duration, Instant},
};

use crate::Server;

pub trait Tickable {
    fn tick(self: &Arc<Self>) -> impl Future<Output = ()> + Send;
}

pub struct Ticker {
    server: Arc<Server>,
    last_tick: Instant,
}

impl Ticker {
    pub fn new(server: Arc<Server>) -> Self {
        Self {
            server,
            last_tick: Instant::now(),
        }
    }

    pub async fn tick(&mut self) {
        if self.last_tick.elapsed() > Duration::from_millis(20) {
            return;
        }

        let server = Arc::clone(&self.server);

        for player in &*server.players.lock().await {
            player.tick().await;
        }

        self.last_tick = Instant::now();
    }
}
