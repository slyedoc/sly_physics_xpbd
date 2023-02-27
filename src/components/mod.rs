mod aabb;
pub use aabb::Aabb;
use bevy::prelude::*;

use crate::{colliders::Collider};

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct PrevPos(pub Vec3);

#[derive(Component,Reflect, Debug, Default)]
#[reflect(Component)]
pub struct PrevRot(pub Quat);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.) // Default to 1 kg
    }
}

#[derive(Component,Reflect, Debug, Deref, DerefMut)]
#[reflect(Component)]
pub struct InverseMass(pub f32);

impl Default for InverseMass {
    fn default() -> Self {
        Self(1.) // Default to 1 kg
    }
}


#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Velocity {
    pub linear: Vec3,
    #[allow(dead_code)]
    pub angular: Vec3,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct PreSolveVelocity {
    pub(crate) linear: Vec3,
    #[allow(dead_code)]
    pub(crate) angular: Vec3,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Restitution(pub f32);

impl Default for Restitution {
    fn default() -> Self {
        Self(0.3)
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct InverseInertiaTensor(pub Mat3);

#[derive(Component, Reflect, Debug, PartialEq, Eq)]
#[reflect(Component)]
pub enum PhysicsMode {
    Dynamic,
    Static,
}

impl Default for PhysicsMode {
    fn default() -> Self {
        Self::Dynamic
    }
}

#[derive(Bundle, Default)]
pub struct PhysicsBundle {    
    pub mode: PhysicsMode,
    pub mass: Mass,    
    pub collider: Handle<Collider>,
    pub velocity: Velocity,    
    pub restitution: Restitution,

    // Should not be set by user
    pub inverse_mass: InverseMass,
    pub inverse_inertia_tensor: InverseInertiaTensor,
    pub aabb: Aabb,
    pub prev_pos: PrevPos,
    pub prev_rot: PrevRot,
    pub pre_solve_velocity: PreSolveVelocity,
}