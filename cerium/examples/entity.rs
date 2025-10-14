use cerium::Server;
use cerium::entity::{Entity, EntityType};
use cerium::event::player::{PlayerConfigEvent, PlayerEvent as _, PlayerSpawnEvent};
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

    let entity = Entity::new(EntityType::Zoglin);
    entity.set_position((8, 71, 8));

    world.spawn_entity(entity.clone());

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            println!("PlayerConfigEvent ({})", event.get_player().name());
            event.set_world(world.clone());
            event.set_position((0.5, 75.0, 0.5));
        })
        .subscribe(move |event: &mut PlayerSpawnEvent| {
            let player = event.get_player();
            entity.show_for(player.clone());
        });

    server.bind("127.0.0.1:25565").unwrap();
}
