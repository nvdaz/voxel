use block_mesh_pop::{MergeVoxel, MeshVoxel, VoxelVisibility};

use crate::prelude::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Voxel(pub u16);

impl Voxel {
    pub const EMPTY: Self = Self(0);

    pub fn get_color(&self) -> Color {
        match self {
            Self(1) => Color::TURQUOISE,
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

impl MeshVoxel for Voxel {
    fn get_visibility(&self) -> block_mesh_pop::VoxelVisibility {
        match *self {
            Self::EMPTY => VoxelVisibility::Empty,
            Self(1) => VoxelVisibility::Translucent,
            _ => VoxelVisibility::Opaque,
        }
    }
}

impl MergeVoxel for Voxel {
    type MergeValue = Self;
    type MergeValueFacingNeighbour = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }

    fn merge_value_facing_neighbour(&self) -> Self::MergeValueFacingNeighbour {
        *self
    }
}
