use bevy::{
    //diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, Window, WindowPlugin, WindowTheme},
};
use rand::Rng;

// Sets a const global var for the screen size
const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;

/// Initializes window settings and starts the game loop  
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "A Crow's Commute".into(),
                    name: Some("bevy.app".into()),
                    resolution: (WIDTH, HEIGHT).into(),
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
        .add_systems(Update, collision)
        //.add_systems(Update, despawn_entity)
        .add_systems(Update, place_worm)
        .add_systems(Update, chase_player)
        .run();
}

#[derive(Component)]
pub struct BackgroundEntity;

#[derive(Component)]
pub struct CrowEntity;

#[derive(Component)]
pub struct WormEntity;

#[derive(Component)]
pub struct HawkEntity;

#[derive(Component)]
pub struct Active {
    active: bool,
}

#[derive(Component)]
pub struct Count {
    count: i32,
}

#[derive(Component)]
pub struct Status {
    event_id: i32,
}

/// ## fn setup()
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
        BackgroundEntity,
        Status { event_id: 0 },
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("crowGame_crow.png")),
        Transform {
            translation: Vec3::new(425., -300., 2.),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        CrowEntity,
        Active { active: false },
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("crowGame_hawk.png")),
        Transform {
            translation: Vec3::new(-200., 0., 3.),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        HawkEntity,
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("crowGame_food.png")),
        Transform {
            translation: Vec3::new(200., 0., 1.),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        WormEntity,
        Active { active: false },
        Count { count: 0 },
    ));
}

/// ## fn movement()  
/// Handles player movement key inputs  
/// and transforms the crow to move in the requested direction
//#[allow(clippy::type_complexity)]
fn movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Active), With<CrowEntity>>,
    mut event_query: Query<&mut Status, With<BackgroundEntity>>,
) {
    for (mut transform, mut active) in query.iter_mut() {
        if !active.active
            && (input.pressed(KeyCode::KeyW)
                || input.pressed(KeyCode::KeyA)
                || input.pressed(KeyCode::KeyD)
                || input.pressed(KeyCode::KeyS)
                || input.pressed(KeyCode::ArrowUp)
                || input.pressed(KeyCode::ArrowLeft)
                || input.pressed(KeyCode::ArrowRight)
                || input.pressed(KeyCode::ArrowDown))
        {
            active.active = true;
            if let Ok(ref mut stat) = event_query.get_single_mut() {
                if stat.event_id != 0 {
                    stat.event_id = 0;
                }
            }
        }

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

        if transform.translation.y > ((HEIGHT - 110.0) / 2.0) {
            transform.translation.y = (HEIGHT - 111.0) / 2.0
        }
        if transform.translation.y < ((-HEIGHT + 110.0) / 2.0) {
            transform.translation.y = (-HEIGHT + 111.0) / 2.0
        }
        if transform.translation.x > ((WIDTH - 110.0) / 2.0) {
            transform.translation.x = (WIDTH - 110.0) / 2.0
        }
        if transform.translation.x < ((-WIDTH + 110.0) / 2.0) {
            transform.translation.x = (-WIDTH + 111.0) / 2.0
        }
    }
}

/// ## fn collision()  
/// Handles collision events.  
/// Such as the player colliding with worms or hawks.  
/// If a player collides with a worm the score is increased.  
/// If a player collides with a hawk the game ends.  
#[allow(clippy::type_complexity)]
fn collision(
    crow_query: Query<&Transform, With<CrowEntity>>,
    mut worm_query: Query<
        (&mut Transform, &mut Active, &mut Count),
        (With<WormEntity>, Without<CrowEntity>, Without<HawkEntity>),
    >,
    hawk_query: Query<&Transform, With<HawkEntity>>,
) {
    if let Ok(crow_transform) = crow_query.get_single() {
        if let Ok(ref mut worm_transform) = worm_query.get_single_mut() {
            if (crow_transform.translation.x - worm_transform.0.translation.x).abs() <= 30.
                && (crow_transform.translation.y - worm_transform.0.translation.y).abs() <= 30.
            {
                worm_transform.1.active = false;
                worm_transform.2.count += 1;
            }
        }
        if let Ok(hawk_transform) = hawk_query.get_single() {
            if (crow_transform.translation.x - hawk_transform.translation.x).abs() <= 45.
                && (crow_transform.translation.y - hawk_transform.translation.y).abs() <= 45.
            {
                //println!("Hawk");
            }
        }
    }
}

