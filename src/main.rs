use bevy::{
    color::palettes::css::{RED, YELLOW},
    prelude::*,
    window::{PresentMode, Window, WindowPlugin, WindowTheme},
};
use rand::Rng;

/// Sets a const global var for the screen width
const WIDTH: f32 = 1920.;
/// Sets a const global var for the screen height
const HEIGHT: f32 = 1080.;

/// Sets a const global var for target worm count
const MAXSCORE: i32 = 10;

/// Tag for tracking background entity
#[derive(Component)]
pub struct BackgroundEntity;

/// Tag for tracking status entity
#[derive(Component)]
pub struct StatusEntity;

/// Tag for tracking crow(player) entity
#[derive(Component)]
pub struct CrowEntity;

/// Tag for tracking worm entity
#[derive(Component)]
pub struct WormEntity;

/// Tag for tracking hawk entity
#[derive(Component)]
pub struct HawkEntity;

/// Tag for tracking text entities
#[derive(Component)]
pub struct TextEntity;

/// Boolean for checking if entities are in an active state
#[derive(Component)]
pub struct Active {
    active: bool,
}

/// Integer for worm collection count checks
#[derive(Component)]
pub struct Count {
    count: i32,
}

/// Integer for tracking the current game status id
#[derive(Component)]
pub struct Status {
    event_id: i32,
}

/// ## fn main() : Begin  
/// Initializes window settings and starts the game loop  
fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
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
        }),))
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .add_systems(Update, collision)
        .add_systems(Update, place_worm)
        .add_systems(Update, chase_player)
        .add_systems(Update, success_check)
        .add_systems(Update, reset)
        .add_systems(Update, despawn_text)
        .add_systems(Update, success_sequence)
        .run();
}

/// ## fn setup() : Startup  
/// Initializes spawn settings for game sprites,  
/// such as location, scale, and other default settings   
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawns camera
    commands.spawn(Camera2d);

    // spawns background image
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

    // spawns crow sprite
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

    // spawns hawk sprite
    commands.spawn((
        Sprite::from_image(asset_server.load("crowGame_hawk.png")),
        Transform {
            translation: Vec3::new(0., 0., 3.),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        HawkEntity,
    ));

    // spawns worm sprite
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
/// Handles player movement key inputs and transforms the crow to move in the requested direction.  
/// Movement requests will be ignored if requested direction would overflow the screen.  
/// Function is locked out if Status.event_id == 4 (credit sequence is active) or 5 (game over lock out).  
/// If event_id == 5 the game over lockout will remain until all movement keys are depressed.
fn movement(
    input: Res<ButtonInput<KeyCode>>,
    mut crow_query: Single<(&mut Transform, &mut Active), With<CrowEntity>>,
    mut event_query: Single<&mut Status, With<BackgroundEntity>>,
) {
    // checks if movement is currently allowed
    if event_query.event_id != 4 && event_query.event_id != 5 {
        // checks if any movement keys are pressed and updates crow to be active if not already,
        // which will start the current game instance.
        if input.pressed(KeyCode::KeyW)
            || input.pressed(KeyCode::KeyA)
            || input.pressed(KeyCode::KeyD)
            || input.pressed(KeyCode::KeyS)
            || input.pressed(KeyCode::ArrowUp)
            || input.pressed(KeyCode::ArrowLeft)
            || input.pressed(KeyCode::ArrowRight)
            || input.pressed(KeyCode::ArrowDown)
        {
            crow_query.1.active = true;
            if event_query.event_id == 0 {
                event_query.event_id = 1;
            }
        }

        // checks if movement keys are pressed and moves the player as requested
        if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
            crow_query.0.rotation = Quat::from_rotation_z(0.0_f32.to_radians());
            crow_query.0.translation.y += 1.
        } else if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
            crow_query.0.rotation = Quat::from_rotation_z(90.0_f32.to_radians());
            crow_query.0.translation.x -= 1.
        } else if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
            crow_query.0.rotation = Quat::from_rotation_z(270.0_f32.to_radians());
            crow_query.0.translation.x += 1.
        } else if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
            crow_query.0.rotation = Quat::from_rotation_z(180.0_f32.to_radians());
            crow_query.0.translation.y -= 1.
        }

        // checks if requested movement overflows the screen and will blocks it if so.
        if crow_query.0.translation.y > ((HEIGHT - 110.0) / 2.0) {
            crow_query.0.translation.y = (HEIGHT - 111.0) / 2.0
        }
        if crow_query.0.translation.y < ((-HEIGHT + 110.0) / 2.0) {
            crow_query.0.translation.y = (-HEIGHT + 111.0) / 2.0
        }
        if crow_query.0.translation.x > ((WIDTH - 110.0) / 2.0) {
            crow_query.0.translation.x = (WIDTH - 110.0) / 2.0
        }
        if crow_query.0.translation.x < ((-WIDTH + 110.0) / 2.0) {
            crow_query.0.translation.x = (-WIDTH + 111.0) / 2.0
        }
    // checks for when all keys are depressed during a game over lock, if so game over lock out is removed.
    } else if event_query.event_id == 5 && input.get_pressed().count() == 0 {
        event_query.event_id = 0;
    }
}

