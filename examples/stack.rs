mod helper;
use bevy::prelude::*;

use helper::HelperPlugin;
use sly_physics_xpbd::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelperPlugin)
        // our physics plugin
        .add_plugin(PhysicsPlugin::default())
        // local setup stuff
        .add_startup_system(helper::setup_camera)
        .add_startup_system(setup)
        .run();
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.5,
        subdivisions: 4,
    }));

    let white = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: sphere.clone(),
            material: white.clone(),
            transform: Transform::from_translation(Vec3::ZERO),
            ..Default::default()
        })
        .insert(PhysicsBundle {
            velocity: Velocity {
                linear: Vec3::new(1., 0., 0.),
                ..default()
            },
            ..default()
        });
}
