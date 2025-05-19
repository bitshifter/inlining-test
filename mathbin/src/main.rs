use rand;
use mathlib::*;

#[inline(never)]
fn vec3_add(a: Vec3, b: Vec3) -> Vec3 {
    a + b
}

#[inline(never)]
fn vec3a_add(a: Vec3A, b: Vec3A) -> Vec3A {
    a + b
}

fn main() {
    {
        let a = Vec3::new(rand::random(), rand::random(), rand::random());
        let b = Vec3::new(rand::random(), rand::random(), rand::random());
        let c = vec3_add(a, b);
        dbg!(c);
    }
    {
        let a = Vec3A::new(rand::random(), rand::random(), rand::random());
        let b = Vec3A::new(rand::random(), rand::random(), rand::random());
        let c = vec3a_add(a, b);
        dbg!(c);
    }
}
