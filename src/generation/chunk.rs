use crate::{
    prelude::*,
    render::mesh::MeshChunkQueue,
    world::chunk::{Chunk, ChunkEntityMap},
};
use bevy::{
    math::Vec3Swizzles,
    tasks::{AsyncComputeTaskPool, Task},
    utils::HashSet,
};
use futures_lite::future::{block_on, poll_once};
use ilattice::prelude::Extent;

use super::heightmap::{generate_heightmap, GenerateHeightmapResult, HeightmapGenerationCache};

pub struct ChunkGenerationPlugin;

impl Plugin for ChunkGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkGenerationQueue>()
            .add_systems((handle_queue, handle_tasks));
    }
}

#[derive(Default, Resource)]
pub struct ChunkGenerationQueue {
    pub queue: HashSet<IVec3>,
}

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
        voxels.fill_extent(extent, Voxel(1));
    }

    let chunk = VoxelChunk { voxels };

    chunk
}

fn handle_queue(
    mut commands: Commands,
    entity_map: Res<ChunkEntityMap>,
    mut queue: ResMut<ChunkGenerationQueue>,
    mut cache: ResMut<HeightmapGenerationCache>,
) {
    for origin in queue.queue.drain() {
        let thread_pool = AsyncComputeTaskPool::get();

        let heightmap_result = generate_heightmap(cache.as_mut(), origin.xz());

        let task = thread_pool.spawn(generate_chunk_impl(origin, heightmap_result));

        let entity = entity_map.map.get(&origin).unwrap().clone();

        commands.entity(entity).insert(ChunkGenerationTask { task });
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
            mesh_queue.queue.insert(chunk.position);
        }
    }
}
