use bevy::prelude::*;

pub struct BroadContactDynamic {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

pub struct BroadContactStatic {
    pub entity: Entity,
    pub static_entity: Entity,
}

#[derive(Copy, Clone, Debug)]
pub struct ContactDynamic {
    pub entity_a: Entity,
    pub entity_b: Entity,
    // pub world_point_a: Vec3,
    // pub world_point_b: Vec3,
    // pub local_point_a: Vec3,
    // pub local_point_b: Vec3,
    pub normal: Vec3,
    // pub separation_dist: f32,
    // pub time_of_impact: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct ContactStatic {
    pub entity: Entity,
    pub static_entity: Entity,
    // pub world_point_a: Vec3,
    // pub world_point_b: Vec3,
    // pub local_point_a: Vec3,
    // pub local_point_b: Vec3,
    pub normal: Vec3,
    // pub separation_dist: f32,
    // pub time_of_impact: f32,
}
