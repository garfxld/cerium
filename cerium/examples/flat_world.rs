use cerium::Server;
use cerium::event::player::PlayerConfigEvent;
use cerium::registry::DimensionType;
use cerium::world::{Block, World};

fn main() {
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
            event.set_world(world.clone());
            event.set_position((0.5, 75.0, 0.5));
        });

    server.bind("127.0.0.1:25565").unwrap();
}
