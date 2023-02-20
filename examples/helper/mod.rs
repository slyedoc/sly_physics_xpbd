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

impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(WorldInspectorPlugin)
            .init_resource::<FontAssets>()
            .init_resource::<ButtonColors>()
            .add_plugin(CameraControllerPlugin)
            .add_plugin(TextOverlayPlugin);
    }
}

#[derive(Component)]
pub struct Keep;

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
