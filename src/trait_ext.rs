use std::ops::Sub;

use crate::prelude::*;

pub trait UVec2Ext {
    fn extend_y(self, y: u32) -> UVec3;
}

impl UVec2Ext for UVec2 {
    fn extend_y(self, y: u32) -> UVec3 {
        UVec3::new(self.x, y, self.y)
    }
}

pub trait LengthSquared {
    type Output;

    fn length_squared(self) -> Self::Output;
}

impl LengthSquared for IVec2 {
    type Output = i32;

    fn length_squared(self) -> Self::Output {
        self.dot(self)
    }
}

impl LengthSquared for IVec3 {
    type Output = i32;

    fn length_squared(self) -> Self::Output {
        self.dot(self)
    }
}

pub trait DistanceSquared: LengthSquared {
    fn distance_squared(self, rhs: Self) -> <Self as LengthSquared>::Output;
}

impl<T, O> DistanceSquared for T
where
    T: LengthSquared<Output = O> + Sub<T>,
    <T as Sub<T>>::Output: LengthSquared<Output = O>,
{
    fn distance_squared(self, rhs: Self) -> <Self as LengthSquared>::Output {
        (self - rhs).length_squared()
    }
}

pub trait DistanceOrd: DistanceSquared {
    type AsOrd: Ord;

    fn distance_ord(self, center: Self) -> Self::AsOrd;
}

impl DistanceOrd for IVec3 {
    type AsOrd = (i32, [i32; 3]);

    fn distance_ord(self, center: Self) -> Self::AsOrd {
        let distance_squared = self.distance_squared(center);
        let location = self.into();

        (distance_squared, location)
    }
}
