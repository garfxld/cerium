use std::io::Cursor;
use std::sync::Arc;

use crate::entity::Player;
use crate::item::ItemStack;
use crate::protocol::packet::PluginMessagePacket;
use crate::protocol::{
    decode::{Decode as _, DecodeError},
    packet::{
        ChatCommandPacket, ChunkBatchReceivedPacket, ClickContainerPacket, ClientInfoPacket,
        ClientTickEndPacket, CloseContainerPacket, ConfirmTeleportationPacket,
        EntityPositionPacket, InteractPacket, PickItemFromBlockPacket, PlayerAbilitiesPacket,
        PlayerActionPacket, PlayerCommandPacket, PlayerInputPacket, PlayerLoadedPacket,
        PlayerMovementFlagsPacket, PlayerPositionAndRotationPacket, PlayerPositionPacket,
        PlayerRotationPacket, PlayerSessionPacket, SetCenterChunkPacket, SetCreativeModeSlotPacket,
        SetHeldItemPacket, SwingArmPacket, UseItemOnPacket,
        client::play::{KeepAlivePacket, PingRequestPacket},
    },
};
use crate::util::Position;
use crate::world::Chunk;

#[rustfmt::skip]
pub async fn handle_packet(player: Arc<Player>, id: i32, data: &mut Cursor<&[u8]>) -> Result<(), DecodeError> {
    match id {
        0x00 => handle_confirm_teleportation(player, ConfirmTeleportationPacket::decode(data)?).await,
        0x06 => handle_chat_command(player, ChatCommandPacket::decode(data)?).await,
        0x09 => handle_player_session(player, PlayerSessionPacket::decode(data)?).await,
        0x0A => handle_chunk_batch_received(player, ChunkBatchReceivedPacket::decode(data)?).await,
        0x0C => handle_client_tick_end(player, ClientTickEndPacket::decode(data)?).await,
        0x0D => handle_client_info(player, ClientInfoPacket::decode(data)?).await,
        0x11 => handle_click_container(player, ClickContainerPacket::decode(data)?).await,
        0x12 => handle_close_container(player, CloseContainerPacket::decode(data)?).await,
        0x15 => handle_plugin_message(player, PluginMessagePacket::decode(data)?).await,
        0x1B => handle_keep_alive(player, KeepAlivePacket::decode(data)?).await,
        0x1D => handle_player_position(player, PlayerPositionPacket::decode(data)?).await,
        0x1E => handle_player_position_and_rotation(player, PlayerPositionAndRotationPacket::decode(data)?).await,
        0x1F => handle_player_rotation(player, PlayerRotationPacket::decode(data)?).await,
        0x19 => handle_interact(player, InteractPacket::decode(data)?).await,
        0x20 => handle_player_movement_flags(player, PlayerMovementFlagsPacket::decode(data)?).await,
        0x23 => handle_pick_item_from_block(player, PickItemFromBlockPacket::decode(data)?).await,
        0x25 => handle_ping_request(player, PingRequestPacket::decode(data)?).await,
        0x27 => handle_player_abilities(player, PlayerAbilitiesPacket::decode(data)?).await,
        0x28 => handle_player_action(player, PlayerActionPacket::decode(data)?).await,
        0x29 => handle_player_command(player, PlayerCommandPacket::decode(data)?).await,
        0x2A => handle_player_input(player, PlayerInputPacket::decode(data)?).await,
        0x2B => handle_player_loaded(player, PlayerLoadedPacket::decode(data)?).await,
        0x34 => handle_set_held_item(player, SetHeldItemPacket::decode(data)?).await,
        0x37 => handle_set_creative_mode_slot(player, SetCreativeModeSlotPacket::decode(data)?).await,
        0x3C => handle_swing_arm(player, SwingArmPacket::decode(data)?).await,
        0x3F => handle_use_item_on(player, UseItemOnPacket::decode(data)?).await,
        _ => return Err(DecodeError::UnkownPacket(id)),
    };
    Ok(())
}

