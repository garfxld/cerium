use parking_lot::Mutex;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    runtime::{Handle, Runtime},
};

use crate::{
    auth::KeyStore, entity::Player, event::Events, network::client::Connection, tickable::Ticker,
};

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("io error")]
    IoError(std::io::Error),
    #[error("unknown error")]
    Unknown,
}

pub struct Server {
    runtime: tokio::runtime::Runtime,
    handle: tokio::runtime::Handle,
    closed: AtomicBool,
    pub(crate) players: Arc<Mutex<Vec<Arc<Player>>>>,
    key_store: Arc<KeyStore>,
    events: Events,
}

impl Server {
    pub fn new() -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("todo");

        let handle = runtime.handle().clone();

        Self {
            runtime: runtime,
            handle,
            closed: AtomicBool::new(false),
            players: Arc::new(Mutex::new(Vec::new())),
            key_store: Arc::new(KeyStore::new()),
            events: Events::new(),
        }
    }

    pub fn bind<A>(self, addr: A) -> Result<(), ServerError>
    where
        A: ToSocketAddrs,
    {
        let handle = self.handle.clone();
        handle.block_on(self._bind(addr))?;
        Ok(())
    }

    async fn _bind<A>(self, addr: A) -> Result<(), ServerError>
    where
        A: ToSocketAddrs,
    {
        #[cfg(debug_assertions)]
        env_logger::try_init().unwrap();

        let listener = TcpListener::bind(addr)
            .await
            .map_err(ServerError::IoError)?;

        log::debug!(
            "Listening on {}",
            listener.local_addr().map_err(ServerError::IoError)?
        );

        let this = Arc::new(self);

        // Tick Task
        this.handle.spawn({
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

            this.handle.spawn({
                let this = this.clone();
                async move {
                    Connection::accept(addr, stream, this.clone()).await;
                }
            });
        }

        Ok(())
    }

    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    pub fn handle(&self) -> &Handle {
        &self.handle
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
