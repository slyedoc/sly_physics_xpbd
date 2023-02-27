mod assets;
mod text_overlay;
use bevy::prelude::*;
use sly_camera_controller::{CameraController, CameraControllerPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use self::{
    assets::{ButtonColors, FontAssets},
    text_overlay::TextOverlayPlugin,
};

pub struct HelperPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ResetState {
    Playing,
    Reset,
}

impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(ResetState::Playing)
            .add_plugin(WorldInspectorPlugin)
            .init_resource::<FontAssets>()
            .init_resource::<ButtonColors>()
            .add_plugin(CameraControllerPlugin)
            .add_plugin(TextOverlayPlugin)
            .add_system_set(
                SystemSet::on_update(ResetState::Playing)
                    .with_system(reset_listen)
            )
            .add_system_set(
                SystemSet::on_update(ResetState::Reset)
                    .with_system(reset)
            )
            ;
    }
}

pub fn setup_camera(mut commands: Commands) {
    // light
    commands
        .spawn(DirectionalLightBundle {
            transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .insert(Keep);

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        // Add our controller
            .insert(CameraController::default())
        .insert(Keep);
}


// Theses system is used to reset the game when the user presses the 'r' key.
#[derive(Component)]
pub struct Keep;

pub fn reset(
    mut commands: Commands, 
    query: Query<Entity, (Without<Keep>, Without<Parent>)>,
    mut app_state: ResMut<State<ResetState>>
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
    app_state.set(ResetState::Playing).unwrap();    
}

pub fn reset_listen(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<ResetState>>
) {
    if keys.just_pressed(KeyCode::R) {
        if app_state.current() == &ResetState::Reset {
            return;
        }
        app_state.set(ResetState::Reset).unwrap();        
        keys.reset(KeyCode::R);
    }
}
