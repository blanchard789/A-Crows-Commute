use bevy::{
    //diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    //text::TextWriter,
    window::{PresentMode, Window, WindowPlugin, WindowTheme},
};
use rand::Rng;

// Sets a const global var for the screen size
const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;

#[derive(Component)]
pub struct BackgroundEntity;

#[derive(Component)]
pub struct StatusEntity;

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

/// ## fn main() : Begin  
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
        .add_systems(Update, place_worm)
        .add_systems(Update, chase_player)
        .add_systems(Update, success_check)
        .add_systems(Update, reset)
        .run();
}

/// ## fn setup() : Startup  
/// Initializes spawn settings for game sprites,  
/// such as location, scale, and other default settings   
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
        Status { event_id: -1 },
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
            translation: Vec3::new(0., 0., 3.),
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

/// ## fn movement() : Update  
/// Handles player movement key inputs  
/// and transforms the crow to move in the requested direction
fn movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Active), With<CrowEntity>>,
    mut event_query: Query<&mut Status, With<BackgroundEntity>>,
) {
    for (mut transform, mut active) in query.iter_mut() {
        if input.pressed(KeyCode::KeyW)
            || input.pressed(KeyCode::KeyA)
            || input.pressed(KeyCode::KeyD)
            || input.pressed(KeyCode::KeyS)
            || input.pressed(KeyCode::ArrowUp)
            || input.pressed(KeyCode::ArrowLeft)
            || input.pressed(KeyCode::ArrowRight)
            || input.pressed(KeyCode::ArrowDown)
        {
            active.active = true;
            if let Ok(ref mut stat) = event_query.get_single_mut() {
                if stat.event_id == 0 {
                    stat.event_id = 1;
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

/// ## fn collision() : Update  
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
    mut status_query: Single<&mut Status, With<BackgroundEntity>>,
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
                status_query.event_id = 2;
            }
        }
    }
}

/// ## fn place_worm() : Update  
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

/// ## fn chase_player() : Update  
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

/// ## fn success_check() : Update  
/// Checks if the crow has collected 10 worms.  
/// If so Status.event_id is set to 3 = success condition  
fn success_check(
    worm_query: Single<&Count, With<WormEntity>>,
    mut status_query: Single<&mut Status, With<BackgroundEntity>>,
) {
    if worm_query.count >= 10 {
        status_query.event_id = 3;
    }
}

/// ## fn reset() : Update  
/// Automatically checks the Status.event_id for event updates  
/// 0 = Inactive/Reset  
/// 1 = Active  
/// 2 = Fail Condition  
/// 3 = Success Condition  
/// If status 2 or 3, sprites are reset back default settings (location, counters, active state)  
/// and status is set to 0.  
#[allow(clippy::type_complexity)]
fn reset(
    mut event_query: Single<&mut Status, With<BackgroundEntity>>,
    mut crow_query: Single<(&mut Transform, &mut Active), (With<CrowEntity>, Without<HawkEntity>)>,
    mut worm_query: Single<(&mut Active, &mut Count), (With<WormEntity>, Without<CrowEntity>)>,
    mut hawk_query: Single<&mut Transform, (With<HawkEntity>, Without<CrowEntity>)>,
) {
    if (event_query.event_id == 2 || event_query.event_id == 3) && crow_query.1.active {
        event_query.event_id = 0;
        crow_query.1.active = false;
        crow_query.0.translation = Vec3::new(425., -300., 2.);
        worm_query.0.active = false;
        worm_query.1.count = 0;
        hawk_query.translation = Vec3::new(-200., 0., 3.);
    }
}
