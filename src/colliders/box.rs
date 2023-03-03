use bevy::{math::vec3, prelude::*};

use crate::{Ray, components::*};

use super::{fastest_linear_speed, find_support_point, Collidable};

#[derive(Debug)]
pub struct Box {
    pub half_size: Vec3,
    pub size: Vec3,
    pub verts: Vec<Vec3>,
    pub center_of_mass: Vec3,    
    pub aabb: Aabb,
}
impl Default for Box {
    fn default() -> Self {
        Self::new(vec3(1.0, 1.0, 1.0))
    }
}

impl Box {
    pub fn new(size: Vec3) -> Self {
        let half_size = size * 0.5;
        let aabb = Aabb {
            mins: -half_size,
            maxs: half_size,
        };


        Box {
            half_size,
            size,
            verts: vec![
                Vec3::new(-half_size.x, -half_size.y, -half_size.z),
                Vec3::new(-half_size.x, -half_size.y, half_size.z),
                Vec3::new(-half_size.x, half_size.y, -half_size.z),
                Vec3::new(-half_size.x, half_size.y, half_size.z),
                Vec3::new(half_size.x, -half_size.y, -half_size.z),
                Vec3::new(half_size.x, -half_size.y, half_size.z),
                Vec3::new(half_size.x, half_size.y, -half_size.z),
                Vec3::new(half_size.x, half_size.y, half_size.z),
            ],
            center_of_mass: vec3(0.0, 0.0, 0.0),
            aabb,
        }
    }
}

impl Collidable for Box {
    fn get_center_of_mass(&self) -> Vec3 {
        self.center_of_mass
    }

    fn get_inertia_tensor(&self, mass: f32) -> Mat3 {
                
        let dd = self.size * self.size;
        let diagonal = Vec3::new(dd.y + dd.z, dd.x + dd.z, dd.x + dd.y) * mass / 12.0;
        return Mat3::from_diagonal(diagonal);
    }

    fn get_aabb(&self) -> Aabb {
        self.aabb
    }

    fn update_aabb(&self, aabb: &mut Aabb, trans: &Transform, velocity: &Velocity, factor: f32) {
        aabb.clear();
        
        let margin = factor * velocity.linear.length();
        let half_extends = self.half_size + Vec3::splat( margin);
         aabb.mins = trans.translation - half_extends;
         aabb.maxs = trans.translation + half_extends;
        // for pt in &self.verts {
        //      aabb.expand_by_point(trans.transform_point(*pt));
        // }
        // let margin = factor * velocity.linear.length();


        // // expand by the linear velocity
        // let p1 = aabb.mins + velocity.linear * factor;
        // aabb.expand_by_point(p1);
        // let p2 = aabb.maxs + velocity.linear * factor;
        // aabb.expand_by_point(p2);

        // let p3 = aabb.mins - Vec3::splat(BOUNDS_EPS);
        // aabb.expand_by_point(p3);
        // let p4 = aabb.maxs + Vec3::splat(BOUNDS_EPS);
        //  aabb.expand_by_point(p4);

        // aabb
    }

    fn get_support(&self, trans: &Transform, dir: Vec3, bias: f32) -> Vec3 {
        find_support_point(&self.verts, dir, trans.translation, trans.rotation, bias)
    }

    fn fastest_linear_speed(&self, angular_velocity: Vec3, dir: Vec3) -> f32 {
        fastest_linear_speed(&self.verts, angular_velocity, self.center_of_mass, dir)
    }
    // Returns distance at which ray would hit the sphere, or None if it doesn't hit
    fn intersect(&self, ray: &mut Ray) -> Option<f32> {
        let mut tmin = (self.aabb.mins.x - ray.origin.x) / ray.direction.x;
        let mut tmax = (self.aabb.maxs.x - ray.origin.x) / ray.direction.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.aabb.mins.y - ray.origin.y) / ray.direction.y;
        let mut tymax = (self.aabb.maxs.y - ray.origin.y) / ray.direction.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return None;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.aabb.mins.z - ray.origin.z) / ray.direction.z;
        let mut tzmax = (self.aabb.maxs.z - ray.origin.z) / ray.direction.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return None;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        if tmax < 0.0 {
            return None;
        }

        if tmin < 0.0 {
            return Some(tmax);
        }

        Some(tmin)
    }
}
