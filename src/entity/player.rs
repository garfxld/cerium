use std::{
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use cerium_protocol::{encode::Encode, packet::KeepAlivePacket};
use cerium_util::auth::GameProfile;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{network::client::ClientConnection, Tickable};

pub struct Player {
    connection: Arc<ClientConnection>,
    game_profile: GameProfile,
    last_keep_alive: Mutex<Instant>,
}

impl Player {
    pub async fn new(connection: Arc<ClientConnection>) -> Self {
        let game_profile = connection.game_profile.lock().await.clone().unwrap();

        Self {
            connection,
            game_profile,
            last_keep_alive: Mutex::new(Instant::now()),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.connection.addr()
    }

    pub fn name(&self) -> String {
        self.game_profile.name.to_owned()
    }

    pub fn uuid(&self) -> Uuid {
        self.game_profile.uuid
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
