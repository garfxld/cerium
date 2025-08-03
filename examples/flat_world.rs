use cerium::event::player::{PlayerConfigEvent, PlayerEvent as _};

use cerium::Server;
use cerium::registry::DimensionType;
use cerium::world::World;

#[tokio::main]
async fn main() {
    let server = Server::new();

    let mut world = World::new(&DimensionType::OVERWORLD);

    for cx in -16..40 {
        for cz in -16..40 {
            world.load_chunk(cx, cz);
        }
    }

    let mut idx = 0;
    for bz in 1..168 {
        for bx in 1..168 {
            world.set_block((bx * 2) - 1, 70, (bz * 2) - 1, idx);
            idx += 1;
        }
    }

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            println!("PlayerConfigEvent ({})", event.get_player().name());
            event.set_world(world.clone());
        })
        .await;

    server.bind("127.0.0.1:25565").await.unwrap();
}
