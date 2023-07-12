use std::sync::Arc;

use crate::prelude::*;

use super::chunk::ChunkGenerator;

#[derive(Default, Resource)]
pub struct VoxelWorldGenerator {
    chunk_generator: Arc<ChunkGenerator>,
}

impl VoxelWorldGenerator {
    pub fn get(&self) -> Arc<ChunkGenerator> {
        self.chunk_generator.clone()
    }
}
