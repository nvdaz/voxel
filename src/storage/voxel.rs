use block_mesh::{MergeVoxel, VoxelVisibility};

use crate::prelude::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Voxel(pub u16);

impl Voxel {
    pub const EMPTY: Self = Self(0);

    pub fn get_color(&self) -> Color {
        match self {
            Self(1) => Color::INDIGO,
            Self(2) => Color::DARK_GRAY,
            Self(3) => Color::GREEN,
            _ => Color::WHITE,
        }
    }
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
            Self(1) => VoxelVisibility::Translucent,
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
