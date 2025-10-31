use std::io::Cursor;

use crate::{
    entity::{EntityAnimation, EntityLike as _, GameMode, Hand, Player},
    item::ItemStack,
    protocol::{
        decode::{Decode as _, DecodeError},
        packet::{
            AcknowledgeBlockChangePacket, ChangeRecipeBookSettingsPacket, ChatCommandPacket,
            ChunkBatchReceivedPacket, ClickContainerPacket, ClientInfoPacket, ClientTickEndPacket,
            ConfirmTeleportationPacket, EntityAnimationPacket, InteractPacket,
            PickItemFromBlockPacket, PlayerActionPacket, PlayerCommand, PlayerCommandPacket,
            PlayerDiggingState, PlayerInputFlags, PlayerInputPacket, PlayerLoadedPacket,
            PlayerMovementFlagsPacket, PlayerPositionAndRotationPacket, PlayerPositionPacket,
            PlayerRotationPacket, PlayerSessionPacket, PluginMessagePacket,
            SetBlockDestroyStagePacket, SetCreativeModeSlotPacket, SwingArmPacket, UseItemOnPacket,
            client::play::{
                CloseContainerPacket, KeepAlivePacket, PingRequestPacket, PlayerAbilitiesPacket,
                SetHeldItemPacket,
            },
        },
    },
    util::{Position, Viewable},
};

#[rustfmt::skip]
pub fn handle_packet(player: Player, id: i32, data: &mut Cursor<&[u8]>) -> Result<(), DecodeError> {
    match id {
        0x00 => handle_confirm_teleportation(player, ConfirmTeleportationPacket::decode(data)?),
        0x06 => handle_chat_command(player, ChatCommandPacket::decode(data)?),
        0x09 => handle_player_session(player, PlayerSessionPacket::decode(data)?),
        0x0A => handle_chunk_batch_received(player, ChunkBatchReceivedPacket::decode(data)?),
        0x0C => handle_client_tick_end(player, ClientTickEndPacket::decode(data)?),
        0x0D => handle_client_info(player, ClientInfoPacket::decode(data)?),
        0x11 => handle_click_container(player, ClickContainerPacket::decode(data)?),
        0x12 => handle_close_container(player, CloseContainerPacket::decode(data)?),
        0x15 => handle_plugin_message(player, PluginMessagePacket::decode(data)?),
        0x1B => handle_keep_alive(player, KeepAlivePacket::decode(data)?),
        0x1D => handle_player_position(player, PlayerPositionPacket::decode(data)?),
        0x1E => handle_player_position_and_rotation(player, PlayerPositionAndRotationPacket::decode(data)?),
        0x1F => handle_player_rotation(player, PlayerRotationPacket::decode(data)?),
        0x19 => handle_interact(player, InteractPacket::decode(data)?),
        0x20 => handle_player_movement_flags(player, PlayerMovementFlagsPacket::decode(data)?),
        0x23 => handle_pick_item_from_block(player, PickItemFromBlockPacket::decode(data)?),
        0x25 => handle_ping_request(player, PingRequestPacket::decode(data)?),
        0x27 => handle_player_abilities(player, PlayerAbilitiesPacket::decode(data)?),
        0x28 => handle_player_action(player, PlayerActionPacket::decode(data)?),
        0x29 => handle_player_command(player, PlayerCommandPacket::decode(data)?),
        0x2A => handle_player_input(player, PlayerInputPacket::decode(data)?),
        0x2B => handle_player_loaded(player, PlayerLoadedPacket::decode(data)?),
        0x2D => hande_change_recipe_book_settings(player, ChangeRecipeBookSettingsPacket::decode(data)?),
        0x34 => handle_set_held_item(player, SetHeldItemPacket::decode(data)?),
        0x37 => handle_set_creative_mode_slot(player, SetCreativeModeSlotPacket::decode(data)?),
        0x3C => handle_swing_arm(player, SwingArmPacket::decode(data)?),
        0x3F => handle_use_item_on(player, UseItemOnPacket::decode(data)?),
        _ => return Err(DecodeError::UnkownPacket(id)),
    };
    Ok(())
}

fn handle_confirm_teleportation(_player: Player, _packet: ConfirmTeleportationPacket) {
    log::warn!("todo: handle_confirm_teleportation");
}

fn handle_chat_command(_player: Player, _packet: ChatCommandPacket) {
    log::warn!("todo: handle_chat_command");
}

fn handle_player_session(_player: Player, _packet: PlayerSessionPacket) {
    log::warn!("todo: handle_player_session");
}

