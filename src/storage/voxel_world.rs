use std::sync::{Arc, RwLock};

use bevy::utils::HashMap;

use crate::{generation::terrain::ChunkGenerator, prelude::*};

#[derive(Resource)]
pub struct VoxelWorld {
    chunks: HashMap<IVec3, Arc<RwLock<VoxelChunk>>>,
    generator: Arc<ChunkGenerator>,
}

impl Default for VoxelWorld {
    fn default() -> Self {
        Self {
            chunks: HashMap::new(),
            generator: Arc::new(ChunkGenerator::default()),
        }
    }
}

impl VoxelWorld {
    pub fn get_generator(&self) -> Arc<ChunkGenerator> {
        self.generator.clone()
    }

    pub fn get(&self, position: &IVec3) -> Option<Arc<RwLock<VoxelChunk>>> {
        self.chunks.get(position).cloned()
    }

    pub fn insert(&mut self, position: IVec3, chunk: VoxelChunk) {
        self.chunks.insert(position, Arc::new(RwLock::new(chunk)));
    }

    pub fn contains(&self, position: &IVec3) -> bool {
        self.chunks.contains_key(position)
    }

    pub fn remove(&mut self, position: &IVec3) -> Option<Arc<RwLock<VoxelChunk>>> {
        self.chunks.remove(position)
    }

    pub fn len(&self) -> usize {
        self.chunks.len()
    }
}
