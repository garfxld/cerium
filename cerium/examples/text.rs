use cerium::Server;
use cerium::entity::Player;
use cerium::event::player::{PlayerConfigEvent, PlayerEvent, PlayerSpawnEvent};
use cerium::registry::DimensionType;
use cerium::scoreboard::Objective;
use cerium::text::{
    Component, ParentComponent, StyledComponent, color::NamedColor, style::HoverEvent,
};
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

    server
        .events()
        .subscribe(move |event: &mut PlayerConfigEvent| {
            event.set_world(world.clone());
            event.set_position((0.5, 71., 0.5));

            let player = event.get_player();
            handle_player_config(player);
        })
        .subscribe(move |event: &mut PlayerSpawnEvent| {
            let player = event.get_player();

            let objective = Objective::new("test", "Hello?");
            objective.add_viewer(player.clone());
        });

    server.bind("127.0.0.1:25565").unwrap();
}

fn handle_player_config(player: &Player) {
    let component = Component::empty()
        .child(Component::text("HOWDY!").bold().color(NamedColor::Red))
        .child(Component::new_line())
        .child("Second Line!")
        .on_hover(HoverEvent::show_text("Hello! I'm a HoverEvent."));

    println!("{}", serde_json::to_string(&component).unwrap());

    player.send_message(component);

    let header = "This is a Header.";
    let footer = Component::empty()
        .child("This is a footer.")
        .child(Component::new_line())
        .child("This is the second line of the footer. ")
        .child(Component::text("YAY!").bold().color(NamedColor::Gold));

    player.set_header_and_footer(header, footer);
}