async fn handle_confirm_teleportation(player: Arc<Player>, packet: ConfirmTeleportationPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_chat_command(player: Arc<Player>, packet: ChatCommandPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_player_session(player: Arc<Player>, packet: PlayerSessionPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_chunk_batch_received(player: Arc<Player>, packet: ChunkBatchReceivedPacket) {
    println!("{}", packet.chunks_per_tick);
    let mut queue = player.chunk_queue.lock();
    queue.lead -= 1;
    queue.target_cpt = if packet.chunks_per_tick.is_nan() {
        0.01
    } else {
        (packet.chunks_per_tick * 1.).clamp(0.01, 64.)
    };

    if queue.max_lead == 1 {
        queue.max_lead = 10;
    }
}

async fn handle_client_tick_end(player: Arc<Player>, packet: ClientTickEndPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_client_info(player: Arc<Player>, packet: ClientInfoPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_click_container(player: Arc<Player>, packet: ClickContainerPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_close_container(player: Arc<Player>, packet: CloseContainerPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_plugin_message(player: Arc<Player>, packet: PluginMessagePacket) {
    println!("{:?}", packet);
    let _ = player;
    let _ = packet;
}

async fn handle_keep_alive(player: Arc<Player>, packet: KeepAlivePacket) {
    let _ = player;
    let _ = packet;
}

// ===== Position & Movement ======

async fn handle_player_position(player: Arc<Player>, packet: PlayerPositionPacket) {
    let new_position = Position::new(
        packet.x,
        packet.feet_y,
        packet.z,
        player.position().yaw(),
        player.position().pitch(),
    );
    handle_movement(player, new_position).await;
}

async fn handle_player_position_and_rotation(
    player: Arc<Player>,
    packet: PlayerPositionAndRotationPacket,
) {
    let new_position = Position::new(packet.x, packet.feet_y, packet.z, packet.yaw, packet.pitch);
    handle_movement(player, new_position).await;
}

async fn handle_player_rotation(player: Arc<Player>, packet: PlayerRotationPacket) {
    let new_position = Position::new(
        player.position().x(),
        player.position().y(),
        player.position().z(),
        packet.yaw,
        packet.pitch,
    );
    handle_movement(player, new_position).await;
}

async fn handle_movement(player: Arc<Player>, new_position: Position) {
    let old_position = player.position();

    if old_position == new_position {
        return;
    }

    let old_chunk = Chunk::to_chunk_pos(old_position);

    let new_chunk = Chunk::to_chunk_pos(new_position);

    player.set_position(new_position);
    if old_chunk != new_chunk {
        player.send_packet(SetCenterChunkPacket {
            chunk_x: new_chunk.0,
            chunk_z: new_chunk.1,
        });
        player.update_chunks(new_chunk, old_chunk);
    }

    let delta_x = new_position.x() * 4096. - old_position.x() * 4096.;
    let delta_y = new_position.y() * 4096. - old_position.y() * 4096.;
    let delta_z = new_position.z() * 4096. - old_position.z() * 4096.;

    player.send_packet(EntityPositionPacket {
        entitiy_id: 0,
        delta_x: delta_x as i16,
        delta_y: delta_y as i16,
        delta_z: delta_z as i16,
        on_ground: false,
    });
}

async fn handle_interact(player: Arc<Player>, packet: InteractPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_player_movement_flags(player: Arc<Player>, packet: PlayerMovementFlagsPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_pick_item_from_block(player: Arc<Player>, packet: PickItemFromBlockPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_ping_request(player: Arc<Player>, packet: PingRequestPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_player_abilities(player: Arc<Player>, packet: PlayerAbilitiesPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_player_action(player: Arc<Player>, packet: PlayerActionPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_player_command(player: Arc<Player>, packet: PlayerCommandPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_player_input(player: Arc<Player>, packet: PlayerInputPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_player_loaded(player: Arc<Player>, packet: PlayerLoadedPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_set_held_item(player: Arc<Player>, packet: SetHeldItemPacket) {
    let _ = player;
    let _ = packet;
}

async fn handle_set_creative_mode_slot(player: Arc<Player>, packet: SetCreativeModeSlotPacket) {
    let inventory = player.inventory();

    let item_stack = ItemStack::from(packet.clicked_item);

    inventory.set_item_stack(packet.slot as i32, item_stack);
}

async fn handle_swing_arm(player: Arc<Player>, packet: SwingArmPacket) {
    let _ = packet;
    let _ = player;
}

async fn handle_use_item_on(player: Arc<Player>, packet: UseItemOnPacket) {
    let _ = player;
    let _ = packet;
}
