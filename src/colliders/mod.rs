mod r#box;
mod sphere;

pub use r#box::*;
pub use sphere::*;

use bevy::{prelude::*, reflect::TypeUuid};
use crate::{components::Aabb, prelude::Velocity};
use enum_dispatch::*;

#[derive(Debug, TypeUuid)]
#[uuid = "241fb60c-c542-4043-a574-a8b28bb3761d"]
#[enum_dispatch]
pub enum Collider {
    Sphere(Sphere),
    Box(Box),
}

impl Collider {
    pub fn new_sphere(radius: f32) -> Self {
        Collider::Sphere(Sphere::new(radius))
    }

    pub fn new_box(x_length: f32, y_length: f32, z_length: f32) -> Self {
        Collider::Box(Box::new(Vec3::new(x_length, y_length, z_length)))
    }
}

impl Default for Collider {
    fn default() -> Self {
        Collider::Sphere(Sphere::default())
    }
}

#[enum_dispatch(Collider)]
pub trait Collidable {
    fn get_center_of_mass(&self) -> Vec3;
    
    // See https://en.wikipedia.org/wiki/List_of_moments_of_inertia
    fn get_inertia_tensor(&self, mass: f32) -> Mat3;
    fn get_aabb(&self) -> Aabb;
    fn update_aabb(&self, aabb: &mut Aabb, trans: &Transform, velocity: &Velocity, factor: f32);
    fn get_support(&self, trans: &Transform, dir: Vec3, bias: f32) -> Vec3;
    fn fastest_linear_speed(&self, angular_velocity: Vec3, dir: Vec3) -> f32;

    /// Note: Ray must already be converted to object space
    fn intersect(&self, ray: &mut Ray) -> Option<f32>;
}

/// Find the point in the furthest in direction
// used by cube and convex
fn find_support_point(verts: &[Vec3], dir: Vec3, pos: Vec3, orient: Quat, bias: f32) -> Vec3 {
    let mut max_pt = (orient * verts[0]) + pos;
    let mut max_dist = dir.dot(max_pt);
    for pt in &verts[1..] {
        let pt = (orient * *pt) + pos;
        let dist = dir.dot(pt);
        if dist > max_dist {
            max_dist = dist;
            max_pt = pt;
        }
    }

    let norm = dir.normalize() * bias;

    max_pt + norm
}

// used by cube and convex
fn fastest_linear_speed(
    verts: &[Vec3],
    angular_velocity: Vec3,
    center_of_mass: Vec3,
    dir: Vec3,
) -> f32 {
    let mut max_speed = 0.0;
    for pt in verts {
        let r = *pt - center_of_mass;
        let linear_velocity = angular_velocity.cross(r);
        let speed = dir.dot(linear_velocity);
        if speed > max_speed {
            max_speed = speed;
        }
    }
    max_speed
}
