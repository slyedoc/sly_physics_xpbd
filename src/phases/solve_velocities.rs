use bevy::prelude::*;
use crate::{components::*, SubstepContacts};

pub fn solve_vel(
    query: Query<(
        &mut Velocity,
        &PreSolveVelocity,
        &InverseMass,
        &Restitution,
    )>,
    contacts: Res<SubstepContacts>,
) {
    for c in contacts.iter() {

        let (
            (mut vel_a, pre_solve_vel_a, inv_mass_a, restitution_a),
            (mut vel_b, pre_solve_vel_b, inv_mass_b, restitution_b),
        ) = unsafe {
            // Ensure safety
            assert!(c.entity_a != c.entity_b);
            (
                query.get_unchecked(c.entity_a).unwrap(),
                query.get_unchecked(c.entity_b).unwrap(),
            )
        };
        
        // Make sure velocities are reflected and restitution/friction calculated
        let pre_solve_relative_vel = pre_solve_vel_a.linear - pre_solve_vel_b.linear;
        let pre_solve_normal_vel = pre_solve_relative_vel.dot(c.normal);

        let relative_vel = vel_a.linear - vel_b.linear;
        let normal_vel = relative_vel.dot(c.normal);
        // averaging restitution
        let restitution = (restitution_a.0 + restitution_b.0) / 2.;

        let w_sum = inv_mass_a.0 + inv_mass_b.0;

        vel_a.linear += c.normal * (-normal_vel - restitution * pre_solve_normal_vel) * inv_mass_a.0 / w_sum;
        vel_b.linear -= c.normal * (-normal_vel - restitution * pre_solve_normal_vel) * inv_mass_b.0 / w_sum;
    }
}
