use bevy::prelude::*;

use crate::{colliders::*, components::*, contacts::*};

pub fn solve_pos(
    mut query: Query<(Entity, &mut Transform, &Mass, &Handle<Collider>)>,
    mut broad_contacts: EventReader<BroadContactDynamic>,
    mut contacts: EventWriter<ContactDynamic>,
    colliders: Res<Assets<Collider>>,
) {
    for c in broad_contacts.iter() {
        let [(entity_a, mut trans_a, mass_a, collider_handle_a), (entity_b, mut trans_b, mass_b, collider_handle_b)] =
            query.get_many_mut([c.entity_a, c.entity_b]).unwrap();

        let collider_a = colliders.get(collider_handle_a).unwrap();
        let collider_b = colliders.get(collider_handle_b).unwrap();

        match (collider_a, collider_b) {
            (Collider::Sphere(sphere_a), Collider::Sphere(sphere_b)) => {
                let ab = trans_b.translation - trans_a.translation;
                let combined_radius = sphere_a.radius + sphere_b.radius;
                let ab_sqr_len = ab.length_squared();
                if ab_sqr_len < combined_radius * combined_radius {
                    let ab_length = ab_sqr_len.sqrt();
                    let penetration_depth = combined_radius - ab_length;
                    let n = ab / ab_length;

                    let w_a = 1. / mass_a.0;
                    let w_b = 1. / mass_b.0;
                    let w_sum = w_a + w_b;

                    trans_a.translation -= n * penetration_depth * w_a / w_sum;
                    trans_b.translation += n * penetration_depth * w_b / w_sum;

                    contacts.send(ContactDynamic {
                        entity_a,
                        entity_b,
                        normal: n,
                    });
                }
            }
            (Collider::Sphere(_), Collider::Box(_)) => todo!(),
            (Collider::Box(_), Collider::Sphere(_)) => todo!(),
            (Collider::Box(_), Collider::Box(_)) => todo!(),
        }
    }
}

pub fn solve_pos_statics(
    mut dynamics: Query<(Entity, &mut Transform, &Handle<Collider>), With<Mass>>,
    statics: Query<(Entity, &Transform, &Handle<Collider>), Without<Mass>>,
    mut broad_contacts: EventReader<BroadContactStatic>,
    mut contacts: EventWriter<ContactStatic>,
    colliders: Res<Assets<Collider>>,
) {
    for c in broad_contacts.iter() {
        let (entity_a, mut trans_a, collider_handle_a) = dynamics.get_mut(c.entity).unwrap();
        let (entity_b, trans_b, collider_handle_b) = statics.get(c.static_entity).unwrap();
        let collider_a = colliders.get(collider_handle_a).unwrap();
        let collider_b = colliders.get(collider_handle_b).unwrap();
        match (collider_a, collider_b) {
            (Collider::Sphere(circle_a), Collider::Sphere(circle_b)) => {
                let ab = trans_b.translation - trans_a.translation;
                let combined_radius = circle_a.radius + circle_b.radius;
                let ab_sqr_len = ab.length_squared();
                if ab_sqr_len < combined_radius * combined_radius {
                    let ab_length = ab_sqr_len.sqrt();
                    let penetration_depth = combined_radius - ab_length;
                    let n = ab / ab_length;
                    trans_a.translation -= n * penetration_depth;
                    contacts.send(ContactStatic {
                        entity: entity_a,
                        static_entity: entity_b,
                        normal: n,
                    });
                }
            }
            (Collider::Sphere(_), Collider::Box(_)) => todo!(),
            (Collider::Box(_), Collider::Sphere(_)) => todo!(),
            (Collider::Box(_), Collider::Box(_)) => todo!(),
        }
    }
}
