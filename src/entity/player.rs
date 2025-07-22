use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::sync::Mutex;

use crate::{
    network::client::ClientConnection,
    protocol::{encode::Encode, packet::KeepAlivePacket},
    Tickable,
};

#[derive(Debug)]
pub struct Player {
    connection: Arc<ClientConnection>,
    last_keep_alive: Mutex<Instant>,
}

impl Player {
    pub fn new(connection: Arc<ClientConnection>) -> Self {
        Self {
            connection,
            last_keep_alive: Mutex::new(Instant::now()),
        }
    }

    pub fn addr(&self) -> std::net::SocketAddr {
        self.connection.addr()
    }

    pub async fn send_packet<P>(&self, packet_id: i32, packet: P)
    where
        P: Encode + std::fmt::Debug,
    {
        self.connection.send_packet(packet_id, packet).await;
    }
}

impl Tickable for Player {
    async fn tick(self: &Arc<Self>) {
        let mut last_keep_alive = self.last_keep_alive.lock().await;
        if last_keep_alive.elapsed() > Duration::from_secs(20) {
            self.send_packet(
                0x26,
                KeepAlivePacket {
                    keep_alive_id: std::time::UNIX_EPOCH.elapsed().unwrap().as_millis() as i64,
                },
            )
            .await;

            *last_keep_alive = Instant::now();
        }
    }
}
