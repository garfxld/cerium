use cerium::Server;


#[tokio::main]
pub async fn main() {
    let _server = Server::bind("127.0.0.1:25565").await;
}