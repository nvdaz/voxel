use ilattice::prelude::Extent;
use ndcopy::fill3;
use ndshape::AbstractShape;

use crate::prelude::*;

pub struct VoxelBuffer {
    pub data: Box<[Voxel]>,
}

impl VoxelBuffer {
    #[inline]
    pub fn new() -> Self {
        Self {
            data: vec![Voxel::EMPTY; PADDED_CHUNK_SIZE.pow(3) as usize].into_boxed_slice(),
        }
    }

    #[inline]
    pub fn voxel_at_index(&self, index: usize) -> Voxel {
        self.data[index]
    }

    #[inline]
    pub fn voxel_at_index_ref(&self, index: usize) -> &Voxel {
        &self.data[index]
    }

    #[inline]
    pub fn voxel_at_index_mut(&mut self, index: usize) -> &mut Voxel {
        &mut self.data[index]
    }

    #[inline]
    pub fn voxel_at(&self, position: UVec3) -> Voxel {
        self.voxel_at_index(CHUNK_SHAPE.linearize(position.to_array()) as usize)
    }

    #[inline]
    pub fn voxel_at_ref(&self, position: UVec3) -> &Voxel {
        self.voxel_at_index_ref(CHUNK_SHAPE.linearize(position.to_array()) as usize)
    }

    #[inline]
    pub fn voxel_at_mut(&mut self, position: UVec3) -> &mut Voxel {
        self.voxel_at_index_mut(CHUNK_SHAPE.linearize(position.to_array()) as usize)
    }

    #[inline]
    pub fn fill_extent(&mut self, extent: Extent<UVec3>, value: Voxel) {
        fill3(
            extent.shape.to_array(),
            value,
            &mut self.data,
            &CHUNK_SHAPE,
            extent.minimum.to_array(),
        );
    }

    #[inline]
    pub fn read_data(&self) -> &[Voxel] {
        &self.data
    }
}
