mod aabb;
pub use aabb::Aabb;
use bevy::prelude::*;

use crate::{colliders::Collider, DELTA_TIME};


#[derive(Component, Debug, Default)]
pub struct PrevPos(pub Vec3);

#[derive(Component, Debug, Default)]
pub struct PrevRot(pub Vec3);

#[derive(Component, Debug)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.) // Default to 1 kg
    }
}

#[derive(Component, Debug, Default)]
pub struct Velocity {
    pub linear: Vec3,
    #[allow(dead_code)]
    pub angular: Vec3,
}

#[derive(Component, Debug, Default)]
pub struct PreSolveVelocity {
    pub(crate) linear: Vec3,
    #[allow(dead_code)]
    pub(crate) angular: Vec3,
}

#[derive(Component, Debug)]
pub struct Restitution(pub f32);

impl Default for Restitution {
    fn default() -> Self {
        Self(0.3)
    }
}

#[derive(Bundle, Default)]
pub struct PhysicsBundle {    
    pub prev_pos: PrevPos,    
    pub prev_rot: PrevRot,
    pub mass: Mass,
    pub aabb: Aabb,
    pub collider: Handle<Collider>,
    pub velocity: Velocity,
    pub pre_solve_velocity: PreSolveVelocity,
    pub restitution: Restitution,
}

// marker component
#[derive(Component, Default, Debug)]
pub struct PhysicsStatic;

#[derive(Bundle, Default)]
pub struct PhysicsStaticBundle {
    pub aabb: Aabb,
    pub collider: Handle<Collider>,
    pub restitution: Restitution,
    pub physics_static: PhysicsStatic,
}
