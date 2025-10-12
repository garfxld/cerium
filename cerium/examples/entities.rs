use cerium::Server;
use cerium::entity::{Entity, EntityType};
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

    let entity = Entity::new(EntityType::AcaciaBoat);
    entity.set_position((8, 71, 8));

    world.spawn_entity(entity.clone());

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            println!("PlayerConfigEvent ({})", event.get_player().name());
            event.set_world(world.clone());
            event.set_position((0.5, 75.0, 0.5));

            // Uuugly!
            let player = event.get_player();
            tokio::spawn({
                let entity = entity.clone();
                let player = player.clone();
                async move {
                    entity.show_for(player).await;
                }
            });
        })
        .await;

    server.bind("127.0.0.1:25565").await.unwrap();
}
