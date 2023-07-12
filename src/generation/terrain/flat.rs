use ilattice::prelude::Extent;

use crate::prelude::*;

use super::TerrainGenerator;

#[derive(Default)]
pub struct FlatTerrainGenerator {
    height: i32,
}

impl TerrainGenerator for FlatTerrainGenerator {
    fn generate_heightmap(&self, _: IVec2) -> Heightmap {
        let mut heightmap = Heightmap::new();

        for (_, height) in heightmap.iter_mut() {
            *height = self.height;
        }

        heightmap
    }

    fn generate_terrain(&self, origin: IVec3, heightmap: &Heightmap, chunk: &mut VoxelChunk) {
        for (position, height) in heightmap.iter() {
            let local_height =
                (height - (origin.y * CHUNK_SIZE as i32)).clamp(0, PADDED_CHUNK_SIZE as i32);

            chunk.voxels.fill_extent(
                Extent::from_min_and_shape(
                    position.extend_y(0),
                    UVec3::new(1, local_height as u32, 1),
                ),
                Voxel(3),
            );
        }
    }
}
