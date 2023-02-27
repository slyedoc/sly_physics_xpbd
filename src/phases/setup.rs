use bevy::prelude::*;

use crate::{colliders::*, components::*, PhysicsConfig};

pub fn setup_prev_pos(
    mut query: Query<(&Transform, &mut Velocity, &mut PrevPos, &PhysicsMode), Added<Velocity>>,
    config: Res<PhysicsConfig>,
) {
    for (trans, mut vel, mut prev_pos, mode) in query.iter_mut() {
        // clear any velocity on static objects
        match mode {
            PhysicsMode::Static => {
                vel.linear = Vec3::ZERO;
            }
            _ => {}
        }
        prev_pos.0 = trans.translation - vel.linear * config.sub_delta_time;
    }
}

pub fn setup_inverse_mass_and_inverse_inertia_tensor(
    mut query: Query<(&mut Mass, &mut InverseMass,  &mut InverseInertiaTensor,  &Handle<Collider>, &PhysicsMode), Changed<Mass>>,
    colliders: Res<Assets<Collider>>,
) {
    // setup inverse mass and mass
    for (mut mass, mut inv_mass, mut inv_inertia_tensor, collider_handle, option) in query.iter_mut() {
        match option {
            PhysicsMode::Dynamic => {
                inv_mass.0 = 1. / mass.0;
            }
            PhysicsMode::Static => {
                mass.0 = f32::INFINITY;
                inv_mass.0 = 0.;
            }
        }
        let collider = colliders.get(collider_handle).unwrap(); 
        let inertia_tensor = collider.get_inertia_tensor();       
        inv_inertia_tensor.0 = inertia_tensor.inverse() * inv_mass.0;
    }
}

pub fn update_aabb(
    mut query: Query<(&Transform, &mut Aabb, &Handle<Collider>, &Velocity)>,
    colliders: Res<Assets<Collider>>,
    config: Res<PhysicsConfig>,
) {
    let k = config.k; // safety margin multiplier bigger than 1 to account for sudden accelerations
    let safety_margin_factor = k * config.delta_time;
    //let safety_margin_factor_sqr = safety_margin_factor * safety_margin_factor;

    for (trans, mut aabb, col, velocity) in query.iter_mut() {
        let collider = colliders.get(col).unwrap();
        collider.update_aabb(&mut aabb, trans, velocity, safety_margin_factor);
    }
}
