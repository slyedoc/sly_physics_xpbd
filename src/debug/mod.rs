mod entity_aabb;

use bevy::prelude::*;
use entity_aabb::DebugEntityAabbPlugin;

pub struct PhysicsDebugPlugin;
impl Plugin for PhysicsDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DebugEntityAabbPlugin);
    }
}

