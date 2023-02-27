mod helper;
use bevy::{prelude::*, time::FixedTimestep};

use helper::HelperPlugin;
use sly_physics_xpbd::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelperPlugin)
        // our physics plugin
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(PhysicsDebugPlugin)

        // local setup stuff
        .add_startup_system(helper::setup_camera)
        .add_system_set(SystemSet::on_enter(helper::ResetState::Playing).with_system(setup))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1. / 20.))
                .with_system(spawn_marbles),
        )
        .add_system(despawn_marbles)
        .run();
}

#[derive(Component)]
struct Marble;

#[derive(Resource)]
struct MarbleAssets {
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    collider: Handle<Collider>,
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut colliders: ResMut<Assets<Collider>>,
) {
    // setup marble assets
    commands.insert_resource(MarbleAssets {
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.4, 0.4, 0.6),
            ..Default::default()
        }),
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.5,
            subdivisions: 4,
        })),
        collider: colliders.add(Collider::new_sphere(0.5)),
    });

    // setup ground
    let radius = 200.;
    let pos = Vec3::new(0., -radius - 2., 0.);

    commands
        .spawn(PbrBundle {
            //mesh: meshes.add(Mesh::from(shape::UVSphere { radius, ..default() })),
            mesh: meshes.add(Mesh::from(shape::Box::new(radius * 2., radius * 2., radius * 2.))),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GREEN,
                ..default()
            }),
            transform: Transform {
                translation: pos,
                ..default()
            },
            ..default()
        })
        .insert(PhysicsBundle {
            mode: PhysicsMode::Static,
            //collider: colliders.add(Collider::new_sphere(radius)),
            collider: colliders.add(Collider::new_box(radius * 2., radius * 2., radius * 2.)),
            ..default()
        })
        .insert(Name::new("Ground"));
}

fn spawn_marbles(mut commands: Commands, marble_assets: Res<MarbleAssets>) {
    let pos = Vec3::new(
        fastrand::f32() - 0.5,
        fastrand::f32() - 0.5,
        fastrand::f32() - 0.5,
    ) * 5.0
        + Vec3::Y * 10.;
    let vel = Vec3::new(fastrand::f32() - 0.5, fastrand::f32() - 0.5, 0.);
    commands
        .spawn(PbrBundle {
            mesh: marble_assets.mesh.clone(),
            material: marble_assets.material.clone(),
            transform: Transform {
                translation: pos,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PhysicsBundle {
            collider: marble_assets.collider.clone(),
            restitution: Restitution(0.9),
            velocity: Velocity {
                linear: vel,
                ..Default::default()
            },
            ..default()
        })
        .insert(Marble)
        .insert(Name::new("Marble"));
}

fn despawn_marbles(mut commands: Commands, query: Query<(Entity, &Transform), With<Marble>>) {
    for (entity, trans) in query.iter() {
        if trans.translation.y < -20. {
            commands.entity(entity).despawn();
        }
    }
}
