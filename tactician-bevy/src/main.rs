use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::math::Vec2;
use bevy::{
    diagnostic::{DiagnosticId, Diagnostics},
    prelude::*,
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(initialize_components.system())
        .add_system(enforce_size.system())
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
        .spawn((Star, Position(Vec2::new(0., 0.)), Mass(1e15), Size(1.0)))
        .with_bundle(SpriteBundle {
            material: planet_material.clone(),
            ..Default::default()
        });

    commands
        .spawn((
            Planet,
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
                color: Color::BLACK,
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

fn enforce_size(mut size_sprite: Query<(&mut Transform, &Size)>) {
    for (mut sprite_pos, Size(size)) in size_sprite.iter_mut() {
        sprite_pos.scale = Vec3::splat(*size);
    }
}