/// ## fn collision() : Update  
/// Handles collision events.  
/// Such as the player colliding with the worm or hawk.  
/// If a player collides with a worm the score is increased.  
/// If a player collides with a hawk the game ends.  
/// Function also reports Status.event_id = 2 if the hawk collides with the player.
#[allow(clippy::type_complexity)]
fn collision(
    crow_query: Single<&Transform, With<CrowEntity>>,
    mut worm_query: Single<
        (&mut Transform, &mut Active, &mut Count),
        (With<WormEntity>, Without<CrowEntity>, Without<HawkEntity>),
    >,
    hawk_query: Single<&Transform, With<HawkEntity>>,
    mut status_query: Single<&mut Status, With<BackgroundEntity>>,
) {
    // checks if crow collides with worm.
    if (crow_query.translation.x - worm_query.0.translation.x).abs() <= 30.
        && (crow_query.translation.y - worm_query.0.translation.y).abs() <= 30.
    {
        worm_query.1.active = false;
        worm_query.2.count += 1;
    }
    // checks if crow collides with hawk.
    if (crow_query.translation.x - hawk_query.translation.x).abs() <= 45.
        && (crow_query.translation.y - hawk_query.translation.y).abs() <= 45.
    {
        status_query.event_id = 2;
    }
}

/// ## fn place_worm() : Update  
/// When a new worm is needed, this function generates a random number and teleports the worm to it.  
/// The function will be locked out if the credit sequence (Status.event_id == 4) is active.  
/// The 10th worm has a scripted spawn location for the credit sequence, but the other 9 are random.
fn place_worm(
    mut query: Single<(&mut Transform, &mut Active, &Count), With<WormEntity>>,
    status_query: Single<&Status, With<BackgroundEntity>>,
) {
    // checks if worm needs to be respawned and if credit sequence is not playing.
    if !query.1.active && status_query.event_id != 4 {
        // checks if less than 10 worms have been collected.
        if query.2.count < MAXSCORE - 1 {
            let mut rng = rand::rng();
            let x: i32 =
                rng.random_range(((-WIDTH + 110.) / 2.) as i32..((WIDTH - 110.) / 2.) as i32);
            let y: i32 =
                rng.random_range(((-HEIGHT + 110.) / 2.) as i32..((HEIGHT - 110.) / 2.) as i32);
            query.0.translation.x = x as f32;
            query.0.translation.y = y as f32;
            query.1.active = true;
        // else the 10th worm will be spawned at scripted location.
        } else {
            query.0.translation.x = (-WIDTH / 2.) + 100.;
            query.0.translation.y = -300.;
            query.1.active = true;
        }
    }
}

