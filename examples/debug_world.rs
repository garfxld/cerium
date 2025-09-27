use cerium::event::player::{PlayerConfigEvent, PlayerEvent as _};

use cerium::Server;
use cerium::registry::DimensionType;
use cerium::util::Position;
use cerium::world::World;
use cerium_registry::BlockState;

#[tokio::main]
async fn main() {
    let server = Server::new();

    let world = World::new(&DimensionType::OVERWORLD);

    for (ix, pos) in (0..27946).enumerate() {
        let bz = (pos / 168) + 1;
        let bx = (pos % 168) + 1;

        let block = BlockState::from_id(ix as i32).unwrap();
        world.set_block((bz * 2) - 1, 70, (bx * 2) - 1, block);
    }

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            println!("PlayerConfigEvent ({})", event.get_player().name());

            event.set_world(world.clone());
            event.set_position(Position::new(0.5, 71., 0.5, 0., 0.));
        })
        .await;

    server.bind("127.0.0.1:25565").await.unwrap();
}
