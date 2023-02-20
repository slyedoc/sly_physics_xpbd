mod broad;
mod solve_positions;

pub(crate) use broad::*;
pub(crate) use solve_positions::*;

use bevy::prelude::*;

use crate::{colliders::*, components::*, DELTA_TIME};

pub fn update_aabb(
    mut query: Query<(&GlobalTransform, &mut Aabb, &Handle<Collider>, &Velocity)>,
    colliders: Res<Assets<Collider>>,
) {
    for (trans, mut aabb, col, lin_vel) in query.iter_mut() {
        let collider = colliders.get(col).unwrap();
        *aabb = collider.get_world_aabb(trans, lin_vel, DELTA_TIME);
        //inv_trans.0 = trans.compute_matrix().inverse();
    }
}

static STATIC_VELOCITY: Velocity = Velocity {
    linear: Vec3::ZERO,
    angular: Vec3::ZERO,
};
pub fn update_aabb_static(
    mut query: Query<
        (&GlobalTransform, &mut Aabb, &Handle<Collider>),
        (Changed<Transform>, With<PhysicsStatic>),
    >,
    colliders: Res<Assets<Collider>>,
) {
    for (trans, mut aabb, col) in query.iter_mut() {
        let collider = colliders.get(col).unwrap();
        *aabb = collider.get_world_aabb(trans, &STATIC_VELOCITY, DELTA_TIME);
        //inv_trans.0 = trans.compute_matrix().inverse();
    }
}
