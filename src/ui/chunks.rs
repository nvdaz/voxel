use crate::{
    generation::chunk::{ChunkGenerationQueue, ChunkGenerationTask},
    player::PlayerCamera,
    prelude::*,
    render::mesh::chunk::{MeshChunkQueue, MeshChunkTask},
};

#[derive(Component)]
struct ChunksText;

#[derive(Component)]
struct GeneratingChunksText;

#[derive(Component)]
struct MeshingChunksText;

#[derive(Component)]
struct PositionText;

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Chunks: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 32.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(64.0),
            ..default()
        }),
        ChunksText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Generating Chunks: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 32.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(96.0),
            ..default()
        }),
        GeneratingChunksText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Meshing Chunks: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 32.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(128.0),
            ..default()
        }),
        MeshingChunksText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Position: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 32.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(160.0),
            ..default()
        }),
        PositionText,
    ));
}

fn update_chunks_system(world: Res<VoxelWorld>, mut text: Query<&mut Text, With<ChunksText>>) {
    for mut text in &mut text {
        let chunks = world.len();
        text.sections[1].value = format!("{chunks}");
    }
}

fn update_generating_chunks_system(
    tasks: Query<&ChunkGenerationTask>,
    queue: Res<ChunkGenerationQueue>,
    mut text: Query<&mut Text, With<GeneratingChunksText>>,
) {
    for mut text in &mut text {
        let tasks_len = tasks.iter().len();
        let queue_len = queue.len();
        text.sections[1].value = format!("{}", tasks_len + queue_len);
    }
}

fn update_meshing_chunks_system(
    tasks: Query<&MeshChunkTask>,
    queue: Res<MeshChunkQueue>,
    mut text: Query<&mut Text, With<MeshingChunksText>>,
) {
    for mut text in &mut text {
        let tasks_len = tasks.iter().len();
        let queue_len = queue.len();
        text.sections[1].value = format!("{}", tasks_len + queue_len);
    }
}

fn update_position_system(
    player: Query<&GlobalTransform, With<PlayerCamera>>,
    mut text: Query<&mut Text, With<PositionText>>,
) {
    for mut text in &mut text {
        let Vec3 { x, y, z } = player.single().translation();
        text.sections[1].value = format!("{x:.0}, {y:.0}, {z:.0}");
    }
}

pub struct ChunksMenuPlugin;

impl Plugin for ChunksMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                update_chunks_system,
                update_generating_chunks_system,
                update_meshing_chunks_system,
                update_position_system,
            ),
        );
    }
}
