use std::cmp;

use crate::{
    player::PlayerCamera,
    prelude::*,
    render::mesh::MeshChunkQueue,
    world::chunk::{Chunk, ChunkEntityMap},
};
use bevy::{
    math::Vec3Swizzles,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future::{block_on, poll_once};
use ilattice::prelude::Extent;

use super::{
    heightmap::{
        generate_heightmap, GenerateHeightmapResult, HeightmapGenerationResultCache,
        HeightmapGenerationTaskCache,
    },
    GenerationSettings,
};

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

#[derive(Clone, Copy)]
pub struct GenerateChunk(IVec3);

impl From<IVec3> for GenerateChunk {
    fn from(value: IVec3) -> Self {
        Self(value)
    }
}

impl From<GenerateChunk> for IVec3 {
    fn from(value: GenerateChunk) -> Self {
        value.0
    }
}

pub type ChunkGenerationQueue = DistanceOrderedQueue<GenerateChunk, IVec3>;

#[derive(Component)]
pub struct ChunkGenerationTask {
    task: Task<VoxelChunk>,
}

async fn generate_chunk_impl(origin: IVec3, heightmap: GenerateHeightmapResult) -> VoxelChunk {
    let heightmap = match heightmap {
        GenerateHeightmapResult::CacheHit(heightmap) => heightmap,
        GenerateHeightmapResult::Loading(future) => future.await,
    };

    let mut voxels = VoxelBuffer::new();
    for (offset, height) in heightmap.iter() {
        let local_height = (height - (origin.y * CHUNK_SIZE as i32)).min(PADDED_CHUNK_SIZE as i32);
        let extent = Extent::from_min_and_shape(
            offset.extend_y(0),
            UVec3::new(1, local_height.max(0) as u32, 1),
        );
        voxels.fill_extent(extent, Voxel(2));
        if local_height > 0 && local_height < PADDED_CHUNK_SIZE as i32 {
            *voxels.voxel_at_mut(offset.extend_y(local_height as u32)) = Voxel(3);
        }

        if height < 0 && local_height > 0 && local_height < PADDED_CHUNK_SIZE as i32 {
            let extent = Extent::from_min_and_shape(
                offset.extend_y(local_height as u32),
                UVec3::new(1, PADDED_CHUNK_SIZE - local_height as u32 - 1, 1),
            );
            voxels.fill_extent(extent, Voxel(1));
        }
    }

    VoxelChunk { voxels }
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
    mut queue: ResMut<ChunkGenerationQueue>,
    mut tasks_cache: ResMut<HeightmapGenerationTaskCache>,
    mut results_cache: ResMut<HeightmapGenerationResultCache>,
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

    while let Some(GenerateChunk(origin)) = queue.pop() {
        let heightmap_result =
            generate_heightmap(&mut tasks_cache, &mut results_cache, origin.xz());

        if let Some(&entity) = entity_map.map.get(&origin) {
            let task = thread_pool.spawn(generate_chunk_impl(origin, heightmap_result));
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
