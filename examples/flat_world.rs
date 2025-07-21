use cerium::Server;

#[tokio::main]
pub async fn main() {
    let server = Server::new();
    server.bind("127.0.0.1:25565").await.unwrap();
}
