use std::{cmp, sync::Arc};

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
use futures_util::FutureExt;

use super::{
    terrain::{standard::StandardTerrainGenerator, TerrainGenerator},
    world::VoxelWorldGenerator,
    GenerationSettings,
};

pub struct ChunkGenerator {
    heightmap_cache: FutureTaskCache<IVec2, Heightmap>,
    terrain_generator: Arc<dyn TerrainGenerator>,
}

impl Default for ChunkGenerator {
    fn default() -> Self {
        Self {
            heightmap_cache: FutureTaskCache::default(),
            terrain_generator: Arc::new(StandardTerrainGenerator::default()),
        }
    }
}

impl ChunkGenerator {
    fn generate_heightmap(&self, origin: IVec2) -> FutureCacheResult<Heightmap> {
        if let Some(result) = self.heightmap_cache.get(&origin) {
            result
        } else {
            let terrain_generator = self.terrain_generator.clone();
            let future = async move { Arc::new(terrain_generator.generate_heightmap(origin)) }
                .boxed()
                .shared();

            self.heightmap_cache.insert_future(origin, future.clone());

            FutureCacheResult::Waiting(future)
        }
    }

    pub async fn generate_chunk(&self, origin: IVec3) -> VoxelChunk {
        let mut chunk = VoxelChunk::default();

        let heightmap = match self.generate_heightmap(origin.xz()) {
            FutureCacheResult::Hit(heightmap) => heightmap,
            FutureCacheResult::Waiting(future) => future.await,
        };

        self.terrain_generator
            .generate_terrain(origin, &heightmap, &mut chunk);

        chunk
    }
}

pub struct ChunkGenerationPlugin;

impl Plugin for ChunkGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkGenerationQueue>()
            .add_systems(Update, (handle_queue, handle_tasks, update_center));
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
    generator: Res<VoxelWorldGenerator>,
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
            let generator = generator.get();
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
