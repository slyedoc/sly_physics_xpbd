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
        //.add_plugin(PhysicsDebugPlugin)
        // local setup stuff
        .add_startup_system(helper::setup_camera)
        .add_system_set(SystemSet::on_enter(helper::ResetState::Playing).with_system(setup))
        .run();
}

enum Shape {
    #[allow(dead_code)]
    Sphere,
    #[allow(dead_code)]
    Box,
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut colliders: ResMut<Assets<Collider>>,
    asset_server: Res<AssetServer>,
) {
    // Ground
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(100., 1., 100.))),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GREEN,
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(0., -1.0, 0.),
                ..default()
            },
            ..default()
        })
        .insert(PhysicsBundle {
            mode: PhysicsMode::Static,
            collider: colliders.add(Collider::new_box(100., 1., 100.)),
            ..default()
        })
        .insert(Name::new("Ground"));

    // stack
    let shape = Shape::Sphere;
    let size = 1.0;
    let count = 10;

    let mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(asset_server.load("checker_red.png")),
        ..default()
    });

    let (mesh, collider) = match shape {
        Shape::Sphere => (
                meshes.add(Mesh::from(shape::Icosphere {
                        radius: size * 0.5,
                        subdivisions: 20,
                })),
                colliders.add(Collider::new_sphere(size * 0.5)) 
            ),
        Shape::Box => (
            meshes.add(Mesh::from(shape::Box::new(size, size, size))),
            colliders.add(Collider::new_box(size, size, size))
        ),
    };
    

    for x in 0..count {
        for y in 0..count {
            for z in 0..1 {
                commands
                    .spawn(PbrBundle {
                        mesh: mesh.clone(),
                        material: mat.clone(),
                        transform: Transform {
                            translation: Vec3::new(size * x as f32,  size *  y as f32, size * z as f32),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(PhysicsBundle {
                        collider: collider.clone(),
                        ..default()
                    })
                    .insert(Name::new("Sphere"));
            }
        }
    }
}
