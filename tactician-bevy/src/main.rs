use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::math::Vec2;
use bevy::{
    diagnostic::{DiagnosticId, Diagnostics},
    prelude::*,
};
use std::ops::Add;

/// Gravitational constant -- should probably be adjustable or something
pub const G: f32 = 0.0000000006;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(initialize_components.system())
        .add_system(enforce_size.system())
        .add_system(move_objects.system())
        .add_system(apply_gravity_from_planets_to_ships.system())
        .add_system(apply_gravity_among_planets.system())
        .add_system(sprite_motion_system.system())
        .add_system(hello_world.system())
        .run();
}

struct FPSCount;

#[derive(Debug)]
struct Position(Vec2);
#[derive(Debug)]
struct Mass(f32);

#[derive(Debug)]
struct Velocity(Vec2);

/// Component for entities that can move themselves
/// (i.e they have an engine to accelerate + decelerate)
struct EnginePhysics {
    max_accel: f32,
    current_accel: f32,
}

/// Component for entities that should be displayed at a certain size
/// These circles should also have physics pos
struct Size(f32);

struct Ship;
struct Missile;
struct Star;
struct Planet;

struct GravitySource;

const FONT: &str = "fonts/FiraMono-Medium.ttf";

fn initialize_components(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // create the ui
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());

    let planet_handle = asset_server.load("images/planet.png");
    let planet_material = materials.add(planet_handle.into());

    let missile_handle = asset_server.load("images/missile.png");
    let missile_material = materials.add(missile_handle.into());

    let ship_handle = asset_server.load("images/ship.png");
    let ship_material = materials.add(ship_handle.into());

    commands
        .spawn((
            Star,
            GravitySource,
            Position(Vec2::new(0., 0.)),
            Mass(1e15),
            Size(1.0),
        ))
        .with_bundle(SpriteBundle {
            material: planet_material.clone(),
            ..Default::default()
        });

    commands
        .spawn((
            Planet,
            GravitySource,
            Position(Vec2::new(0.0, 250.0)),
            Mass(1e14),
            Velocity(Vec2::new(-40.0, 0.0)),
            Size(1.0),
        ))
        .with_bundle(SpriteBundle {
            material: planet_material.clone(),
            ..Default::default()
        });

    commands
        .spawn((
            Planet,
            GravitySource,
            Position(Vec2::new(-80.0, 320.0)),
            Mass(1e5),
            Velocity(Vec2::new(-20.0, -1.0)),
            Size(0.5),
        ))
        .with_bundle(SpriteBundle {
            material: planet_material,
            ..Default::default()
        });

    commands
        .spawn((
            Ship,
            Position(Vec2::new(-70., 240.)),
            Mass(30.),
            Velocity(Vec2::new(-20.0, -20.0)),
            Size(0.3),
            EnginePhysics {
                current_accel: 0.0,
                max_accel: 3.0,
            },
        ))
        .with_bundle(SpriteBundle {
            material: ship_material,
            ..Default::default()
        });

    // create the fps counter
    commands.spawn((FPSCount,)).with_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        },
        text: Text {
            value: "FPS counter".to_string(),
            font: asset_server.load(FONT),
            style: TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..Default::default()
            },
        },
        ..Default::default()
    });
}

fn hello_world(time: Res<Diagnostics>, mut texts: Query<&mut Text, With<FPSCount>>) {
    if let Some(fps_stats) = time.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_num) = fps_stats.average() {
            for mut text in texts.iter_mut() {
                text.value = format!("{:.2}", fps_num);
            }
        }
    }
}

fn sprite_motion_system(mut physics_sprite: Query<(&mut Transform, &Position)>) {
    for (mut sprite_pos, physics_pos) in physics_sprite.iter_mut() {
        sprite_pos.translation = (physics_pos.0, 0.).into();
    }
}

fn apply_gravity_from_planets_to_ships(
    planets: Query<(&Position, &Mass), With<GravitySource>>,
    mut ships: Query<(&Position, &mut Velocity), Without<GravitySource>>,
    time: Res<Time>
) {
    for (ship_pos, mut ship_vel) in ships.iter_mut() {
        let aggregate_grav_accel = planets
            .iter()
            .map(|(Position(p_pos), Mass(p_mass))| {
                // points from ship to planet
                let pos_delta: Vec2 = *p_pos - ship_pos.0;
                let dist2 = pos_delta.length_squared();

                let accel_direction = pos_delta.normalize();

                // don't multiply by ship mass - we want acceleration on ship (F = ma)
                let accel_magnitude = G * p_mass / dist2;
                return accel_direction * accel_magnitude;
            })
            .fold(Vec2::zero(), Vec2::add);

        ship_vel.0 += aggregate_grav_accel * time.delta_seconds();
    }
}

/// For all objects that have a position and a velocity, it moves the object 
/// according to the velocity and the time elapsed
fn move_objects(mut objects: Query<(&mut Position, &Velocity)>, dt: Res<Time>) {
    for (mut pos, Velocity(vel)) in objects.iter_mut() {
        let pos_delta = dt.delta_seconds() * (*vel);
        pos.0 += pos_delta;
    }
}

fn apply_gravity_among_planets(
    stars: Query<(Entity, &Position, &Mass), With<Star>>,
    mut planets: Query<(Entity, &Position, &Mass, &mut Velocity), With<Planet>>,
    time: Res<Time>
) {
    // FIXME: uses aliased mutability :/
    unsafe {
        for mut planet in planets.iter_unsafe() {
            let mut new_accel = Vec2::zero();
            for gravity_source in planets.iter_unsafe().map(|(a, b, c, _)| (a, b, c)).chain(stars.iter()) {
                if planet.0 != gravity_source.0 {
                    let planet_position = (*planet.1).0;
                    let gravity_source_pos = (*gravity_source.1).0;

                    // TODO: this code is wet - same as the ship gravity impl, maybe we can combine the systems?
                    // points from the planet to the gravity source
                    let pos_delta: Vec2 = gravity_source_pos - planet_position;
                    let dist2 = pos_delta.length_squared();
    
                    let accel_direction = pos_delta.normalize();
    
                    // don't multiply by ship mass - we want acceleration on ship (F = ma)
                    let gravity_source_mass = (*gravity_source.2).0;
                    let accel_magnitude = G * gravity_source_mass / dist2;

                    new_accel += accel_direction * accel_magnitude;
                }
            }
            planet.3.0 += new_accel * time.delta_seconds();
        }
    }
}

fn enforce_size(mut size_sprite: Query<(&mut Transform, &Size)>) {
    for (mut sprite_pos, Size(size)) in size_sprite.iter_mut() {
        sprite_pos.scale = Vec3::splat(*size);
    }
}
