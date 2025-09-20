use cerium::event::player::{PlayerConfigEvent, PlayerEvent as _};

use cerium::Server;
use cerium::registry::DimensionType;
use cerium::util::Position;
use cerium::world::World;
use cerium_registry::generated::block::Block;

#[tokio::main]
async fn main() {
    let server = Server::new();

    let world = World::new(&DimensionType::OVERWORLD);

    for bz in 0..16 {
        for bx in 0..16 {
            world.set_block(bx, 70, bz, Block::GrassBlock).await;
        }
    }

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            println!("PlayerConfigEvent ({})", event.get_player().name());
            event.set_world(world.clone());
            event.set_position(Position::new(0., 75., 0., 0., 0.));
        })
        .await;

    server.bind("127.0.0.1:25565").await.unwrap();
}
