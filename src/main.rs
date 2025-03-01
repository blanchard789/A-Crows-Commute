use bevy::{
    //diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, Window, WindowPlugin, WindowTheme},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "A Crow's Commute".into(),
                    name: Some("bevy.app".into()),
                    resolution: (1920., 1080.).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
            //LogDiagnosticsPlugin::default(),
            //FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

/// Initializes spawn settings for game sprites.
/// Such as location and scale
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("crowGame_downtown.png")),
        Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("crowGame_crow.png")),
        Transform {
            translation: Vec3::new(0., 0., 1.),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
    ));
}
