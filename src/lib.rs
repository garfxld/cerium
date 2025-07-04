use std::sync::Arc;

use tokio::net::{TcpListener, TcpStream};

use crate::network::client::ClientConnection;

mod protocol;

mod network;

pub struct Server {}

impl Server {
    pub async fn bind(addr: &'static str) -> Self {
        let listener = TcpListener::bind(addr).await.unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                Self::handle_connection(stream).await.unwrap();
            });
        }

        // Self {  }
    }

    async fn handle_connection(stream: TcpStream) -> anyhow::Result<()> {
        stream.set_nodelay(true)?;

        let stream = Arc::new(stream);
        let mut conn = ClientConnection::new(Arc::clone(&stream));

        conn.read().await?;

        Ok(())
    }
}
