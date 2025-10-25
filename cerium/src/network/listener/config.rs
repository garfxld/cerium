use std::{io::Cursor, sync::Arc};

use crate::entity::EntityLike as _;
use crate::event::player::PlayerSpawnEvent;
use crate::registry::{DimensionType, REGISTRIES};
use crate::util::{Position, TeleportFlags, Viewable};
use crate::{entity::Player, event::player::PlayerConfigEvent, network::client::Connection};
use crate::{
    protocol::{
        ProtocolState,
        decode::{Decode as _, DecodeError},
        packet::{
            AcknowledgeFinishConfigPacket, ClientInfoPacket, FeatureFlagsPacket,
            FinishConfigPacket, GameEventPacket, LoginPacket, PluginMessagePacket,
            RegistryDataPacket, SetCenterChunkPacket, client, server,
        },
    },
    util::Identifier,
};

#[rustfmt::skip]
pub async fn handle_packet(client: Arc<Connection>, id: i32, data: &mut Cursor<&[u8]>) -> Result<(), DecodeError> {
    match id {
        0x00 => handle_client_info(client, ClientInfoPacket::decode(data)?).await,
        0x01 => handle_cookie_response(client).await,
        0x02 => handle_plugin_message(client, PluginMessagePacket::decode(data)?).await,
        0x03 => handle_acknowledge_finish_config(client, AcknowledgeFinishConfigPacket::decode(data)?).await,
        0x04 => handle_keep_alive(client).await,
        0x05 => handle_pong(client).await,
        0x06 => handle_resource_pack_response(client).await,
        0x07 => handle_client_known_packs(client, client::config::KnownPacksPacket::decode(data)?).await,
        0x08 => handle_custom_click_action(client).await,
        _ => return Err(DecodeError::UnkownPacket(id)),
    };
    Ok(())
}

async fn handle_client_info(client: Arc<Connection>, packet: ClientInfoPacket) {
    let _ = packet;

    client.send_packet(server::config::KnownPacksPacket {
        known_packs: Vec::new(),
    });

    client.send_packet(FeatureFlagsPacket {
        feature_flags: vec![Identifier::vanilla("vanilla")],
    });

    client.send_packet(RegistryDataPacket::from(&REGISTRIES.cat_variant));
    client.send_packet(RegistryDataPacket::from(&REGISTRIES.chicken_variant));
    client.send_packet(RegistryDataPacket::from(&REGISTRIES.cow_variant));
    client.send_packet(RegistryDataPacket::from(&REGISTRIES.frog_variant));
    client.send_packet(RegistryDataPacket::from(&REGISTRIES.painting_variant));
    client.send_packet(RegistryDataPacket::from(&REGISTRIES.pig_variant));
    client.send_packet(RegistryDataPacket::from(&REGISTRIES.wolf_sound_variant));
    client.send_packet(RegistryDataPacket::from(&REGISTRIES.wolf_variant));
    client.send_packet(RegistryDataPacket::from(&REGISTRIES.damage_type));
    client.send_packet(RegistryDataPacket::from(&REGISTRIES.dimension_type));
    client.send_packet(RegistryDataPacket::from(&REGISTRIES.biome));

    client.send_packet(FinishConfigPacket {});
}

async fn handle_cookie_response(client: Arc<Connection>) {
    let _ = client;
}

async fn handle_plugin_message(client: Arc<Connection>, packet: PluginMessagePacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_acknowledge_finish_config(
    client: Arc<Connection>,
    packet: AcknowledgeFinishConfigPacket,
) {
    let _ = packet;

    client.set_state(ProtocolState::Play);

    let player = Arc::new(Player::new(client.clone(), client.server().clone()));
    {
        let mut players = client.server().players.lock();
        players.push(player.clone());

        let mut guard = client.player.lock();
        *guard = Some(player.clone());
    }

    let mut event = PlayerConfigEvent {
        player: player.clone(),
        world: None,
        position: None,
    };
    client.server().events().fire(&mut event);

    if let Some(world) = event.world {
        player.set_world(world);
    } else {
        todo!("no world set");
    }

    let position = if let Some(position) = event.position {
        player.set_position(position);
        position
    } else {
        todo!("no position set");
    };

    client.send_packet(LoginPacket {
        entity_id: player.id(),
        is_hardcore: false,
        dimension_names: vec!["minecraft:overworld".to_owned()],
        max_players: 20,
        view_distance: 32,
        simulation_distance: 8,
        reduced_debug_info: false,
        enable_respawn_screen: true,
        do_limited_crafting: false,
        dimension_type: REGISTRIES
            .dimension_type
            .get_id(&DimensionType::OVERWORLD)
            .unwrap_or(0) as i32,
        dimension_name: "minecraft:overworld".to_owned(),
        hashed_seed: 93522819,
        game_mode: 0,
        previous_game_mode: -1,
        is_debug: false,
        is_flat: false,
        death_location: None,
        portal_cooldown: 4,
        sea_level: 64,
        enforces_secure_chat: false,
    });

    player.synchronize_position(position, Position::ZERO, TeleportFlags::empty());

    client.send_packet(GameEventPacket::START_WAITING_FOR_CHUNKS);

    client.send_packet(SetCenterChunkPacket {
        chunk_x: 0.into(),
        chunk_z: 0.into(),
    });

    client.server().events().fire(&mut PlayerSpawnEvent {
        player: player.clone(),
    });

    let online_players = &*client.server().players.lock();

    // Add player to tab for already playing players.
    for online_player in online_players {
        online_player.send_packet(player.add_to_list_packet());
        if *online_player != player {
            player.add_viewer(online_player.clone());
        }
    }

    // Add already playing player to tab for player.
    for online_player in online_players {
        if *online_player == player {
            continue;
        }
        online_player.add_viewer(player.clone());
    }

    player.load_chunks();
}

async fn handle_keep_alive(client: Arc<Connection>) {
    let _ = client;
}

async fn handle_pong(client: Arc<Connection>) {
    let _ = client;
}

async fn handle_resource_pack_response(client: Arc<Connection>) {
    let _ = client;
}

async fn handle_client_known_packs(
    client: Arc<Connection>,
    packet: client::config::KnownPacksPacket,
) {
    let _ = client;
    let _ = packet;
}

async fn handle_custom_click_action(client: Arc<Connection>) {
    let _ = client;
}
