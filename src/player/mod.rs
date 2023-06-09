use crate::{generation::chunk::ChunkGenerationQueue, prelude::*, world::chunk::LoadChunkQueue};
use bevy::{input::mouse::MouseMotion, window::PrimaryWindow};
use bevy_dolly::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DollyCursorGrab)
            .add_startup_system(setup)
            .add_systems((
                update_camera,
                Dolly::<PlayerCamera>::update_active,
                load_chunks,
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
}

fn setup(mut commands: Commands) {
    let transform = Transform::from_xyz(2.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn(PlayerCameraBundle {
        player_camera: PlayerCamera,
        rig: Rig::builder()
            .with(Fpv::from_position_target(transform))
            .build(),
        camera: Camera3dBundle {
            transform,
            ..default()
        },
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
    let boost_mult = 5.0f32;
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
        1.
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
    player_transform: Query<&GlobalTransform, With<PlayerCamera>>,
    mut chunk_queue: ResMut<LoadChunkQueue>,
) {
    let origin = player_transform.single().translation();
    for x in -16..16 {
        for y in -4..4 {
            for z in -16..16 {
                let chunk = (origin / 32.0).as_ivec3() + IVec3::new(x, y, z);

                chunk_queue.queue.insert(chunk);
            }
        }
    }
}