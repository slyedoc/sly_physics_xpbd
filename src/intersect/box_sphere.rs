use crate::contacts::Intersection;
use bevy::prelude::*;

// TODO: this is broken
#[allow(dead_code)]
pub fn box_sphere_intersect(
    box_trans: &Transform,
    box_half_size: Vec3,
    sphere_trans: &Transform,
    sphere_radius: f32,
) -> Option<Intersection> {
    // TODO: Maybe cache this inverse
    let relative_center = box_trans.transform_point(sphere_trans.translation);

    // Early-out check to see if we can exclude the contact.
    if relative_center.x.abs() - sphere_radius > box_half_size.x
        || relative_center.y.abs() - sphere_radius > box_half_size.y
        || relative_center.z.abs() - sphere_radius > box_half_size.z
    {
        return None;
    }

    let mut closest_point = Vec3::ZERO;

    // Clamp each coordinate to the box.
    let mut dist = relative_center.x;
    if dist > box_half_size.x {
        dist = box_half_size.x;
    }
    if dist < -box_half_size.x {
        dist = -box_half_size.x;
    }
    closest_point.x = dist;

    dist = relative_center.y;
    if dist > box_half_size.y {
        dist = box_half_size.y;
    }
    if dist < -box_half_size.y {
        dist = -box_half_size.y;
    }
    closest_point.y = dist;

    dist = relative_center.z;
    if dist > box_half_size.z {
        dist = box_half_size.z;
    }
    if dist < -box_half_size.z {
        dist = -box_half_size.z;
    }
    closest_point.z = dist;

    // Check we’re in contact.
    dist = (closest_point - relative_center).length_squared();
    if dist > sphere_radius * sphere_radius {
        return None;
    }

    // Compile the contact.
    let closest_point_world = box_trans.transform_point(closest_point);

    Some(Intersection {
        normal: (sphere_trans.translation - closest_point_world).normalize(),
        penetration: sphere_radius - dist.sqrt(),
    })
}
