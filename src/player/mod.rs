use crate::{
    prelude::*,
    render::RenderSettings,
    world::{
        chunk::{ChunkEntityMap, DropChunkQueue, LoadChunkQueue},
        heightmap::{DropHeightmapQueue, HeightmapEntityMap, LoadHeightmapQueue},
    },
};
use bevy::{
    core_pipeline::experimental::taa::TemporalAntiAliasBundle, input::mouse::MouseMotion,
    math::Vec3Swizzles, pbr::ScreenSpaceAmbientOcclusionBundle, window::PrimaryWindow,
};
use bevy_dolly::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DollyCursorGrab)
            // .add_plugin(AtmospherePlugin)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    update_camera,
                    Dolly::<PlayerCamera>::update_active,
                    load_chunks,
                    drop_chunks,
                ),
            );
    }
}

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Bundle)]
pub struct PlayerCameraBundle {
    player_camera: PlayerCamera,
    rig: Rig,
    camera: Camera3dBundle,
    // atmosphere_camera: AtmosphereCamera,
}

fn setup(mut commands: Commands, render_settings: Res<RenderSettings>) {
    let transform = Transform::from_xyz(2.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands
        .spawn(PlayerCameraBundle {
            player_camera: PlayerCamera,
            rig: Rig::builder()
                .with(Fpv::from_position_target(transform))
                .build(),
            camera: Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                transform,
                projection: Projection::Perspective(PerspectiveProjection {
                    far: render_settings.view_radius.as_vec3().length() * CHUNK_SIZE as f32,
                    ..default()
                }),
                ..default()
            },
            // atmosphere_camera: AtmosphereCamera::default(),
        })
        .insert(ScreenSpaceAmbientOcclusionBundle::default())
        .insert(TemporalAntiAliasBundle::default());
}

fn update_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut rig_query: Query<&mut Rig, With<PlayerCamera>>,
) {
    let time_delta_seconds: f32 = time.delta_seconds();
    let boost_mult = 60.0f32;
    let sensitivity = Vec2::splat(1.0);

    let mut move_vec = Vec3::ZERO;

    if keys.pressed(KeyCode::W) {
        move_vec.z -= 1.0;
    }
    if keys.pressed(KeyCode::S) {
        move_vec.z += 1.0;
    }
    if keys.pressed(KeyCode::A) {
        move_vec.x -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        move_vec.x += 1.0;
    }

    if keys.pressed(KeyCode::E) || keys.pressed(KeyCode::Space) {
        move_vec.y += 1.0;
    }
    if keys.pressed(KeyCode::Q) {
        move_vec.y -= 1.0;
    }

    let boost: f32 = if keys.pressed(KeyCode::ShiftLeft) {
        boost_mult
    } else {
        1.0
    };

    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }
    delta.x *= sensitivity.x;
    delta.y *= sensitivity.y;

    let mut rig = rig_query.single_mut();

    if let Ok(window) = windows.get_single() {
        if !window.cursor.visible {
            rig.driver_mut::<Fpv>().update_pos_rot(
                move_vec,
                delta,
                false,
                boost,
                time_delta_seconds,
            );
        }
    }
}

fn load_chunks(
    render_settings: Res<RenderSettings>,
    player_transform: Query<&GlobalTransform, With<PlayerCamera>>,
    mut chunk_queue: ResMut<LoadChunkQueue>,
    mut heightmap_queue: ResMut<LoadHeightmapQueue>,
) {
    let view_distance = render_settings.view_radius;
    let center = player_transform.single().translation();
    for x in 0..=(view_distance.x * 2) as i32 {
        for y in 0..=(view_distance.y * 2) as i32 {
            for z in 0..=(view_distance.z * 2) as i32 {
                let offset = IVec3::new(
                    x - view_distance.x as i32,
                    y - view_distance.y as i32,
                    z - view_distance.z as i32,
                );
                let position = center.as_ivec3() / CHUNK_SIZE as i32 + offset;
                chunk_queue.push(position);
            }
        }
    }

    let far_view_distance = render_settings.far_view_radius;
    for x in 0..=(far_view_distance.x * 2) as i32 {
        for y in 0..=(far_view_distance.y * 2) as i32 {
            let offset = IVec2::new(
                x - far_view_distance.x as i32,
                y - far_view_distance.y as i32,
            );
            let position = center.xz().as_ivec2() / CHUNK_SIZE as i32 + offset;
            heightmap_queue.push(position);
        }
    }
}

fn drop_chunks(
    render_settings: Res<RenderSettings>,
    player_transform: Query<&GlobalTransform, With<PlayerCamera>>,
    chunk_entity_map: Res<ChunkEntityMap>,
    heightmap_entity_map: Res<HeightmapEntityMap>,
    mut drop_chunk_queue: ResMut<DropChunkQueue>,
    mut drop_heightmap_queue: ResMut<DropHeightmapQueue>,
) {
    let center = player_transform.single().translation();

    let view_distance = render_settings.view_radius;
    for &offset in chunk_entity_map.keys() {
        let distance = (center.as_ivec3() / CHUNK_SIZE as i32 - offset).abs();
        if distance
            .cmpgt(view_distance.as_ivec3() + IVec3::splat(render_settings.drop_padding as i32))
            .any()
        {
            drop_chunk_queue.push(offset);
        }
    }

    let far_view_distance = render_settings.far_view_radius;
    for &offset in heightmap_entity_map.keys() {
        let distance = (center.xz().as_ivec2() / CHUNK_SIZE as i32 - offset).abs();
        if distance
            .cmpgt(far_view_distance.as_ivec2() + IVec2::splat(render_settings.drop_padding as i32))
            .any()
        {
            drop_heightmap_queue.push(offset);
        }
    }
}