/// ## fn place_worm()
/// When the worm is spawned or eaten and a new worm is needed.
/// This function generates a random number and teleports worm to it.
fn place_worm(mut query: Query<(&mut Transform, &mut Active), With<WormEntity>>) {
    for (mut transform, mut active) in query.iter_mut() {
        if !active.active {
            let mut rng = rand::rng();
            let x: i32 =
                rng.random_range(((-WIDTH + 110.) / 2.) as i32..((WIDTH - 110.) / 2.) as i32);
            let y: i32 =
                rng.random_range(((-HEIGHT + 110.) / 2.) as i32..((HEIGHT - 110.) / 2.) as i32);
            transform.translation.x = x as f32;
            transform.translation.y = y as f32;
            active.active = true;
        }
    }
}

/// ## fn chase_player()
/// Extracts the player's location (the Crow) and compares it the Hawk's location.  
/// Then moves Hawk towards the player.  
fn chase_player(
    mut query: Query<&mut Transform, (With<HawkEntity>, Without<CrowEntity>)>,
    player_query: Query<(&Transform, &Active), With<CrowEntity>>,
) {
    if let Ok(c_transform) = player_query.get_single() {
        if c_transform.1.active {
            for mut transform in query.iter_mut() {
                if transform.translation.y <= c_transform.0.translation.y
                    && transform.translation.x == c_transform.0.translation.x
                {
                    transform.rotation = Quat::from_rotation_z(0.0_f32.to_radians());
                    transform.translation.y += 0.5;
                } else if transform.translation.x >= c_transform.0.translation.x
                    && transform.translation.y == c_transform.0.translation.y
                {
                    transform.rotation = Quat::from_rotation_z(90.0_f32.to_radians());
                    transform.translation.x -= 0.5;
                } else if transform.translation.x <= c_transform.0.translation.x
                    && transform.translation.y == c_transform.0.translation.y
                {
                    transform.rotation = Quat::from_rotation_z(270.0_f32.to_radians());
                    transform.translation.x += 0.5;
                } else if transform.translation.y >= c_transform.0.translation.y
                    && transform.translation.x == c_transform.0.translation.x
                {
                    transform.rotation = Quat::from_rotation_z(180.0_f32.to_radians());
                    transform.translation.y -= 0.5;
                } else if transform.translation.y <= c_transform.0.translation.y
                    && transform.translation.x >= c_transform.0.translation.x
                {
                    transform.rotation = Quat::from_rotation_z(45.0_f32.to_radians());
                    transform.translation.y += 0.50;
                    transform.translation.x -= 0.50;
                } else if transform.translation.y < c_transform.0.translation.y
                    && transform.translation.x < c_transform.0.translation.x
                {
                    transform.rotation = Quat::from_rotation_z(315.0_f32.to_radians());
                    transform.translation.y += 0.50;
                    transform.translation.x += 0.50;
                } else if transform.translation.y > c_transform.0.translation.y
                    && transform.translation.x > c_transform.0.translation.x
                {
                    transform.rotation = Quat::from_rotation_z(135.0_f32.to_radians());
                    transform.translation.y -= 0.50;
                    transform.translation.x -= 0.50;
                } else if transform.translation.y > c_transform.0.translation.y
                    && transform.translation.x < c_transform.0.translation.x
                {
                    transform.rotation = Quat::from_rotation_z(225.0_f32.to_radians());
                    transform.translation.y -= 0.50;
                    transform.translation.x += 0.50;
                }
            }
        }
    }
}
