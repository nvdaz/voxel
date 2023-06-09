use crate::prelude::*;

pub trait UVec2Ext {
    fn extend_y(self, y: u32) -> UVec3;
}

impl UVec2Ext for UVec2 {
    fn extend_y(self, y: u32) -> UVec3 {
        UVec3::new(self.x, y, self.y)
    }
}
