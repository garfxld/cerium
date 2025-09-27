use cerium::Server;
use cerium::event::player::{PlayerConfigEvent, PlayerEvent as _};
use cerium::registry::DimensionType;
use cerium::world::{Block, World};

#[tokio::main]
async fn main() {
    let server = Server::new();

    let world = World::new(&DimensionType::OVERWORLD);

    for bz in 0..16 {
        for bx in 0..16 {
            world.set_block(bx, 70, bz, Block::GrassBlock);
        }
    }

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            println!("PlayerConfigEvent ({})", event.get_player().name());
            event.set_world(world.clone());
            event.set_position((0.5, 75.0, 0.5));
        })
        .await;

    server.bind("127.0.0.1:25565").await.unwrap();
}
