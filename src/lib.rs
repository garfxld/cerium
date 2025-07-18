use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tokio::{net::TcpListener, sync::Mutex};

use crate::network::{auth::KeyStore, client::ClientConnection};

mod network;
mod protocol;
mod registry;
mod world;

mod identifier;

pub struct Server {
    running: AtomicBool,
}

impl Server {
    pub fn new() -> Self {
        Self {
            running: AtomicBool::new(true),
        }
    }

    pub async fn bind(self, addr: &'static str) {
        let listener = TcpListener::bind(addr).await.unwrap();

        let key_store = Arc::new(KeyStore::new());

        while self.running.load(Ordering::Relaxed) {
            let (stream, _) = listener.accept().await.unwrap();

            let connection = Arc::new(Mutex::new(ClientConnection::new(
                Arc::new(stream),
                Arc::clone(&key_store),
            )));

            // todo: configure socket (e.g. nodelay, ...)

            let read_handle = tokio::spawn({
                let connection = Arc::clone(&connection);
                async move {
                    connection.lock().await.read_loop().await;
                }
            });

            let write_handle = tokio::spawn({
                let _connection = Arc::clone(&connection);
                async move {
                    // connection.lock().await.write_loop().await;
                }
            });

            tokio::try_join!(read_handle, write_handle).unwrap();
        }
    }
}
