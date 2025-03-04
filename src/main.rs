use bevy::{
    //diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, Window, WindowPlugin, WindowTheme},
};

/// Initializes window settings and starts the game loop
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
        .add_systems(Update, movement)
        .run();
}

/// Initializes spawn settings for game sprites,  
/// such as location and scale
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("crowGame_downtown.png")),
        Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        Name::new("Background"),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("crowGame_crow.png")),
        Transform {
            translation: Vec3::new(0., 0., 2.),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        Name::new("Crow"),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("crowGame_hawk.png")),
        Transform {
            translation: Vec3::new(-200., 0., 3.),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        Name::new("Hawk"),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("crowGame_food.png")),
        Transform {
            translation: Vec3::new(200., 0., 1.),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        Name::new("Worm"),
    ));
}

/// Handles player movement key inputs  
/// and transforms the crow to move in the requested direction
fn movement(input: Res<ButtonInput<KeyCode>>, mut query: Query<(&Name, &mut Transform)>) {
    for (name, mut transform) in query.iter_mut() {
        if name.as_str() == "Crow" {
            if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
                transform.rotation = Quat::from_rotation_z(0.0_f32.to_radians());
                transform.translation.y += 1.
            } else if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
                transform.rotation = Quat::from_rotation_z(90.0_f32.to_radians());
                transform.translation.x -= 1.
            } else if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
                transform.rotation = Quat::from_rotation_z(270.0_f32.to_radians());
                transform.translation.x += 1.
            } else if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
                transform.rotation = Quat::from_rotation_z(180.0_f32.to_radians());
                transform.translation.y -= 1.
            }

            if transform.translation.y > ((1070.0 - 100.0) / 2.0) {
                transform.translation.y = (1069.0 - 100.0) / 2.0
            }
            if transform.translation.y < ((-1070.0 + 100.0) / 2.0) {
                transform.translation.y = (-1069.0 + 100.0) / 2.0
            }
            if transform.translation.x > ((1910.0 - 100.0) / 2.0) {
                transform.translation.x = (1909.0 - 100.0) / 2.0
            }
            if transform.translation.x < ((-1910.0 + 100.0) / 2.0) {
                transform.translation.x = (-1909.0 + 100.0) / 2.0
            }
        }
    }
}
