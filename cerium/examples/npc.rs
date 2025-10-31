use cerium::Server;
use cerium::entity::{Entity, EntityLike as _, EntityType, GameMode};
use cerium::event::player::{PlayerConfigEvent, PlayerEvent as _, PlayerSpawnEvent};
use cerium::protocol::packet::{
    PlayerAction, PlayerEntry, PlayerInfoFlags, PlayerInfoUpdatePacket,
};
use cerium::registry::DimensionType;
use cerium::util::Viewable;
use cerium::world::{Block, World};

fn main() {
    let server = Server::new();

    let world = World::new(&DimensionType::OVERWORLD);

    for bz in 0..16 {
        for bx in 0..16 {
            world.set_block(bx, 70, bz, Block::GrassBlock);
        }
    }

    let entity = Entity::new(EntityType::Player);
    entity.set_position((8, 71, 8, 0., 0.));

    world.spawn_entity(entity.clone());

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            event.set_world(world.clone());
            event.set_position((0.5, 71.0, 0.5));
        })
        .subscribe(move |event: &mut PlayerSpawnEvent| {
            let player = event.get_player();
            player.set_game_mode(GameMode::Creative);

            player.send_packet(&PlayerInfoUpdatePacket {
                actions: (PlayerInfoFlags::ADD_PLAYER | PlayerInfoFlags::UPDATE_LISTED).bits(),
                players: vec![PlayerEntry {
                    uuid: entity.uuid(),
                    player_actions: vec![
                        PlayerAction::AddPlayer {
                            name: "Custom NPC".to_string(),
                            properties: vec![],
                        },
                        PlayerAction::UpdateListed { listed: true },
                    ],
                }],
            });
            entity.add_viewer(player.clone());
        });

    server.bind("127.0.0.1:25565").unwrap();
}
