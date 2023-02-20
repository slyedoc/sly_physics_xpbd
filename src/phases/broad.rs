use bevy::prelude::*;

use crate::{
    components::{Aabb, PhysicsStatic},
    contacts::*,
};

// Sweep and Prune
// The board phase is responsible for pruning the search space of possible collisions
// I have tried different approaches, and I am sure I will try a few more
// So far this simple approach has been the fastest
pub fn broad_phase(
    mut broad_dynamic_contacts: EventWriter<BroadContactDynamic>,
    mut broad_static_contacts: EventWriter<BroadContactStatic>,
    query: Query<(Entity, &Aabb, Option<&PhysicsStatic>)>,
) {
    // TODO: Yes, we are copying the array out here, only way to sort it
    // Ideally we would keep the array around, it should already near sorted
    let mut list = query.iter().collect::<Vec<_>>();

    // Sort the array on currently selected sorting axis
    // Note: Update inter loop if you change the axis
    list.sort_unstable_by(cmp_y_axis);

    //let t1 = Instant::now();
    // Sweep the array for collisions
    for (i, (a, aabb_a, static_a)) in list.iter().enumerate() {
        // Test collisions against all possible overlapping AABBs following current one
        for (b, aabb_b, static_b) in list.iter().skip(i + 1) {
            // Stop when tested AABBs are beyond the end of current AABB
            if aabb_b.mins.y > aabb_a.maxs.y {
                break;
            }

            // SAT test
            if aabb_a.mins.x >= aabb_b.maxs.x {
                continue;
            }
            if aabb_a.maxs.x <= aabb_b.mins.x {
                continue;
            }

            if aabb_a.mins.y >= aabb_b.maxs.y {
                continue;
            }
            if aabb_a.maxs.y <= aabb_b.mins.y {
                continue;
            }

            if aabb_a.mins.z >= aabb_b.maxs.z {
                continue;
            }
            if aabb_a.maxs.z <= aabb_b.mins.z {
                continue;
            }


            match (static_a, static_b) {
                (None, None) => {
                    broad_dynamic_contacts.send(BroadContactDynamic {
                        entity_a: *a,
                        entity_b: *b,
                    });
                }
                (None, Some(_)) => {
                    broad_static_contacts.send(BroadContactStatic {
                        entity: *a,
                        static_entity: *b,
                    });
                }
                (Some(_), None) => {
                    broad_static_contacts.send(BroadContactStatic {
                        entity: *b,
                        static_entity: *a,
                    });
                }
                (Some(_), Some(_)) => {
                    // Both are static, do nothing
                }
            }
        }
    }
}

#[allow(dead_code)]
fn cmp_x_axis(
    a: &(Entity, &Aabb, Option<&PhysicsStatic>),
    b: &(Entity, &Aabb, Option<&PhysicsStatic>),
) -> std::cmp::Ordering {
    // Sort on minimum value along either x, y, or z axis
    let min_a = a.1.mins.x;
    let min_b = b.1.mins.x;
    if min_a < min_b {
        return std::cmp::Ordering::Less;
    }
    if min_a > min_b {
        return std::cmp::Ordering::Greater;
    }
    std::cmp::Ordering::Equal
}

#[allow(dead_code)]
fn cmp_y_axis(
    a: &(Entity, &Aabb, Option<&PhysicsStatic>),
    b: &(Entity, &Aabb, Option<&PhysicsStatic>),
) -> std::cmp::Ordering {
    // Sort on minimum value along either x, y, or z axis
    let min_a = a.1.mins.y;
    let min_b = b.1.mins.y;
    if min_a < min_b {
        return std::cmp::Ordering::Less;
    }
    if min_a > min_b {
        return std::cmp::Ordering::Greater;
    }
    std::cmp::Ordering::Equal
}

#[allow(dead_code)]
fn cmp_z_axis(
    a: &(Entity, Aabb, Option<&PhysicsStatic>),
    b: &(Entity, Aabb, Option<&PhysicsStatic>),
) -> std::cmp::Ordering {
    // Sort on minimum value along either x, y, or z axis
    let min_a = a.1.mins.z;
    let min_b = b.1.mins.z;
    if min_a < min_b {
        return std::cmp::Ordering::Less;
    }
    if min_a > min_b {
        return std::cmp::Ordering::Greater;
    }
    std::cmp::Ordering::Equal
}
