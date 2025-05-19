use core::arch::x86_64::*;
use core::ops::Add;

#[derive(Copy, Clone, Debug)]
pub struct Vec3A(__m128);
// pub struct Vec3 {
//     x: f32,
//     y: f32,
//     z: f32,
// }

#[repr(C)]
union UnionCast {
    a: [f32; 4],
    v: Vec3A,
}

impl Vec3A {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        unsafe { UnionCast { a: [x, y, z, z] }.v }
    }
}

impl Add for Vec3A {
    type Output = Self;
    #[cfg_attr(feature = "inline", inline)]
    fn add(self, rhs: Self) -> Self::Output {
        Self(unsafe { _mm_add_ps(self.0, rhs.0) })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
