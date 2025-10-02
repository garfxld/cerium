use std::sync::Arc;

use cerium::Server;
use cerium::entity::Player;
use cerium::event::player::{PlayerConfigEvent, PlayerEvent as _};
use cerium::registry::DimensionType;
use cerium::text::{
    Component, ParentComponent, StyledComponent, color::NamedColor, style::HoverEvent,
};
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

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            println!("PlayerConfigEvent ({})", event.get_player().name());

            event.set_world(world.clone());
            event.set_position((0.5, 71., 0.5));

            let player = event.get_player();
            tokio::spawn({
                let player = player.clone();
                async move {
                    send_text(player).await;
                }
            });
        })
        .await;

    server.bind("127.0.0.1:25565").await.unwrap();
}

async fn send_text(player: Arc<Player>) {
    let component = Component::empty()
        .child(Component::text("HOWDY!").bold().color(NamedColor::Red))
        .child(Component::new_line())
        .child("Second Line!")
        .on_hover(HoverEvent::show_text("hi"));

    println!("{}", serde_json::to_string(&component).unwrap());

    player.send_message(component).await;
}
