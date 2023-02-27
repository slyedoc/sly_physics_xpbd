use bevy::prelude::*;

use crate::{
    colliders::*, components::*, contacts::*, intersect::*, CollisionPairs, SubstepContacts,
};

pub fn solve_pos(
    mut query: Query<(Entity, &mut Transform, &InverseMass, &Handle<Collider>)>,
    collison_pairs: Res<CollisionPairs>,
    mut contacts: ResMut<SubstepContacts>,
    colliders: Res<Assets<Collider>>,
) {
    contacts.clear();
    for c in collison_pairs.iter() {
        let [(entity_a, mut trans_a, inv_mass_a, collider_handle_a), (entity_b, mut trans_b, inv_mass_b, collider_handle_b)] =
            query.get_many_mut([c.entity_a, c.entity_b]).unwrap();

        let collider_a = colliders.get(collider_handle_a).unwrap();
        let collider_b = colliders.get(collider_handle_b).unwrap();

        match (collider_a, collider_b) {
            (Collider::Sphere(sphere_a), Collider::Sphere(sphere_b)) => {
                if let Some(intersection) = sphere_sphere_intersect(
                    trans_a.translation,
                    sphere_a.radius,
                    trans_b.translation,
                    sphere_b.radius,
                ) {
                    constrain_body_positions(
                        &mut trans_a,
                        &mut trans_b,
                        inv_mass_a,
                        inv_mass_b,
                        intersection.normal,
                        intersection.penetration,
                    );
                    contacts.push(Contact {
                        entity_a,
                        entity_b,
                        normal: intersection.normal,
                        penetration: intersection.penetration,
                    });
                }
            }
            (_, _) => {
                if let Some(intersect) =
                    gjk_intersect(&collider_a, &trans_a, &collider_b, &trans_b, 0.001)
                {
                    constrain_body_positions(
                        &mut trans_a,
                        &mut trans_b,
                        inv_mass_a,
                        inv_mass_b,
                        intersect.normal,
                        intersect.penetration,
                    );
                    contacts.push(Contact {
                        entity_a,
                        entity_b,
                        normal: intersect.normal,
                        penetration: intersect.penetration,
                    });
                }
            }
        }
    }
}

/// Solves overlap between two dynamic bodies according to their masses
fn constrain_body_positions(
    trans_a: &mut Transform,
    trans_b: &mut Transform,
    inv_mass_a: &InverseMass,
    inv_mass_b: &InverseMass,
    n: Vec3,
    penetration_depth: f32,
) {
    let w_sum = inv_mass_a.0 + inv_mass_b.0;
    let pos_impulse = n * (-penetration_depth / w_sum);
    trans_a.translation += pos_impulse * inv_mass_a.0;
    trans_b.translation -= pos_impulse * inv_mass_b.0;
}

// Solve a overlap between a dynamic object and a static object
// fn constrain_body_position(trans: &mut Transform, normal: Vec3, penetration_depth: f32) {
//     trans.translation -= normal * penetration_depth;
// }
