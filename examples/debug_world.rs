use cerium::event::player::{PlayerConfigEvent, PlayerEvent as _};

use cerium::Server;
use cerium::registry::DimensionType;
use cerium::util::Position;
use cerium::world::World;

#[tokio::main]
async fn main() {
    let server = Server::new();

    let world = World::new(&DimensionType::OVERWORLD);

    let mut idx = 0;
    'outer: for bz in 1..169 {
        for bx in 1..169 {
            world.set_block((bz * 2) - 1, 70, (bx * 2) - 1, idx).await;
            idx += 1;
            if idx >= 27946 {
                break 'outer;
            }
        }
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
