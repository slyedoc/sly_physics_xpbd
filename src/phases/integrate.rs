use crate::{colliders::*, components::*, Gravity, PhysicsConfig};
use bevy::prelude::*;

pub fn integrate(
    mut query: Query<(
        &mut Transform,
        &mut PrevPos,
        &mut PrevRot,
        &mut Velocity,

        &InverseMass,
        &InertiaTensor,
        &InverseInertiaTensor,
        &PhysicsMode,
        &Handle<Collider>,
    )>,
    gravity: Res<Gravity>,
    config: Res<PhysicsConfig>,
    colliders: Res<Assets<Collider>>,
) {
    for (
        mut trans,
        mut prev_pos,
        mut prev_rot,
        mut vel,        
        inv_mass,
        inertia_tensor,
        inv_inertia_tensor,
        mode,
        collider_handle,
    ) in query.iter_mut()
    {
        if mode == &PhysicsMode::Static {
            continue;
        }

        // position
        
        prev_pos.0 = trans.translation;
        prev_rot.0 = trans.rotation;

        let gravitation_force = inv_mass.0 * gravity.0;        
        let external_force = gravitation_force;
        let external_torque = Vec3::ZERO;

        vel.linear += config.sub_delta_time * external_force;
        trans.translation += config.sub_delta_time * vel.linear;
        

        // rotation
        let collider = colliders.get(collider_handle).unwrap();
        
        let change = config.sub_delta_time * inv_inertia_tensor.0 * (external_torque - vel.angular.cross( inertia_tensor.0 * vel.angular));        
        vel.angular += change;

        // USE_QUATERNIONS_LINEARIZED_FORMULAS
        let aux = Quat::from_xyzw( vel.angular.x, vel.angular.y, vel.angular.z, 0.0);
        let q = aux * trans.rotation;
        trans.rotation.x +=  config.sub_delta_time * 0.5 * q.x;
        trans.rotation.y +=  config.sub_delta_time * 0.5 * q.y;
        trans.rotation.z +=  config.sub_delta_time * 0.5 * q.y;
        trans.rotation.w +=  config.sub_delta_time * 0.5 * q.w;
        trans.rotation = trans.rotation.normalize();
    }
}
