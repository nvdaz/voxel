use block_mesh::{MergeVoxel, VoxelVisibility};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Voxel(pub u16);

impl Voxel {
    pub const EMPTY: Self = Self(0);
}

impl Default for Voxel {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl block_mesh::Voxel for Voxel {
    fn get_visibility(&self) -> VoxelVisibility {
        match *self {
            Self::EMPTY => VoxelVisibility::Empty,
            _ => VoxelVisibility::Opaque,
        }
    }
}

impl MergeVoxel for Voxel {
    type MergeValue = u16;

    fn merge_value(&self) -> Self::MergeValue {
        self.0
    }
}
