use ndshape::Shape;

use crate::prelude::*;

/// (0, 0) is equivalent to (-1, -1) world position.
#[derive(Clone)]
pub struct Heightmap {
    data: [i32; PADDED_CHUNK_SIZE.pow(2) as usize],
}

impl Default for Heightmap {
    fn default() -> Self {
        Self::new()
    }
}

impl Heightmap {
    pub fn new() -> Self {
        Self {
            data: [0; PADDED_CHUNK_SIZE.pow(2) as usize],
        }
    }

    #[inline]
    pub fn get(&self, position: UVec2) -> i32 {
        self.data[FLAT_CHUNK_SHAPE.linearize(position.to_array()) as usize]
    }

    #[inline]
    pub fn get_mut(&mut self, position: UVec2) -> &mut i32 {
        &mut self.data[FLAT_CHUNK_SHAPE.linearize(position.to_array()) as usize]
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (UVec2, i32)> + '_ {
        self.data.iter().enumerate().map(|(i, height)| {
            (
                UVec2::from_slice(&FLAT_CHUNK_SHAPE.delinearize(i as u32)),
                *height,
            )
        })
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (UVec2, &mut i32)> + '_ {
        self.data.iter_mut().enumerate().map(|(i, height)| {
            (
                UVec2::from_slice(&FLAT_CHUNK_SHAPE.delinearize(i as u32)),
                height,
            )
        })
    }
}
