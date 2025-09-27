use rustc_hash::FxHashMap;
use std::{
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
    auth::KeyStore, entity::Player, event::Events, network::client::ClientConnection,
    tickable::Ticker,
};

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("io error")]
    IoError(std::io::Error),
    #[error("unknown error")]
    Unknown,
}

pub struct Server {
    closed: AtomicBool,
    connections: Arc<Mutex<FxHashMap<SocketAddr, Arc<ClientConnection>>>>,
    pub(crate) players: Arc<Mutex<Vec<Arc<Player>>>>,
    key_store: Arc<KeyStore>,
    events: Events,
}

impl Server {
    pub fn new() -> Self {
        Self {
            closed: AtomicBool::new(false),
            connections: Arc::new(Mutex::new(FxHashMap::default())),
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
                while !this.closed() {
                    ticker.tick().await;
                }
            }
        });

        while !this.closed() {
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

    pub fn closed(&self) -> bool {
        self.closed.load(Ordering::Relaxed)
    }

    pub fn close(&self) {
        self.closed.store(true, Ordering::Release);
    }

    pub fn key_store(&self) -> Arc<KeyStore> {
        self.key_store.clone()
    }

    pub fn events(&self) -> &Events {
        &self.events
    }
}
