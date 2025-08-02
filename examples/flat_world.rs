use cerium::event::player::{PlayerConfigEvent, PlayerEvent as _};
use cerium::event::ServerListPingEvent;
use cerium::registry::registry::REGISTRIES;
use cerium::world::World;
use cerium::Server;

#[tokio::main]
pub async fn main() {
    let server = Server::new();

    let overworld = REGISTRIES
        .dimension_type
        .get("minecraft:overworld".to_owned())
        .expect("failed to get dimension_type");

    let mut world = World::new(overworld.clone());

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

    server
        .events()
        .subscribe(|event: &mut ServerListPingEvent| {
            event.set_response(
                r#"
                {
                    "version": {
                        "name": "1.21.7",
                        "protocol": 772
                    },
                    "players": {
                        "max": 100,
                        "online": 5,
                        "sample": [
                            {
                                "name": "thinkofdeath",
                                "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
                            }
                        ]
                    },
                    "description": {
                        "text": "CUSTOM SERVER LIST PING EVENT!!! LESSGOO"
                    },
                    "favicon": "data:image/png;base64,<data>",
                    "enforcesSecureChat": false
                }
            "#
                .to_owned(),
            );
        })
        .await;

    server.bind("127.0.0.1:25565").await.unwrap();
}
