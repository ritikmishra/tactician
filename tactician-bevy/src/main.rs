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
        .add_system(sprite_motion_system.system())
        .add_system(hello_world.system())
        .run();
}

struct FPSCount;

struct Position(Vec2);
struct Mass(f32);
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

    // TODO: add sprite

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
        }); // TODO: add sprite

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
    mut ships: Query<(&mut Position, &mut Velocity), Without<GravitySource>>,
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

fn move_objects(mut objects: Query<(&mut Position, &Velocity)>, dt: Res<Time>) {
    for (mut pos, Velocity(vel)) in objects.iter_mut() {
        let pos_delta = dt.delta_seconds() * (*vel);
        pos.0 += pos_delta;
    }
}

fn apply_gravity_to_missiles_and_ships(
    mut q: QuerySet<(
        Query<&Mass, With<GravitySource>>,
        Query<(&mut Position, &mut Mass, &mut Velocity)>,
    )>,
) {
    let x = q.q0_mut().iter_mut();
    // // Step 1: Update ships based on planets, sun
    // for ship in self.ships.iter_mut() {
    //     let mut aggregate_force = self
    //         .planets
    //         .iter()
    //         .map(|planet| planet.calculate_force_applied_to_object(&ship.phys))
    //         .fold(Vector2::new(0.0, 0.0), |acc, force| force + acc);
    //     aggregate_force += self.sun.calculate_force_applied_to_object(&ship.phys);

    //     let vel_change = aggregate_force / ship.phys.mass * interval;

    //     // To have good numerical integration, we multiply the velocity change in half
    //     // to account for the fact that, at the beginning of this time interval, the
    //     // velocity change was 0. This assumes that the velocity increased linearly over time
    //     // which is a better estimate than assuming that it discontinuously jumps around
    //     ship.phys.velocity += vel_change * 0.5;
    //     ship.phys.pos += ship.phys.velocity * interval;
    // }

    // // Step 2: Update planets based on each other + sun
    // for this_planet_id in 0..self.planets.len() {
    //     let this_planet = self.planets.get(this_planet_id).unwrap();

    //     // sum of force applied by fellow planets + sun
    //     let mut aggregate_force = self
    //         .planets
    //         .iter()
    //         .enumerate()
    //         .map(|(i, other_planet)| {
    //             if i != this_planet_id {
    //                 return other_planet.calculate_force_applied_to_object(&this_planet.phys);
    //             }
    //             return Vector2::new(0.0, 0.0);
    //         })
    //         .fold(Vector2::new(0.0, 0.0), |acc, force| force + acc);
    //     aggregate_force += self.sun.calculate_force_applied_to_object(&this_planet.phys);

    //     let vel_change = aggregate_force / this_planet.phys.mass * interval;
    //     let this_planet = self.planets.get_mut(this_planet_id).unwrap();
    //     this_planet.phys.velocity += vel_change * 0.5;
    //     this_planet.phys.pos += this_planet.phys.velocity * interval;
}

fn enforce_size(mut size_sprite: Query<(&mut Transform, &Size)>) {
    for (mut sprite_pos, Size(size)) in size_sprite.iter_mut() {
        sprite_pos.scale = Vec3::splat(*size);
    }
}
