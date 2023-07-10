use crate::{
    prelude::*,
    render::RenderSettings,
    world::chunk::{ChunkEntityMap, DropChunkQueue, LoadChunkQueue},
};
use bevy::{input::mouse::MouseMotion, window::PrimaryWindow};
use bevy_atmosphere::prelude::{AtmosphereCamera, AtmospherePlugin};
use bevy_dolly::prelude::*;
use ilattice::prelude::Extent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DollyCursorGrab)
            .add_plugin(AtmospherePlugin)
            .add_startup_system(setup)
            .add_systems((
                update_camera,
                Dolly::<PlayerCamera>::update_active,
                load_chunks,
                drop_chunks,
            ));
    }
}

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Bundle)]
pub struct PlayerCameraBundle {
    player_camera: PlayerCamera,
    rig: Rig,
    camera: Camera3dBundle,
    atmosphere_camera: AtmosphereCamera,
}

fn setup(mut commands: Commands, render_settings: Res<RenderSettings>) {
    let transform = Transform::from_xyz(2.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn(PlayerCameraBundle {
        player_camera: PlayerCamera,
        rig: Rig::builder()
            .with(Fpv::from_position_target(transform))
            .build(),
        camera: Camera3dBundle {
            transform,
            projection: Projection::Perspective(PerspectiveProjection {
                far: render_settings.view_radius.as_vec3().length() * CHUNK_SIZE as f32,
                ..default()
            }),
            ..default()
        },
        atmosphere_camera: AtmosphereCamera::default(),
    });
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

    let boost: f32 = if keys.pressed(KeyCode::LShift) {
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
) {
    let view_distance = render_settings.view_radius;
    let center = player_transform.single().translation();
    for offset in
        Extent::from_min_and_shape(-view_distance.as_ivec3(), view_distance.as_ivec3() * 2).iter3()
    {
        let chunk = center.as_ivec3() / CHUNK_SIZE as i32 + offset;

        chunk_queue.push(chunk);
    }
}

fn drop_chunks(
    render_settings: Res<RenderSettings>,
    player_transform: Query<&GlobalTransform, With<PlayerCamera>>,
    entity_map: Res<ChunkEntityMap>,
    mut drop_chunk_queue: ResMut<DropChunkQueue>,
) {
    let view_distance = render_settings.view_radius;
    let center = player_transform.single().translation();
    for &offset in entity_map.keys() {
        let distance = (center.as_ivec3() / CHUNK_SIZE as i32 - offset).abs();
        if distance
            .cmpgt(view_distance.as_ivec3() + IVec3::splat(render_settings.drop_padding as i32))
            .any()
        {
            drop_chunk_queue.push(offset);
        }
    }
}