/// ## fn chase_player() : Update  
/// Extracts the player's location (the Crow) and compares it the Hawk's location,  
/// then moves Hawk towards the player.  
/// If the hawk is both vertically and horizontally not parallel with the crow,  
/// the hawk moves slightly faster until one or both axes are in line with the crow.  
/// This allows the hawk to catch up to the crow a bit, if they are far apart.
fn chase_player(
    mut hawk_query: Single<&mut Transform, (With<HawkEntity>, Without<CrowEntity>)>,
    crow_query: Single<(&Transform, &Active), With<CrowEntity>>,
) {
    // checks if the crow is currently active
    if crow_query.1.active {
        // calculates path to crow and moves hawk accordingly.
        if hawk_query.translation.y <= crow_query.0.translation.y
            && hawk_query.translation.x == crow_query.0.translation.x
        {
            hawk_query.rotation = Quat::from_rotation_z(0.0_f32.to_radians());
            hawk_query.translation.y += 0.5;
        } else if hawk_query.translation.x >= crow_query.0.translation.x
            && hawk_query.translation.y == crow_query.0.translation.y
        {
            hawk_query.rotation = Quat::from_rotation_z(90.0_f32.to_radians());
            hawk_query.translation.x -= 0.5;
        } else if hawk_query.translation.x <= crow_query.0.translation.x
            && hawk_query.translation.y == crow_query.0.translation.y
        {
            hawk_query.rotation = Quat::from_rotation_z(270.0_f32.to_radians());
            hawk_query.translation.x += 0.5;
        } else if hawk_query.translation.y >= crow_query.0.translation.y
            && hawk_query.translation.x == crow_query.0.translation.x
        {
            hawk_query.rotation = Quat::from_rotation_z(180.0_f32.to_radians());
            hawk_query.translation.y -= 0.5;
        } else if hawk_query.translation.y <= crow_query.0.translation.y
            && hawk_query.translation.x >= crow_query.0.translation.x
        {
            hawk_query.rotation = Quat::from_rotation_z(45.0_f32.to_radians());
            hawk_query.translation.y += 0.50;
            hawk_query.translation.x -= 0.50;
        } else if hawk_query.translation.y < crow_query.0.translation.y
            && hawk_query.translation.x < crow_query.0.translation.x
        {
            hawk_query.rotation = Quat::from_rotation_z(315.0_f32.to_radians());
            hawk_query.translation.y += 0.50;
            hawk_query.translation.x += 0.50;
        } else if hawk_query.translation.y > crow_query.0.translation.y
            && hawk_query.translation.x > crow_query.0.translation.x
        {
            hawk_query.rotation = Quat::from_rotation_z(135.0_f32.to_radians());
            hawk_query.translation.y -= 0.50;
            hawk_query.translation.x -= 0.50;
        } else if hawk_query.translation.y > crow_query.0.translation.y
            && hawk_query.translation.x < crow_query.0.translation.x
        {
            hawk_query.rotation = Quat::from_rotation_z(225.0_f32.to_radians());
            hawk_query.translation.y -= 0.50;
            hawk_query.translation.x += 0.50;
        }
    }
}

/// ## fn success_check() : Update  
/// Checks if the crow has collected 10 worms.  
/// If so Status.event_id is set to 4 (success condition credit sequence active)  
fn success_check(
    mut worm_query: Single<&mut Count, With<WormEntity>>,
    mut status_query: Single<&mut Status, With<BackgroundEntity>>,
) {
    // checks for when the player has collected 10 worms.
    if worm_query.count >= MAXSCORE {
        status_query.event_id = 4;
        worm_query.count = 0;
    }
}

