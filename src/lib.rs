use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use tokio::{
    net::{TcpListener, ToSocketAddrs},
    sync::Mutex,
};

use crate::{
    entity::player::Player,
    event::events::Events,
    network::{auth::KeyStore, client::ClientConnection},
};

pub use cerium_protocol as protocol;
pub use cerium_registry as registry;
pub use cerium_util as util;
pub use cerium_world as world;

mod entity;
mod network;

pub mod event;
mod tickable;

pub use tickable::{Tickable, Ticker};

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("io error")]
    IoError(std::io::Error),
    #[error("unknown error")]
    Unknown,
}

pub struct Server {
    stopped: AtomicBool,
    connections: Arc<Mutex<HashMap<SocketAddr, Arc<ClientConnection>>>>,
    players: Arc<Mutex<Vec<Arc<Player>>>>,
    key_store: Arc<KeyStore>,
    events: Events,
}

impl Server {
    pub fn new() -> Self {
        Self {
            stopped: AtomicBool::new(false),
            connections: Arc::new(Mutex::new(HashMap::new())),
            players: Arc::new(Mutex::new(Vec::new())),
            key_store: Arc::new(KeyStore::new()),
            events: Events::new(),
        }
    }

    pub async fn bind<A>(self, addr: A) -> Result<(), ServerError>
    where
        A: ToSocketAddrs,
    {
        #[cfg(debug_assertions)]
        env_logger::try_init().unwrap();

        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| ServerError::IoError(e))?;

        log::debug!(
            "Listening on {}",
            listener.local_addr().map_err(|e| ServerError::IoError(e))?
        );

        let this = Arc::new(self);

        // Tick Task

        tokio::spawn({
            let this = this.clone();
            let mut ticker = Ticker::new(this.clone());

            async move {
                while !this.stopped() {
                    ticker.tick().await;
                }
            }
        });

        while !this.stopped() {
            let (stream, addr) = listener.accept().await.unwrap();

            let conn = Arc::new(ClientConnection::new(addr, stream, this.clone()));

            this.connections.lock().await.insert(addr, conn.clone());

            // todo: configure socket (e.g. nodelay, ...)

            tokio::spawn({
                let connection = conn.clone();
                let connections = this.connections.clone();
                let players = this.players.clone();

                async move {
                    connection.read_loop().await;

                    connections.lock().await.remove(&addr);
                    players.lock().await.retain(|p| p.addr() != addr);
                }
            });
        }

        Ok(())
    }

    pub fn stopped(&self) -> bool {
        self.stopped.load(Ordering::Relaxed)
    }

    pub fn stop(&self) {
        self.stopped.store(true, Ordering::Release);
    }

    pub fn key_store(&self) -> Arc<KeyStore> {
        self.key_store.clone()
    }

    pub fn events(&self) -> &Events {
        &self.events
    }
}
