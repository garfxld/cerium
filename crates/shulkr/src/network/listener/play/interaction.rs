use crate::{
    entity::{EntityAnimation, EntityLike as _, EntityType, GameMode, Hand, Player},
    event::player::{PlayerAttackEvent, PlayerPickBlockEvent, PlayerPickEntityEvent},
    protocol::packet::{
        AcknowledgeBlockChangePacket, AttackPacket, EntityAnimationPacket, InteractPacket,
        PickItemFromBlockPacket, PickItemFromEntityPacket, PlayerActionPacket, PlayerDiggingState,
        SetBlockDestroyStagePacket, SwingArmPacket, UseItemOnPacket, UseItemPacket,
    },
    registry::Id,
    util::{BlockPosition, Viewable as _},
};

pub(crate) fn handle_attack(player: Player, packet: AttackPacket) {
    let server = player.server();
    let world = player.world();
    let entities = world.entities();

    let Some(entity) = entities.iter().find(|e| e.id() == packet.entity_id) else {
        return;
    };

    let event = &mut PlayerAttackEvent {
        player: player.clone(),
        entity: entity.clone(),
    };
    server.events().fire(event);
}

pub(crate) fn handle_interact(_player: Player, _packet: InteractPacket) {
    tracing::warn!("todo: handle_interact");
}

pub(crate) fn handle_pick_item_from_block(player: Player, packet: PickItemFromBlockPacket) {
    let server = player.server();
    server.events().fire(&mut PlayerPickBlockEvent {
        player: player.clone(),
        position: BlockPosition::from_long(packet.position),
        include_data: packet.include_data,
    });
}

pub(crate) fn handle_pick_item_from_entity(player: Player, packet: PickItemFromEntityPacket) {
    let server = player.server();
    server.events().fire(&mut PlayerPickEntityEvent {
        player: player.clone(),
        entity_type: EntityType::from_id(packet.entity_id as Id).expect("Invalid EntityType"),
        include_data: packet.include_data,
    });
}

pub(crate) fn handle_player_action(player: Player, packet: PlayerActionPacket) {
    let world = player.world();
    let status = packet.status;
    let position = packet.position;
    let face = packet.face;
    let sequence = packet.sequence;

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
    player.broadcast_packet(&packet);

    player.send_packet(&AcknowledgeBlockChangePacket {
        sequence_id: sequence,
    });
}

pub(crate) fn handle_swing_arm(player: Player, packet: SwingArmPacket) {
    player.broadcast_packet(&EntityAnimationPacket {
        entity_id: player.id(),
        animation: if packet.hand == Hand::Main {
            EntityAnimation::SwingMainArm
        } else {
            EntityAnimation::SwingOffhand
        },
    });
}

pub(crate) fn handle_use_item_on(player: Player, packet: UseItemOnPacket) {
    let world = player.world();
    let position = packet.position;

    let Some(placed_block) = player.get_item_in_hand(packet.hand) else {
        return;
    };

    let Some(block) = placed_block.material().data().block else {
        return;
    };

    world.place_block(player.clone(), position, packet.face, block.default_state());
    player.send_packet(&AcknowledgeBlockChangePacket {
        sequence_id: packet.sequence,
    });
}

pub(crate) fn handle_use_item(_player: Player, _packet: UseItemPacket) {}
