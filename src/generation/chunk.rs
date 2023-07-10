use std::cmp;

use crate::{
    player::PlayerCamera,
    prelude::*,
    render::mesh::MeshChunkQueue,
    world::chunk::{Chunk, ChunkEntityMap},
};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future::{block_on, poll_once};

use super::GenerationSettings;

pub struct ChunkGenerationPlugin;

impl Plugin for ChunkGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkGenerationQueue>().add_systems((
            handle_queue,
            handle_tasks,
            update_center,
        ));
    }
}

pub struct GenerateChunk;

pub type ChunkGenerationQueue = DistanceOrderedQueue<IVec3, GenerateChunk>;

#[derive(Component)]
pub struct ChunkGenerationTask {
    task: Task<VoxelChunk>,
}

fn update_center(
    camera: Query<&GlobalTransform, With<PlayerCamera>>,
    mut queue: ResMut<ChunkGenerationQueue>,
) {
    let camera = camera.single();

    let center = camera.translation().as_ivec3() / CHUNK_SIZE as i32;

    queue.update_center(center)
}

fn handle_queue(
    mut commands: Commands,
    entity_map: Res<ChunkEntityMap>,
    world: Res<VoxelWorld>,
    mut queue: ResMut<ChunkGenerationQueue>,
    tasks: Query<Entity, With<ChunkGenerationTask>>,
    settings: Res<GenerationSettings>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    let mut i = 0;
    let to = cmp::min(
        settings
            .max_generation_tasks
            .saturating_sub(tasks.iter().len()),
        queue.len(),
    );

    while let Some(origin) = queue.pop() {
        if let Some(entity) = entity_map.get(&origin) {
            let generator = world.get_generator();
            let task = thread_pool.spawn(async move { generator.generate_chunk(origin).await });
            commands.entity(entity).insert(ChunkGenerationTask { task });
        }

        i += 1;
        if i >= to {
            break;
        }
    }
}

fn handle_tasks(
    mut commands: Commands,
    mut voxel_world: ResMut<VoxelWorld>,
    mut tasks: Query<(Entity, &Chunk, &mut ChunkGenerationTask)>,
    mut mesh_queue: ResMut<MeshChunkQueue>,
) {
    for (entity, chunk, mut task) in &mut tasks {
        if let Some(voxel_chunk) = block_on(poll_once(&mut task.task)) {
            voxel_world.insert(chunk.position, voxel_chunk);
            commands.entity(entity).remove::<ChunkGenerationTask>();
            mesh_queue.push(chunk.position);
        }
    }
}
