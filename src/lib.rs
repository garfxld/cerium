use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::{
    net::TcpListener,
    sync::{Mutex, RwLock},
};

use crate::{
    entity::player::Player,
    network::{auth::KeyStore, client::ClientConnection},
};

mod entity;
mod network;
mod protocol;
mod registry;
mod world;

mod identifier;
mod tickable;

pub use tickable::{Tickable, Ticker};

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("io error")]
    IoError(std::io::Error),
    #[error("unknown error")]
    Unknown,
}

#[derive(Debug)]
pub struct Server {
    running: AtomicBool,
    connections: Arc<RwLock<HashMap<SocketAddr, Arc<ClientConnection>>>>,
    pub players: Arc<Mutex<Vec<Arc<Player>>>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            running: AtomicBool::new(true),
            connections: Arc::new(RwLock::new(HashMap::new())),
            players: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn bind<T>(self, addr: T) -> Result<(), ServerError>
    where
        T: ToString,
    {
        #[cfg(debug_assertions)]
        env_logger::try_init().unwrap();

        let address = addr.to_string();
        let listener = TcpListener::bind(address)
            .await
            .map_err(|e| ServerError::IoError(e))?;

        log::debug!(
            "Listening on {}",
            listener.local_addr().map_err(|e| ServerError::IoError(e))?
        );

        let key_store = Arc::new(KeyStore::new());

        let this = Arc::new(self);

        // Tick Task

        tokio::spawn({
            let this = this.clone();
            async move {
                let ticker = Ticker::new(this.clone());
                while this.running.load(Ordering::Relaxed) {
                    ticker.tick().await;
                    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
                }
            }
        });

        while this.running.load(Ordering::Relaxed) {
            let (stream, addr) = listener.accept().await.unwrap();

            let connection = Arc::new(ClientConnection::new(
                stream,
                addr,
                this.clone(),
                key_store.clone(),
            ));

            this.connections
                .write()
                .await
                .insert(addr, connection.clone());

            // todo: configure socket (e.g. nodelay, ...)

            tokio::spawn({
                let connection = connection.clone();
                let connections = this.connections.clone();
                let players = this.players.clone();

                async move {
                    connection.read_loop().await;

                    // todo: move to connection close
                    connections.write().await.remove(&addr);
                    players.lock().await.retain(|p| p.addr() != addr);
                }
            });
        }

        Ok(())
    }
}
