use ilattice::prelude::Extent;
use noise::{Clamp, Curve, Fbm, MultiFractal, NoiseFn, OpenSimplex};

use crate::prelude::*;

use super::TerrainGenerator;

#[derive(Default)]
pub struct StandardTerrainGenerator;

impl TerrainGenerator for StandardTerrainGenerator {
    fn generate_heightmap(&self, origin: IVec2) -> Heightmap {
        let simplex = Fbm::<OpenSimplex>::new(0)
            .set_octaves(4)
            .set_frequency(0.005)
            .set_persistence(0.5)
            .set_lacunarity(2.0);

        let noise = Curve::new(simplex)
            .add_control_point(-1.0, 0.0)
            .add_control_point(-0.8, 0.0)
            .add_control_point(-0.75, -0.25)
            .add_control_point(-0.7, 0.0)
            .add_control_point(0.25, 0.0)
            .add_control_point(0.5, 0.75)
            .add_control_point(1.0, 1.0);

        // let rivers_simplex = Fbm::<OpenSimplex>::new(1)
        //     .set_octaves(1)
        //     .set_frequency(0.005)
        //     .set_persistence(0.5)
        //     .set_lacunarity(2.0);

        // let rivers = Curve::new(rivers_simplex)
        //     .add_control_point(-1.0, -1.0)
        //     .add_control_point(-0.05, -1.0)
        //     .add_control_point(0.05, 0.0)
        //     .add_control_point(0.05, -1.0)
        //     .add_control_point(1.0, -1.0);

        let noise = Clamp::new(noise).set_bounds(-1.0, 1.0);
        // let rivers = Clamp::new(rivers).set_bounds(-1.0, 0.0);

        let mut heightmap = Heightmap::new();

        for (offset, height) in heightmap.iter_mut() {
            let position = (origin * CHUNK_SIZE as i32) + offset.as_ivec2();

            let dposition = position.as_dvec2().to_array();

            *height = noise.get(dposition).mul_add(100.0, 0.0) as i32;
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