fn handle_chunk_batch_received(player: Player, packet: ChunkBatchReceivedPacket) {
    let mut queue = player.0.chunk_queue.lock();
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

fn handle_client_tick_end(_player: Player, _packet: ClientTickEndPacket) {
    // todo: handle_client_tick_end
}

fn handle_client_info(_player: Player, _packet: ClientInfoPacket) {
    log::warn!("todo: handle_client_info");
}

fn handle_click_container(player: Player, packet: ClickContainerPacket) {
    log::warn!("todo: handle_click_container");
    if packet.slot == -1 {
        return;
    }

    if packet.window_id == 0 {
        // todo

        return;
    }

    let Some(_inventory) = player.get_open_inventory() else {
        return;
    };

    // todo
}

fn handle_close_container(player: Player, packet: CloseContainerPacket) {
    let _ = packet;
    player.close_inventory();
}

fn handle_plugin_message(_player: Player, _packet: PluginMessagePacket) {
    log::warn!("todo: handle_plugin_message");
}

fn handle_keep_alive(_player: Player, _packet: KeepAlivePacket) {
    log::warn!("todo: handle_keep_alive");
}

// ===== Position & Movement ======

fn handle_player_position(player: Player, packet: PlayerPositionPacket) {
    let new_position = Position::new(
        packet.x,
        packet.feet_y,
        packet.z,
        player.position().yaw(),
        player.position().pitch(),
    );
    handle_movement(player, new_position, packet.flags & 1 != 0);
}

fn handle_player_position_and_rotation(player: Player, packet: PlayerPositionAndRotationPacket) {
    let new_position = Position::new(packet.x, packet.feet_y, packet.z, packet.yaw, packet.pitch);
    handle_movement(player, new_position, packet.flags & 1 != 0);
}

fn handle_player_rotation(player: Player, packet: PlayerRotationPacket) {
    let new_position = Position::new(
        player.position().x(),
        player.position().y(),
        player.position().z(),
        packet.yaw,
        packet.pitch,
    );
    handle_movement(player, new_position, packet.flags & 1 != 0);
}

fn handle_movement(player: Player, new_position: Position, on_ground: bool) {
    let old_position = player.position();

    if new_position == old_position {
        return;
    }

    player.refresh_position(new_position);
    player.refresh_on_ground(on_ground);
}

fn handle_interact(_player: Player, _packet: InteractPacket) {
    log::warn!("todo: handle_interact");
}

fn handle_player_movement_flags(_player: Player, _packet: PlayerMovementFlagsPacket) {
    log::warn!("todo: handle_player_movement_flags");
}

fn handle_pick_item_from_block(_player: Player, _packet: PickItemFromBlockPacket) {
    log::warn!("todo: handle_pick_item_from_block");
}

fn handle_ping_request(_player: Player, _packet: PingRequestPacket) {
    log::warn!("todo: handle_ping_request");
}

fn handle_player_abilities(player: Player, packet: PlayerAbilitiesPacket) {
    let can_fly = player.allow_flying() || player.game_mode() == GameMode::Creative;

    if can_fly {
        let flying = (packet.flags & 0x02) != 0;
        player.set_flying(flying);
    }
}

fn handle_player_action(player: Player, packet: PlayerActionPacket) {
    let world = player.world();
    let status = packet.status;
    let position = packet.position;
    let face = packet.face;

    match status {
        PlayerDiggingState::StartDigging => {
            if player.game_mode() == GameMode::Creative {
                // intant break the block
                world.break_block(player.clone(), position, face);
            }
        }
        PlayerDiggingState::CancelledDigging => {}
        PlayerDiggingState::FinishedDigging => {
            world.break_block(player.clone(), position, face);
        }
        PlayerDiggingState::DropItemStack => {}
        PlayerDiggingState::DropItem => {}
        PlayerDiggingState::ItemUpdated => {}
        PlayerDiggingState::SwapItemInHand => {}
    }

    let packet = SetBlockDestroyStagePacket {
        entitiy_id: player.id(),
        location: position,
        destroy_stage: status as u8,
    };
    player.send_packet(&packet);
    player.send_packet_to_viewers(&packet);
}

fn handle_player_command(player: Player, packet: PlayerCommandPacket) {
    match packet.action_id {
        PlayerCommand::StartSprinting => player.0.set_sprinting(true),
        PlayerCommand::StopSprinting => player.0.set_sprinting(false),
        _ => todo!(),
    }
}

fn handle_player_input(player: Player, packet: PlayerInputPacket) {
    player
        .0
        .set_sneaking(packet.flags.contains(PlayerInputFlags::SNEAK));
}

fn handle_player_loaded(_player: Player, _packet: PlayerLoadedPacket) {
    log::warn!("todo: handle_player_loaded");
}

fn hande_change_recipe_book_settings(_player: Player, _packet: ChangeRecipeBookSettingsPacket) {
    log::warn!("todo: hande_change_recipe_book_settings");
}

fn handle_set_held_item(player: Player, packet: SetHeldItemPacket) {
    player.0.update_held_slot(packet.slot as u8);
}

fn handle_set_creative_mode_slot(player: Player, packet: SetCreativeModeSlotPacket) {
    let inventory = player.inventory();

    let item_stack = ItemStack::from(packet.clicked_item);

    inventory.set_item_stack(packet.slot as i32, item_stack);
}

fn handle_swing_arm(player: Player, packet: SwingArmPacket) {
    player.send_packet_to_viewers(&EntityAnimationPacket {
        entity_id: player.id(),
        animation: if packet.hand == Hand::MainHand {
            EntityAnimation::SwingMainArm
        } else {
            EntityAnimation::SwingOffhand
        },
    });
}

fn handle_use_item_on(player: Player, packet: UseItemOnPacket) {
    let world = player.world();
    let position = packet.position;

    let Some(placed_block) = player.get_item_in_hand(packet.hand) else {
        return;
    };

    let Some(block) = placed_block.material().block() else {
        return;
    };

    world.place_block(player.clone(), position, packet.face, block.clone());
    player.send_packet(&AcknowledgeBlockChangePacket {
        sequence_id: packet.sequence,
    });
}
