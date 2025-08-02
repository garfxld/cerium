use std::sync::Arc;

use crate::{entity::player::Player, network::client::ClientConnection};
use cerium_protocol::{
    buffer::ByteBuffer,
    decode::{Decode as _, DecodeError},
    packet::{
        AcknowledgeFinishConfigPacket, ChunkDataAndUpdateLightPacket, ClientInfoPacket,
        ClientKnownPacksPacket, ClientPluginMessagePacket, FinishConfigPacket, GameEventPacket,
        LoginPacket, PlayerAction, PlayerEntry, PlayerInfoFlags, PlayerInfoUpdatePacket,
        RegistryDataPacket, ServerKnownPacksPacket, SetCenterChunkPacket, SyncPlayerPositionPacket,
    },
    ProtocolState,
};
use cerium_registry::registry::REGISTRIES;
use cerium_world::World;

pub async fn handle_packet(
    client: Arc<ClientConnection>,
    id: i32,
    data: &mut ByteBuffer,
) -> Result<(), DecodeError> {
    match id {
        0x00 => handle_client_info(client, ClientInfoPacket::decode(data)?).await,
        0x01 => handle_cookie_response(client).await,
        0x02 => handle_plugin_message(client, ClientPluginMessagePacket::decode(data)?).await,
        0x03 => {
            handle_acknowledge_finish_config(client, AcknowledgeFinishConfigPacket::decode(data)?)
                .await
        }
        0x04 => handle_keep_alive(client).await,
        0x05 => handle_pong(client).await,
        0x06 => handle_resource_pack_response(client).await,
        0x07 => handle_client_known_packs(client, ClientKnownPacksPacket::decode(data)?).await,
        0x08 => handle_custom_click_action(client).await,
        _ => panic!("Unknown packet! ({})", id),
    };
    Ok(())
}

async fn handle_client_info(client: Arc<ClientConnection>, packet: ClientInfoPacket) {
    log::trace!("{:?}", &packet);

    client
        .send_packet(
            0x0E,
            ServerKnownPacksPacket {
                known_packs: Vec::new(),
            },
        )
        .await;

    client
        .send_packet(
            0x07,
            RegistryDataPacket::from(REGISTRIES.cat_variant.clone()),
        )
        .await;
    client
        .send_packet(
            0x07,
            RegistryDataPacket::from(REGISTRIES.chicken_variant.clone()),
        )
        .await;
    client
        .send_packet(
            0x07,
            RegistryDataPacket::from(REGISTRIES.cow_variant.clone()),
        )
        .await;
    client
        .send_packet(
            0x07,
            RegistryDataPacket::from(REGISTRIES.frog_variant.clone()),
        )
        .await;
    client
        .send_packet(
            0x07,
            RegistryDataPacket::from(REGISTRIES.painting_variant.clone()),
        )
        .await;
    client
        .send_packet(
            0x07,
            RegistryDataPacket::from(REGISTRIES.pig_variant.clone()),
        )
        .await;
    client
        .send_packet(
            0x07,
            RegistryDataPacket::from(REGISTRIES.wolf_sound_variant.clone()),
        )
        .await;
    client
        .send_packet(
            0x07,
            RegistryDataPacket::from(REGISTRIES.wolf_variant.clone()),
        )
        .await;
    client
        .send_packet(
            0x07,
            RegistryDataPacket::from(REGISTRIES.damage_type.clone()),
        )
        .await;
    client
        .send_packet(
            0x07,
            RegistryDataPacket::from(REGISTRIES.dimension_type.clone()),
        )
        .await;
    client
        .send_packet(0x07, RegistryDataPacket::from(REGISTRIES.biome.clone()))
        .await;

    client.send_packet(0x03, FinishConfigPacket {}).await;
}

async fn handle_cookie_response(client: Arc<ClientConnection>) {
    let _ = client;
    todo!()
}

async fn handle_plugin_message(client: Arc<ClientConnection>, packet: ClientPluginMessagePacket) {
    log::trace!("{:?}", &packet);
    let _ = client;
    let _ = packet;
}

async fn handle_acknowledge_finish_config(
    client: Arc<ClientConnection>,
    packet: AcknowledgeFinishConfigPacket,
) {
    log::trace!("{:?}", &packet);
    *client.state.lock().await = ProtocolState::Play;

    {
        let mut players = client.server.players.lock().await;
        players.push(Arc::new(Player::new(client.clone())));
    }

    client
        .send_packet(
            0x2B,
            LoginPacket {
                entity_id: 0,
                is_hardcore: false,
                dimension_names: vec!["minecraft:overworld".to_owned()],
                max_players: 20,
                view_distance: 16,
                simulation_distance: 16,
                reduced_debug_info: false,
                enable_respawn_screen: true,
                do_limited_crafting: false,
                dimension_type: REGISTRIES
                    .dimension_type
                    .get_id("minecraft:overworld".to_owned())
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
            },
        )
        .await;

    client
        .send_packet(
            0x41,
            SyncPlayerPositionPacket {
                teleport_id: 0.into(),
                x: 0.5,
                y: 71.,
                z: 0.5,
                velocity_x: 0.,
                velocity_y: 0.,
                velocity_z: 0.,
                yaw: 0.,
                pitch: 0.,
                flags: 0,
            },
        )
        .await;

    let game_profile = client.game_profile.lock().await.clone().unwrap();

    client
        .send_packet(
            0x3F,
            PlayerInfoUpdatePacket {
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
            },
        )
        .await;

    client
        .send_packet(
            0x22,
            GameEventPacket {
                event: 13,
                value: 0.,
            },
        )
        .await;

    client
        .send_packet(
            0x57,
            SetCenterChunkPacket {
                chunk_x: 0.into(),
                chunk_z: 0.into(),
            },
        )
        .await;

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

    for cx in -16..16 {
        for cz in -16..16 {
            let chunk = world.get_chunk(cx, cz).unwrap();
            client
                .send_packet::<ChunkDataAndUpdateLightPacket>(0x27, chunk.clone().into())
                .await;
        }
    }
}

async fn handle_keep_alive(client: Arc<ClientConnection>) {
    let _ = client;
    todo!()
}

async fn handle_pong(client: Arc<ClientConnection>) {
    let _ = client;
    todo!()
}

async fn handle_resource_pack_response(client: Arc<ClientConnection>) {
    let _ = client;
    todo!()
}

async fn handle_client_known_packs(client: Arc<ClientConnection>, packet: ClientKnownPacksPacket) {
    let _ = client;
    let _ = packet;
}

async fn handle_custom_click_action(client: Arc<ClientConnection>) {
    let _ = client;
    todo!()
}
