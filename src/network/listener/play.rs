use std::sync::Arc;

use cerium_protocol::{
    buffer::ByteBuffer,
    decode::{Decode as _, DecodeError},
    packet::{
        ChatCommandPacket, ClientInfoPacket, ClientPluginMessagePacket, ClientTickEndPacket,
        CloseContainerPacket, ConfirmTeleportationPacket, KeepAlivePacket, PickItemFromBlockPacket,
        PlayerAbilitiesPacket, PlayerActionPacket, PlayerCommandPacket, PlayerInputPacket,
        PlayerLoadedPacket, PlayerMovementFlagsPacket, PlayerPositionAndRotationPacket,
        PlayerPositionPacket, PlayerRotationPacket, PlayerSessionPacket, SetHeldItemPacket,
        SwingArmPacket, UseItemOnPacket,
    },
};

use crate::network::client::ClientConnection;

pub async fn handle_packet(
    client: Arc<ClientConnection>,
    id: i32,
    data: &mut ByteBuffer,
) -> Result<(), DecodeError> {
    match id {
        0x00 => {
            handle_confirm_teleportation(client, ConfirmTeleportationPacket::decode(data)?).await
        }
        0x06 => handle_chat_command(client, ChatCommandPacket::decode(data)?).await,
        0x09 => handle_player_session(client, PlayerSessionPacket::decode(data)?).await,
        0x0C => handle_client_tick_end(client, ClientTickEndPacket::decode(data)?).await,
        0x0D => handle_client_info(client, ClientInfoPacket::decode(data)?).await,
        0x11 => handle_click_container(client).await,
        0x12 => handle_close_container(client, CloseContainerPacket::decode(data)?).await,
        0x15 => handle_plugin_message(client, ClientPluginMessagePacket::decode(data)?).await,
        0x1B => handle_keep_alive(client, KeepAlivePacket::decode(data)?).await,
        0x1D => handle_player_position(client, PlayerPositionPacket::decode(data)?).await,
        0x1E => {
            handle_player_position_and_rotation(
                client,
                PlayerPositionAndRotationPacket::decode(data)?,
            )
            .await
        }
        0x1F => handle_player_rotation(client, PlayerRotationPacket::decode(data)?).await,
        0x20 => {
            handle_player_movement_flags(client, PlayerMovementFlagsPacket::decode(data)?).await
        }
        0x23 => handle_pick_item_from_block(client, PickItemFromBlockPacket::decode(data)?).await,
        0x27 => handle_player_abilities(client, PlayerAbilitiesPacket::decode(data)?).await,
        0x28 => handle_player_action(client, PlayerActionPacket::decode(data)?).await,
        0x29 => handle_player_command(client, PlayerCommandPacket::decode(data)?).await,
        0x2A => handle_player_input(client, PlayerInputPacket::decode(data)?).await,
        0x2B => handle_player_loaded(client, PlayerLoadedPacket::decode(data)?).await,
        0x34 => handle_set_held_item(client, SetHeldItemPacket::decode(data)?).await,
        0x3C => handle_swing_arm(client, SwingArmPacket::decode(data)?).await,
        0x3F => handle_use_item_on(client, UseItemOnPacket::decode(data)?).await,
        _ => panic!("Unknown packet! ({})", id),
    };
    Ok(())
}

async fn handle_confirm_teleportation(
    client: Arc<ClientConnection>,
    packet: ConfirmTeleportationPacket,
) {
    let _ = client;
    let _ = packet;
}

async fn handle_chat_command(client: Arc<ClientConnection>, packet: ChatCommandPacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_player_session(client: Arc<ClientConnection>, packet: PlayerSessionPacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_client_tick_end(client: Arc<ClientConnection>, packet: ClientTickEndPacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_client_info(client: Arc<ClientConnection>, packet: ClientInfoPacket) {
    println!("{:?}", packet);
    let _ = client;
    let _ = packet;
}

async fn handle_click_container(client: Arc<ClientConnection>) {
    let _ = client;
    todo!();
}

async fn handle_close_container(client: Arc<ClientConnection>, packet: CloseContainerPacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_plugin_message(client: Arc<ClientConnection>, packet: ClientPluginMessagePacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_keep_alive(client: Arc<ClientConnection>, packet: KeepAlivePacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_player_position(client: Arc<ClientConnection>, packet: PlayerPositionPacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_player_position_and_rotation(
    client: Arc<ClientConnection>,
    packet: PlayerPositionAndRotationPacket,
) {
    let _ = client;
    let _ = packet;
}

async fn handle_player_rotation(client: Arc<ClientConnection>, packet: PlayerRotationPacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_player_movement_flags(
    client: Arc<ClientConnection>,
    packet: PlayerMovementFlagsPacket,
) {
    let _ = client;
    let _ = packet;
}

async fn handle_pick_item_from_block(
    client: Arc<ClientConnection>,
    packet: PickItemFromBlockPacket,
) {
    let _ = client;
    let _ = packet;
}

async fn handle_player_abilities(client: Arc<ClientConnection>, packet: PlayerAbilitiesPacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_player_action(client: Arc<ClientConnection>, packet: PlayerActionPacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_player_command(client: Arc<ClientConnection>, packet: PlayerCommandPacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_player_input(client: Arc<ClientConnection>, packet: PlayerInputPacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_player_loaded(client: Arc<ClientConnection>, packet: PlayerLoadedPacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_set_held_item(client: Arc<ClientConnection>, packet: SetHeldItemPacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_swing_arm(client: Arc<ClientConnection>, packet: SwingArmPacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_use_item_on(client: Arc<ClientConnection>, packet: UseItemOnPacket) {
    let _ = client;
    let _ = packet;
}
