use bevy::prelude::*;
use crate::{components::*, PhysicsConfig};

pub fn update_vel(
    mut query: Query<(&Transform, &PrevPos, &PrevRot, &mut Velocity, &mut PreSolveVelocity)>,
    config: Res<PhysicsConfig>,
) {
    for (trans, prev_pos, prev_rot, mut vel, mut pre_solve_vel) in query.iter_mut() {

        // Storing the current velocities
        pre_solve_vel.linear = vel.linear;
        pre_solve_vel.angular = vel.angular;


        // Updating the linear velocity from the position change
        vel.linear = (trans.translation - prev_pos.0) / config.sub_delta_time;

        // Update the angular velocity based on the orientation difference
        let inv = prev_rot.0.inverse();
        let delta_q = trans.rotation * inv;
        if delta_q.w >= 0.0 {
            vel.angular = (2.0 / config.sub_delta_time) *  Vec3::new(delta_q.x, delta_q.y, delta_q.z);
        } else {
            vel.angular = (-2.0 / config.sub_delta_time) *  Vec3::new(delta_q.x, delta_q.y, delta_q.z);            
        }
        
    }
}
