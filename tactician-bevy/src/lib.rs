use bevy::{diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}, prelude::*, transform};
use bevy::{input::keyboard, math::Vec2};
use bevy_prototype_lyon::{prelude::*, utils::Convert};
use bundles::*;
use components::Size;
use components::*;
use std::ops::Add;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod bundles;
mod components;
mod physics;
use physics::PhysicsPlugin;

#[cfg(all(not(feature="wasm"), not(feature="native")))]
compile_error!("You have to build this binary (tactician-bevy) with either the 'wasm' feature or 'native' feature");

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn run_game() {

    let mut app = App::build();
    app.add_plugins(DefaultPlugins);

    #[cfg(feature="wasm_panic")]
    {
        use console_error_panic_hook;
        console_error_panic_hook::set_once();
    }

    #[cfg(feature="wasm")]
    {
        use bevy_webgl2;
        app.add_plugin(bevy_webgl2::WebGL2Plugin);
    }

    app
        .add_plugin(ShapePlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(initialize_components.system())
        .add_system(enforce_size.system())
        .add_system(connect_ship_acceleration_to_user_input.system())
        .add_system(fps_counter.system())
        .add_system(kill_expired_objects.system())
        .add_system(render_snailtrail.system())
        .run();
}

const FONT: &str = "fonts/FiraMono-Medium.ttf";

fn initialize_components(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // create the ui
    // camera2dbundle is needed for the 2d rendering
    // camerauibundle is needed for text/UI element rendering
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());

    let planet_handle = asset_server.load("images/planet.png");
    let planet_material = materials.add(planet_handle.into());

    let missile_handle = asset_server.load("images/missile.png");
    let missile_material = materials.add(missile_handle.into());

    let ship_handle = asset_server.load("images/ship.png");
    let ship_material = materials.add(ship_handle.into());

    commands.insert_resource(Materials {
        ship_mat_handle: ship_material.clone(),
        planet_mat_handle: planet_material.clone(),
        missile_mat_handle: missile_material.clone(),
    });

    commands
        .spawn(StarBundle {
            position: Position(Vec2::new(0., 0.)),
            mass: Mass(1e15),
            ..Default::default()
        })
        .with_bundle(SpriteBundle {
            material: planet_material.clone(),
            ..Default::default()
        });

    commands
        .spawn(PlanetBundle {
            position: Position(Vec2::new(0.0, 250.0)),
            mass: Mass(1e14),
            velocity: Velocity(Vec2::new(-40., 0.)),
            ..Default::default()
        })
        .with_bundle(SpriteBundle {
            material: planet_material.clone(),
            ..Default::default()
        });

    commands
        .spawn(PlanetBundle {
            position: Position(Vec2::new(-80.0, 320.0)),
            mass: Mass(1e5),
            velocity: Velocity(Vec2::new(-20.0, -1.0)),
            size: Size(0.5),
            ..Default::default()
        })
        .with_bundle(SpriteBundle {
            material: planet_material,
            ..Default::default()
        });

    commands
        .spawn(ShipBundle {
            position: Position(Vec2::new(70., 240.)),
            mass: Mass(0.0001),
            velocity: Velocity(Vec2::new(40.0, -20.0)),
            size: Size(0.3),
            engine: EnginePhysics {
                current_accel: 0.0,
                max_accel: 3.0,
            },
            ..Default::default()
        })
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

fn render_snailtrail(
    commands: &mut Commands,
    mut objects_with_snail_trail: Query<(&Position, &mut SnailTrail)>,
    snail_trails: Query<Entity, With<SnailTrailEntityMarker>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // despawn the old snail trails
    for old_snail_trail in snail_trails.iter() {
        commands.despawn(old_snail_trail);
    }

    // update the snail trail (point vec) for the objects
    // and render the snail trail
    // TODO: only collect the position maybe 15 times a second? does not need to run every framemaybe only collect
    for (pos, mut trail) in objects_with_snail_trail.iter_mut() {
        trail.0.push(pos.0.convert());

        // TODO: remove magic number
        if trail.0.len() > 7000 {
            trail.0.remove(0);
        }
        commands
            .spawn(GeometryBuilder::build_as(
                &*trail,
                materials.add(ColorMaterial::color(Color::WHITE)),
                TessellationMode::Stroke(StrokeOptions::default()),
                Transform::default(),
            ))
            .with(SnailTrailEntityMarker);
    }
}

fn fps_counter(time: Res<Diagnostics>, mut texts: Query<&mut Text, With<FPSCount>>) {
    if let Some(fps_stats) = time.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_num) = fps_stats.average() {
            for mut text in texts.iter_mut() {
                text.value = format!("{:.2}", fps_num);
            }
        }
    }
}

fn enforce_size(mut size_sprite: Query<(&mut Transform, &Size)>) {
    for (mut sprite_pos, Size(size)) in size_sprite.iter_mut() {
        sprite_pos.scale = Vec3::splat(*size);
    }
}


// FIXME: yucky! this method has to handle missle spawning? gross!
// it should emit an event!
fn connect_ship_acceleration_to_user_input(
    commands: &mut Commands,
    mut query: Query<&mut EnginePhysics>,
    ship: Query<(&Position, &Velocity), With<Ship>>,
    keyboard_input: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    time: Res<Time>
) {
    if let Some(mut val) = query.iter_mut().next() {
        if keyboard_input.pressed(KeyCode::Up) {
            val.current_accel = 1.;
        } else if keyboard_input.pressed(KeyCode::Down) {
            val.current_accel = -1.;
        } else {
            val.current_accel = 0.;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            if let Some((ship_pos, ship_vel)) = ship.iter().next() {
                commands
                    .spawn(MissileBundle {
                        position: ship_pos.clone(),
                        velocity: Velocity(ship_vel.0 + (45.0 * ship_vel.0.normalize()) ),
                        size: Size(0.17),
                        lifespan: Lifespan {
                            created_on: time.seconds_since_startup(),
                            lifespan: 15.0
                        },
                        ..Default::default()
                    })
                    .with_bundle(SpriteBundle {
                        material: materials.missile_mat_handle.clone(),
                        transform: Transform {
                            translation: Vec3::new(ship_vel.0.x, ship_vel.0.y, 0.0),
                            scale: Vec3::zero(),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
            }
        }
    }
}


fn kill_expired_objects(commands: &mut Commands, time: Res<Time>, lifespan_objects: Query<(Entity, &Lifespan)>) {
    for (id, lifespan) in lifespan_objects.iter() {
        if lifespan.created_on + lifespan.lifespan < time.seconds_since_startup() {
            commands.despawn(id);
        }        
    }
}