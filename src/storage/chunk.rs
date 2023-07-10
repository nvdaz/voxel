use ndshape::{ConstShape2u32, ConstShape3u32};

use crate::prelude::*;

pub const CHUNK_SIZE: u32 = 64;
pub const PADDED_CHUNK_SIZE: u32 = CHUNK_SIZE + 2;

pub type ChunkShape = ConstShape3u32<PADDED_CHUNK_SIZE, PADDED_CHUNK_SIZE, PADDED_CHUNK_SIZE>;
pub type FlatChunkShape = ConstShape2u32<PADDED_CHUNK_SIZE, PADDED_CHUNK_SIZE>;

pub const CHUNK_SHAPE: ChunkShape = ChunkShape {};
pub const FLAT_CHUNK_SHAPE: FlatChunkShape = FlatChunkShape {};

#[derive(Default)]
pub struct VoxelChunk {
    pub voxels: VoxelBuffer,
}
