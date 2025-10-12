use std::{io::Cursor, sync::Arc};

use crate::protocol::{
    ProtocolState,
    decode::{Decode as _, DecodeError},
    packet::{
        AcknowledgeFinishConfigPacket, ClientInfoPacket, FinishConfigPacket, GameEventPacket,
        LoginPacket, PlayerAction, PlayerEntry, PlayerInfoFlags, PlayerInfoUpdatePacket,
        PluginMessagePacket, RegistryDataPacket, SetCenterChunkPacket, SyncPlayerPositionPacket,
        client, server,
    },
};
use crate::registry::{DimensionType, REGISTRIES};
use crate::{entity::Player, event::player::PlayerConfigEvent, network::client::ClientConnection};

#[rustfmt::skip]
pub async fn handle_packet(client: Arc<ClientConnection>, id: i32, data: &mut Cursor<&[u8]>) -> Result<(), DecodeError> {
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

async fn handle_client_info(client: Arc<ClientConnection>, packet: ClientInfoPacket) {
    let _ = packet;

    client
        .send_packet(server::config::KnownPacksPacket {
            known_packs: Vec::new(),
        })
        .await;

    client
        .send_packet(RegistryDataPacket::from(&REGISTRIES.cat_variant))
        .await;
    client
        .send_packet(RegistryDataPacket::from(&REGISTRIES.chicken_variant))
        .await;
    client
        .send_packet(RegistryDataPacket::from(&REGISTRIES.cow_variant))
        .await;
    client
        .send_packet(RegistryDataPacket::from(&REGISTRIES.frog_variant))
        .await;
    client
        .send_packet(RegistryDataPacket::from(&REGISTRIES.painting_variant))
        .await;
    client
        .send_packet(RegistryDataPacket::from(&REGISTRIES.pig_variant))
        .await;
    client
        .send_packet(RegistryDataPacket::from(&REGISTRIES.wolf_sound_variant))
        .await;
    client
        .send_packet(RegistryDataPacket::from(&REGISTRIES.wolf_variant))
        .await;
    client
        .send_packet(RegistryDataPacket::from(&REGISTRIES.damage_type))
        .await;
    client
        .send_packet(RegistryDataPacket::from(&REGISTRIES.dimension_type))
        .await;
    client
        .send_packet(RegistryDataPacket::from(&REGISTRIES.biome))
        .await;

    client.send_packet(FinishConfigPacket {}).await;
}

async fn handle_cookie_response(client: Arc<ClientConnection>) {
    let _ = client;
}

async fn handle_plugin_message(client: Arc<ClientConnection>, packet: PluginMessagePacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_acknowledge_finish_config(
    client: Arc<ClientConnection>,
    packet: AcknowledgeFinishConfigPacket,
) {
    let _ = packet;

    *client.state.lock().await = ProtocolState::Play;

    let player = Arc::new(Player::new(client.clone(), client.server.clone()).await);
    {
        let mut players = client.server.players.lock().await;
        players.push(player.clone());

        let mut guard = client.player.lock().await;
        *guard = Some(player.clone());
    }

    let mut event = PlayerConfigEvent {
        player: player.clone(),
        world: None,
        position: None,
    };
    client.server.events().fire(&mut event).await;

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

    client
        .send_packet(LoginPacket {
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
            game_mode: 1,
            previous_game_mode: -1,
            is_debug: false,
            is_flat: false,
            death_location: None,
            portal_cooldown: 4,
            sea_level: 64,
            enforces_secure_chat: false,
        })
        .await;

    client
        .send_packet(SyncPlayerPositionPacket {
            teleport_id: 0,
            x: position.x(),
            y: position.y(),
            z: position.z(),
            velocity_x: 0.,
            velocity_y: 0.,
            velocity_z: 0.,
            yaw: position.yaw(),
            pitch: position.pitch(),
            flags: 0,
        })
        .await;

    let game_profile = client.game_profile.lock().await.clone().unwrap();

    client
        .send_packet(PlayerInfoUpdatePacket {
            actions: (PlayerInfoFlags::ADD_PLAYER | PlayerInfoFlags::UPDATE_LISTED).bits(),
            players: vec![PlayerEntry {
                uuid: game_profile.uuid,
                player_actions: vec![
                    PlayerAction::AddPlayer {
                        name: game_profile.name.clone(),
                        properties: game_profile.properties.clone(),
                    },
                    PlayerAction::UpdateListed { listed: true },
                ],
            }],
        })
        .await;

    client
        .send_packet(GameEventPacket {
            event: 13,
            value: 0.,
        })
        .await;

    client
        .send_packet(SetCenterChunkPacket {
            chunk_x: 0.into(),
            chunk_z: 0.into(),
        })
        .await;

    player.load_chunks().await;
}

async fn handle_keep_alive(client: Arc<ClientConnection>) {
    let _ = client;
}

async fn handle_pong(client: Arc<ClientConnection>) {
    let _ = client;
}

async fn handle_resource_pack_response(client: Arc<ClientConnection>) {
    let _ = client;
}

async fn handle_client_known_packs(
    client: Arc<ClientConnection>,
    packet: client::config::KnownPacksPacket,
) {
    let _ = client;
    let _ = packet;
}

async fn handle_custom_click_action(client: Arc<ClientConnection>) {
    let _ = client;
}
