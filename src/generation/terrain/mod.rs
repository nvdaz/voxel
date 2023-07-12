use crate::prelude::*;

pub mod flat;
pub mod standard;

pub trait TerrainGenerator: Send + Sync {
    fn generate_heightmap(&self, origin: IVec2) -> Heightmap;
    fn generate_terrain(&self, origin: IVec3, heightmap: &Heightmap, chunk: &mut VoxelChunk);
}
