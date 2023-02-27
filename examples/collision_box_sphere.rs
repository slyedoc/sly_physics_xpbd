mod helper;
use bevy::prelude::*;

use helper::HelperPlugin;
use sly_physics_xpbd::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelperPlugin)
        // our physics plugin
        .add_plugin(PhysicsPlugin {
            gravity: Vec3::new(0., 0., 0.),
            ..default()
        })
        // local setup stuff
        .add_startup_system(helper::setup_camera)
        .add_system_set(SystemSet::on_enter(helper::ResetState::Playing).with_system(setup))
        .run();
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut colliders: ResMut<Assets<Collider>>,
) {


    let mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..Default::default()
    });

    // Left sphere
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.5,
                subdivisions: 4,
            })),
            material: mat.clone(),
            transform: Transform::from_translation(Vec3::new(-2., 0., 0.)),
            ..Default::default()
        })
        .insert(PhysicsBundle {
            velocity: Velocity {
                linear: Vec3::new(2., 0., 0.),
                ..default()
            },
            collider: colliders.add(Collider::new_sphere(0.5)),
            ..default()
        })
        .insert(Name::new("P1"));

    // Right sphere
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1., 1., 1.))),
            material: mat.clone(),
            transform: Transform::from_translation(Vec3::new(2., 0., 0.)),
            ..Default::default()
        })
        .insert(PhysicsBundle {
            velocity: Velocity {
                linear: Vec3::new(-2., 0., 0.),
                ..default()
            },
            collider: colliders.add(Collider::new_box(1., 1., 1.)),
            ..default()
        })
        .insert(Name::new("P2"));
}
