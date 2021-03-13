use bevy::math::Vec2;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::camera::Camera,
};
use bevy_prototype_lyon::{prelude::*, utils::Convert};
use bundles::*;
use components::Size;
use components::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod bundles;
mod components;
mod physics;
use physics::PhysicsPlugin;

#[cfg(all(not(feature = "wasm"), not(feature = "native")))]
compile_error!("You have to build this binary (tactician-bevy) with either the 'wasm' feature or 'native' feature");

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn run_game() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "wasm_panic")]
    {
        use console_error_panic_hook;
        console_error_panic_hook::set_once();
    }

    #[cfg(feature = "wasm")]
    {
        use bevy_webgl2;
        app.add_plugin(bevy_webgl2::WebGL2Plugin);
    }

    app.add_plugin(ShapePlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(initialize_components.system())
        .add_system(handle_window_zoom.system())
        .add_system(enforce_size.system())
        .add_system(connect_ship_acceleration_to_user_input.system())
        .add_system(fps_counter.system())
        .add_system(kill_out_of_bounds_missiles.system())
        .add_system(kill_expired_objects.system())
        .add_system(update_missilecount.system())
        .add_system(follow_ship.system())
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
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.transform.scale = Vec3::splat(5.0);
    commands
        .spawn(camera_bundle)
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
            mass: Mass(3e15),
            ..Default::default()
        })
        .with_bundle(SpriteBundle {
            material: planet_material.clone(),
            ..Default::default()
        });

    const max: u32 = 5;
    const diam: f32 = 500.;
    const speed: f32 = 90.;
    for i in 0..max {
        let angle = std::f32::consts::PI * 2.0 * (i as f32) / (max as f32);
        let planet_pos = Vec2::new(angle.sin() * (diam), angle.cos() * (diam));
        commands
            .spawn(PlanetBundle {
                position: Position(planet_pos),
                mass: Mass(1e15),
                velocity: Velocity(Vec2::new(-angle.cos() * speed, angle.sin() * speed)),
                ..Default::default()
            })
            .with_bundle(SpriteBundle {
                material: planet_material.clone(),
                ..Default::default()
            });

        commands
            .spawn(PlanetBundle {
                position: Position(planet_pos + Vec2::new(45., 45.,)),
                mass: Mass(1e9),
                velocity: Velocity(Vec2::new(-20.0, -1.0)),
                size: Size(0.5),
                ..Default::default()
            })
            .with_bundle(SpriteBundle {
                material: planet_material.clone(),
                ..Default::default()
            });
    }

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
            flex_direction: FlexDirection::Column,
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

    commands.spawn((MissileCount,)).with_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexStart,
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        text: Text {
            value: "missileCount".into(),
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

fn update_missilecount(
    missiles: Query<Entity, With<Missile>>,
    mut missile_count_texts: Query<&mut Text, With<MissileCount>>,
) {
    let missile_count = missiles.iter().count();
    for mut missile_count_text in missile_count_texts.iter_mut() {
        missile_count_text.value = format!("{}", missile_count);
    }
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
        trail.points.push(pos.0.convert());

        // TODO: remove magic number
        if trail.points.len() > trail.max_points {
            trail.points.remove(0);
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
    time: Res<Time>,
) {
    if let Some(mut val) = query.iter_mut().next() {
        if keyboard_input.pressed(KeyCode::Up) {
            val.current_accel = 1.;
        } else if keyboard_input.pressed(KeyCode::Down) {
            val.current_accel = -1.;
        } else {
            val.current_accel = 0.;
        }

        if keyboard_input.pressed(KeyCode::Space) {
            if let Some((ship_pos, ship_vel)) = ship.iter().next() {
                commands
                    .spawn(MissileBundle {
                        position: ship_pos.clone(),
                        velocity: Velocity(ship_vel.0 + (45.0 * ship_vel.0.normalize())),
                        size: Size(0.17),
                        lifespan: Lifespan {
                            created_on: time.seconds_since_startup(),
                            lifespan: 150000.0,
                        },
                        snail_trail: SnailTrail {
                            max_points: 30,
                            points: Vec::with_capacity(3),
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

fn kill_expired_objects(
    commands: &mut Commands,
    time: Res<Time>,
    lifespan_objects: Query<(Entity, &Lifespan)>,
) {
    for (id, lifespan) in lifespan_objects.iter() {
        if lifespan.created_on + lifespan.lifespan < time.seconds_since_startup() {
            commands.despawn(id);
        }
    }
}

fn kill_out_of_bounds_missiles(
    commands: &mut Commands,
    cam_trans_query: Query<&Transform, With<Camera>>,
    window: Res<Windows>,
    missiles: Query<(Entity, &Transform), With<Missile>>,
) {
    let cam_scale;
    let cam_offset;
    if let Some(cam_trans) = cam_trans_query.iter().next() {
        cam_scale = cam_trans.scale.max_element();
        cam_offset = cam_trans.translation;
    } else {
        return;
    }
    let window = window.get_primary().unwrap();
    let (half_width, half_height) = (
        window.width() * cam_scale / 2.0,
        window.height() * cam_scale / 2.0,
    );
    for (missile_id, missile_pos) in missiles.iter() {
        if !(-half_width < missile_pos.translation.x - cam_offset.x
            && missile_pos.translation.x - cam_offset.x < half_width)
            || !(-half_height < missile_pos.translation.y - cam_offset.y
                && missile_pos.translation.y - cam_offset.y < half_height)
        {
            commands.despawn(missile_id);
        }
    }
}

fn handle_window_zoom(
    keyboard_input: ResMut<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    if let Some(mut cam) = camera.iter_mut().next() {
        // scale vec should always have x == y == z. so if x == y == z == 1, length squared == 3
        if keyboard_input.pressed(KeyCode::Equals) && cam.scale.length_squared() > 3. {
            cam.scale = cam.scale / 1.01;
        } else if keyboard_input.pressed(KeyCode::Minus) && cam.scale.length_squared() < 200. {
            cam.scale = cam.scale * 1.01;
        }
    }
}

fn follow_ship(
    mut camera: Query<&mut Transform, With<Camera>>,
    ship: Query<&Transform, With<Ship>>,
) {
    if let (Some(mut cam_trans), Some(ship_trans)) = (camera.iter_mut().next(), ship.iter().next())
    {
        cam_trans.translation = ship_trans.translation;
    }
}