/// ## fn reset() : Update  
/// Automatically checks the Status.event_id for event updates  
/// 0 = Inactive/Reset  
/// 1 = Active  
/// 2 = Fail Condition  
/// 3 = Success Condition (Credit Sequence Ended)  
/// 4 = Success Condition (Credit Sequence Active)  
/// 5 = Game Over Lock Out  
/// If status 2 or 3 is reported, sprites are reset back to default settings (location, counters, active state)  
/// and status is set to either 0 (inactive) or 5 (game over lock out)  
#[allow(clippy::type_complexity)]
fn reset(
    mut commands: Commands,
    mut event_query: Single<&mut Status, With<BackgroundEntity>>,
    mut crow_query: Single<(&mut Transform, &mut Active), (With<CrowEntity>, Without<HawkEntity>)>,
    mut worm_query: Single<(&mut Active, &mut Count), (With<WormEntity>, Without<CrowEntity>)>,
    mut hawk_query: Single<&mut Transform, (With<HawkEntity>, Without<CrowEntity>)>,
) {
    // checks for event_id == 2 (loss condition)
    // if true, spawns game_over text entitity.
    if event_query.event_id == 2 {
        commands.spawn((
            Text::new("GAME OVER"),
            TextFont {
                font_size: 50.0,
                ..Default::default()
            },
            TextColor(RED.into()),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px((HEIGHT / 2.) + 150.),
                right: Val::Px((WIDTH / 2.) - 135.),
                ..default()
            },
            TextEntity,
        ));
    }
    // checks for event_id 2 (loss condition) or 3 (success credit sequence ended).
    if (event_query.event_id == 2 && crow_query.1.active) || event_query.event_id == 3 {
        // checks if it was a loss, sets event_id to 5 (game over lockout).
        if event_query.event_id == 2 {
            event_query.event_id = 5;
        // else sets event_id to 0 (inactive/reset).
        } else {
            event_query.event_id = 0;
        }
        // resets sprites and counters to default.
        crow_query.1.active = false;
        crow_query.0.translation = Vec3::new(425., -300., 2.);
        crow_query.0.rotation = Quat::from_rotation_z(0.0_f32.to_radians());
        worm_query.0.active = false;
        worm_query.1.count = 0;
        hawk_query.translation = Vec3::new(0., 0., 3.);
        hawk_query.rotation = Quat::from_rotation_z(0.0_f32.to_radians());
    }
}

/// ## fn despawn_text() : Update  
/// Despawns game over and credit text entities  
/// if still alive when the player moves starting a new game
fn despawn_text(
    mut commands: Commands,
    status_query: Single<&Status, With<BackgroundEntity>>,
    text_query: Query<Entity, With<TextEntity>>,
) {
    // checks for Status.event_id == 1 (active).
    if status_query.event_id == 1 {
        // despawns any text entities.
        for entity in text_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

/// ## fn success_sequence() : Update  
/// Plays a quick credit sequence, once 10 worms are collected.  
/// Then sets Status.event_id to 3 (credit sequence ended).
#[allow(clippy::type_complexity)]
fn success_sequence(
    mut commands: Commands,
    mut crow_query: Single<
        (&mut Transform, &mut Active),
        (With<CrowEntity>, Without<WormEntity>, Without<HawkEntity>),
    >,
    mut worm_query: Single<
        &mut Transform,
        (With<WormEntity>, Without<HawkEntity>, Without<CrowEntity>),
    >,
    mut hawk_query: Single<
        &mut Transform,
        (With<HawkEntity>, Without<WormEntity>, Without<CrowEntity>),
    >,
    mut status_query: Single<&mut Status, With<BackgroundEntity>>,
) {
    // checks if Status.event_id == 4 (success credit sequence) and if crow is active.
    // this means that the credit sequence is called but not yet started.
    // Spawns credit text.
    if status_query.event_id == 4 && crow_query.1.active {
        crow_query.1.active = false;
        worm_query.translation.x = 2000.;
        hawk_query.translation.x = 2000.;
        crow_query.0.rotation = Quat::from_rotation_z(270.0_f32.to_radians());
        commands.spawn((
            Text::new("Game By: Hayden Blanchard"),
            TextFont {
                font_size: 25.0,
                ..Default::default()
            },
            TextColor(YELLOW.into()),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(230.0),
                right: Val::Px(35.0),
                ..default()
            },
            TextEntity,
        ));

        commands.spawn((
            Text::new("A Crow's Commute"),
            TextFont {
                font_size: 35.0,
                ..Default::default()
            },
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(260.0),
                right: Val::Px(100.0),
                ..default()
            },
            TextEntity,
        ));
    // checks if credit sequence is active and crow is inactive.
    // this means that the credit sequence is active and the credit text is already spawned.
    // Then Moves player toward credit text, until it reaches the park with the other crows.
    // Then event_id is set to 3 (success credit sequence ended).
    } else if status_query.event_id == 4 && !crow_query.1.active {
        if crow_query.0.translation.x < 425. {
            crow_query.0.translation.x += 1.;
        } else {
            status_query.event_id = 3;
        }
    }
}
