use std::{future::Future, sync::Arc};

use crate::Server;

pub trait Tickable {
    fn tick(self: &Arc<Self>) -> impl Future<Output = ()> + Send;
}

pub struct Ticker {
    server: Arc<Server>,
}

impl Ticker {
    pub fn new(server: Arc<Server>) -> Self {
        Self { server }
    }

    pub async fn tick(&self) {
        let server = Arc::clone(&self.server);

        for player in &*server.players.lock().await {
            player.tick().await;
        }
    }
}
