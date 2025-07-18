use crate::{
    network::client::ClientConnection,
    protocol::{
        buffer::ByteBuffer,
        decode::Decode as _,
        packet::{
            ChatCommandPacket, ClientPluginMessagePacket, ClientTickEndPacket,
            CloseContainerPacket, ConfirmTeleportationPacket, KeepAlivePacket,
            PickItemFromBlockPacket, PlayerAbilitiesPacket, PlayerActionPacket,
            PlayerCommandPacket, PlayerInputPacket, PlayerLoadedPacket,
            PlayerPositionAndRotationPacket, PlayerPositionPacket, PlayerRotationPacket,
            PlayerSessionPacket, SetHeldItemPacket, SwingArmPacket, UseItemOnPacket,
        },
    },
};

pub(crate) fn handle_packet(client: &mut ClientConnection, id: i32, data: &mut ByteBuffer) {
    match id {
        0x00 => {
            handle_confirm_teleportation(client, ConfirmTeleportationPacket::decode(data).unwrap())
        }
        0x06 => handle_chat_command(client, ChatCommandPacket::decode(data).unwrap()),
        0x09 => handle_player_session(client, PlayerSessionPacket::decode(data).unwrap()),
        0x0C => handle_client_tick_end(client, ClientTickEndPacket::decode(data).unwrap()),
        0x11 => handle_click_container(client),
        0x12 => handle_close_container(client, CloseContainerPacket::decode(data).unwrap()),
        0x15 => handle_plugin_message(client, ClientPluginMessagePacket::decode(data).unwrap()),
        0x1B => handle_keep_alive(client, KeepAlivePacket::decode(data).unwrap()),
        0x1D => handle_player_position(client, PlayerPositionPacket::decode(data).unwrap()),
        0x1E => handle_player_position_and_rotation(
            client,
            PlayerPositionAndRotationPacket::decode(data).unwrap(),
        ),
        0x1F => handle_player_rotation(client, PlayerRotationPacket::decode(data).unwrap()),
        0x23 => handle_pick_item_from_block(client, PickItemFromBlockPacket::decode(data).unwrap()),
        0x27 => handle_player_abilities(client, PlayerAbilitiesPacket::decode(data).unwrap()),
        0x28 => handle_player_action(client, PlayerActionPacket::decode(data).unwrap()),
        0x29 => handle_player_command(client, PlayerCommandPacket::decode(data).unwrap()),
        0x2A => handle_player_input(client, PlayerInputPacket::decode(data).unwrap()),
        0x2B => handle_player_loaded(client, PlayerLoadedPacket::decode(data).unwrap()),
        0x34 => handle_set_held_item(client, SetHeldItemPacket::decode(data).unwrap()),
        0x3C => handle_swing_arm(client, SwingArmPacket::decode(data).unwrap()),
        0x3F => handle_use_item_on(client, UseItemOnPacket::decode(data).unwrap()),
        _ => panic!("Unknown packet! ({})", id),
    }
}

fn handle_confirm_teleportation(client: &mut ClientConnection, packet: ConfirmTeleportationPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_chat_command(client: &mut ClientConnection, packet: ChatCommandPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_player_session(client: &mut ClientConnection, packet: PlayerSessionPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_client_tick_end(client: &mut ClientConnection, packet: ClientTickEndPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_click_container(client: &mut ClientConnection) {
    let _ = client;
    todo!();
}

fn handle_close_container(client: &mut ClientConnection, packet: CloseContainerPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_plugin_message(client: &mut ClientConnection, packet: ClientPluginMessagePacket) {
    let _ = client;
    let _ = packet;
}

fn handle_keep_alive(client: &mut ClientConnection, packet: KeepAlivePacket) {
    let _ = client;
    let _ = packet;
}

fn handle_player_position(client: &mut ClientConnection, packet: PlayerPositionPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_player_position_and_rotation(
    client: &mut ClientConnection,
    packet: PlayerPositionAndRotationPacket,
) {
    let _ = client;
    let _ = packet;
}

fn handle_player_rotation(client: &mut ClientConnection, packet: PlayerRotationPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_pick_item_from_block(client: &mut ClientConnection, packet: PickItemFromBlockPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_player_abilities(client: &mut ClientConnection, packet: PlayerAbilitiesPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_player_action(client: &mut ClientConnection, packet: PlayerActionPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_player_command(client: &mut ClientConnection, packet: PlayerCommandPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_player_input(client: &mut ClientConnection, packet: PlayerInputPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_player_loaded(client: &mut ClientConnection, packet: PlayerLoadedPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_set_held_item(client: &mut ClientConnection, packet: SetHeldItemPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_swing_arm(client: &mut ClientConnection, packet: SwingArmPacket) {
    let _ = client;
    let _ = packet;
}

fn handle_use_item_on(client: &mut ClientConnection, packet: UseItemOnPacket) {
    let _ = client;
    let _ = packet;
}