use cerium::Server;
use cerium::entity::GameMode;
use cerium::event::player::{PlayerConfigEvent, PlayerEvent, PlayerSpawnEvent};
use cerium::registry::DimensionType;
use cerium::world::{BlockState, World};

fn main() {
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
            event.set_world(world.clone());
            event.set_position((0.5, 71., 0.5));
        })
        .subscribe(move |event: &mut PlayerSpawnEvent| {
            event.get_player().set_game_mode(GameMode::Creative);
        });

    server.bind("127.0.0.1:25565").unwrap();
}
