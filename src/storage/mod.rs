pub mod array_buffer;
pub mod chunk;
pub mod heightmap;
pub mod voxel;
pub mod voxel_world;

pub use array_buffer::*;
pub use chunk::*;
pub use heightmap::*;
pub use voxel::*;
pub use voxel_world::*;

use crate::prelude::*;

pub struct StoragePlugin;

impl Plugin for StoragePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoxelWorld>();
    }
}
