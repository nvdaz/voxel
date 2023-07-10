use std::sync::Arc;

use bevy::math::Vec3Swizzles;
use futures_util::FutureExt;

use crate::prelude::*;

use self::{standard::StandardTerrainGenerator};

pub mod flat;
pub mod standard;

pub trait TerrainGenerator: Send + Sync {
    fn generate_heightmap(&self, origin: IVec2) -> Heightmap;
    fn generate_terrain(&self, origin: IVec3, heightmap: &Heightmap, chunk: &mut VoxelChunk);
}

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
