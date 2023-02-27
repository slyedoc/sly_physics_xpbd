use bevy::prelude::*;

#[derive(Debug)]
pub struct CollisionPair {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

#[derive(Copy, Clone, Debug)]
pub struct Contact {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub normal: Vec3,
    pub penetration: f32,
    // pub world_point_a: Vec3,
    // pub world_point_b: Vec3,
    // pub local_point_a: Vec3,
    // pub local_point_b: Vec3,
    // pub separation_dist: f32,
    // pub time_of_impact: f32,
}

pub struct Intersection {
    pub normal: Vec3,
    pub penetration: f32,    
}