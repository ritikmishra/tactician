use std::num::NonZeroU32;

use crate::bundles::*;
use crate::components::Size;
use crate::components::*;
use crate::events::*;
use crate::misc::AppState;
use crate::resources::*;
use bevy::math::Vec2;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_prototype_lyon::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod bundles;
mod components;
mod events;
mod menu;
mod misc;
mod physics;
mod resources;
use physics::PhysicsPlugin;

// #[cfg(all(not(feature = "wasm"), not(feature = "native")))]
// compile_error!("You have to build this binary (tactician-bevy) with either the 'wasm' feature or 'native' feature");

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run_game() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_plugin(ShapePlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .init_resource::<Typography>()
        .init_resource::<Materials>()
        .add_event::<SpawnMissileFromShip>()
        .add_event::<CreateExplosionEvent>();

    // Add default menu state
    app.add_state(AppState::Menu);

    app.add_startup_system(init_camera);

    // menu stuff
    app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(menu::init_menu))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu::update_menu))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(delete_all_entities));

    // In game stuff
    app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(initialize_components))
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(handle_window_zoom)
                .with_system(enforce_size)
                .with_system(animate_sprite_system)
                .with_system(connect_ship_acceleration_to_user_input)
                .with_system(fps_counter)
                .with_system(kill_out_of_bounds_missiles)
                .with_system(kill_expired_objects)
                .with_system(explode_missiles_near_planets)
                .with_system(handle_spawn_missile_event)
                .with_system(update_missilecount)
                .with_system(follow_ship)
                .with_system(render_snailtrail)
                .with_system(check_if_missile_should_kill_ship)
                .with_system(create_explosion),
        )
        .add_system_set(SystemSet::on_exit(AppState::Game).with_system(delete_all_entities));

    app.run()
}

fn init_camera(mut commands: Commands) {
    // create the ui
    // OrthographicCameraBundle is needed for the 2d rendering
    // uicamerabundle is needed for text/UI element rendering
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.transform.scale = Vec3::splat(5.0);
    commands.spawn_bundle(camera_bundle).insert(ShipCamera);
}

fn initialize_components(
    mut commands: Commands,
    typography: Res<Typography>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(StarBundle {
            position: Position(Vec2::new(0., 0.)),
            mass: Mass(3e15),
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("images/planet.png"),
            ..Default::default()
        });

    const max: u32 = 5;
    const diam: f32 = 500.;
    const speed: f32 = 90.;
    for i in 0..max {
        let angle = std::f32::consts::PI * 2.0 * (i as f32) / (max as f32);
        let planet_pos = Vec2::new(angle.sin() * (diam), angle.cos() * (diam));
        commands
            .spawn_bundle(PlanetBundle {
                position: Position(planet_pos),
                mass: Mass(1e15),
                velocity: Velocity(Vec2::new(-angle.cos() * speed, angle.sin() * speed)),
                ..Default::default()
            })
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("images/planet.png"),
                ..Default::default()
            });

        commands
            .spawn_bundle(PlanetBundle {
                position: Position(planet_pos + Vec2::new(45., 45.)),
                mass: Mass(1e9),
                velocity: Velocity(Vec2::new(-20.0, -1.0)),
                size: Size(0.5),
                ..Default::default()
            })
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("images/planet.png"),
                ..Default::default()
            });
    }

    commands
        .spawn_bundle(ShipBundle {
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
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("images/ship.png"),
            ..Default::default()
        });

    commands
        .spawn_bundle(ShipBundle {
            position: Position(Vec2::new(300., 0.)),
            mass: Mass(0.0001),
            velocity: Velocity(Vec2::new(0.0, -40.0)),
            size: Size(0.3),
            engine: EnginePhysics {
                current_accel: 0.0,
                max_accel: 3.0,
            },
            team: Team(NonZeroU32::new(3)),
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            // FIXME: enemy ships should use a different sprite/color
            texture: asset_server.load("images/ship.png"),
            ..Default::default()
        });

    // create the fps counter
    commands.spawn().insert(FPSCount).insert_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        text: Text::from_section("FPS Counter", typography.body.clone()),
        ..Default::default()
    });

    commands
        .spawn()
        .insert(MissileCount)
        .insert_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            text: Text::from_section("missileCount", typography.body.clone()),
            ..Default::default()
        });
}

fn update_missilecount(
    missiles: Query<Entity, With<Missile>>,
    mut missile_count_texts: Query<&mut Text, With<MissileCount>>,
) {
    let missile_count = missiles.iter().count();
    for mut missile_count_text in missile_count_texts.iter_mut() {
        missile_count_text.sections[0].value = format!("{}", missile_count);
    }
}

fn animate_sprite_system(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        Option<&AnimateOnce>,
    )>,
) {
    for (entity_id, mut timer, mut sprite, texture_atlas_handle, maybe_animate_once) in
        query.iter_mut()
    {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            let next_sprite_idx = (sprite.index as usize + 1) % texture_atlas.textures.len();

            // delete entity if it's only supposed to animate once (e.g like explosions)
            if next_sprite_idx == 0 && maybe_animate_once.is_some() {
                commands.entity(entity_id).despawn();
            } else {
                sprite.index = next_sprite_idx;
            }
        }
    }
}

fn render_snailtrail(
    mut commands: Commands,
    mut objects_with_snail_trail: Query<(&Position, &mut SnailTrail)>,
    snail_trails: Query<Entity, With<SnailTrailEntityMarker>>,
) {
    // despawn the old snail trails
    for old_snail_trail in snail_trails.iter() {
        commands.entity(old_snail_trail).despawn();
    }

    // update the snail trail (point vec) for the objects
    // and render the snail trail
    // TODO: only collect the position maybe 15 times a second? does not need to run every framemaybe only collect
    for (pos, mut trail) in objects_with_snail_trail.iter_mut() {
        trail.points.push(Into::<(f32, f32)>::into(pos.0).into());

        // TODO: remove magic number
        if trail.points.len() > trail.max_points {
            trail.points.remove(0);
        }
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &*trail,
                DrawMode::Stroke(StrokeMode {
                    options: StrokeOptions::default(),
                    color: Color::WHITE,
                }),
                Transform::default(),
            ))
            .insert(SnailTrailEntityMarker);
    }
}

fn fps_counter(time: Res<Diagnostics>, mut texts: Query<&mut Text, With<FPSCount>>) {
    if let Some(fps_stats) = time.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_num) = fps_stats.average() {
            for mut text in texts.iter_mut() {
                text.sections[0].value = format!("{:.2}", fps_num);
            }
        }
    }
}

fn enforce_size(mut size_sprite: Query<(&mut Transform, &Size)>) {
    for (mut sprite_pos, Size(size)) in size_sprite.iter_mut() {
        sprite_pos.scale = Vec3::splat(*size);
    }
}

fn connect_ship_acceleration_to_user_input(
    mut query: Query<&mut EnginePhysics>,
    ship: Query<(&Position, &Velocity, &Team), With<Ship>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut spawn_missile_event: EventWriter<SpawnMissileFromShip>,
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
            if let Some((ship_pos, ship_vel, ship_team)) = ship.iter().next() {
                spawn_missile_event.send(SpawnMissileFromShip {
                    position: ship_pos.clone(),
                    velocity: Velocity(ship_vel.0 + (45.0 * ship_vel.0.normalize())),
                    team: ship_team.clone(),
                });
            }
        }
    }
}

fn handle_spawn_missile_event(
    mut event_reader: EventReader<SpawnMissileFromShip>,
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for missile_spawn_request in event_reader.iter() {
        commands
            .spawn_bundle(MissileBundle {
                position: missile_spawn_request.position.clone(),
                velocity: missile_spawn_request.velocity.clone(),
                team: missile_spawn_request.team.clone(),
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
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("images/missile.png"),
                transform: Transform {
                    translation: Vec3::new(
                        missile_spawn_request.position.0.x,
                        missile_spawn_request.position.0.y,
                        0.0,
                    ),
                    scale: Vec3::ZERO,
                    ..Default::default()
                },
                ..Default::default()
            });
    }
}

fn kill_expired_objects(
    mut commands: Commands,
    time: Res<Time>,
    lifespan_objects: Query<(Entity, &Lifespan)>,
) {
    for (id, lifespan) in lifespan_objects.iter() {
        if lifespan.created_on + lifespan.lifespan < time.seconds_since_startup() {
            commands.entity(id).despawn();
        }
    }
}

fn check_if_missile_should_kill_ship(
    mut commands: Commands,
    ships: Query<(Entity, &Position, &Velocity, &Team, &Sprite, &Size), With<Ship>>,
    missiles: Query<(Entity, &Position, &Velocity, &Team), With<Missile>>,
    mut explosion_event: EventWriter<CreateExplosionEvent>,
) {
    for (
        ship_id,
        Position(ship_pos),
        Velocity(ship_vel),
        Team(ship_team_id),
        ship_sprite,
        Size(ship_size),
    ) in ships.iter()
    {
        for (missile_id, Position(missile_pos), Velocity(missile_vel), Team(missile_team_id)) in
            missiles.iter()
        {
            if ship_team_id != missile_team_id {
                let dist_from_ship_to_missile = (*ship_pos - *missile_pos).length();
                let ship_size = 50.0 * 0.5 * ship_size;
                if dist_from_ship_to_missile < ship_size {
                    commands.entity(ship_id).despawn();
                    commands.entity(missile_id).despawn();
                    explosion_event.send(CreateExplosionEvent {
                        position: Position(*ship_pos),
                        velocity: Velocity(*ship_vel + *missile_vel),
                    });
                }
            }
        }
    }
}

fn create_explosion(
    mut commands: Commands,
    mut event_reader: EventReader<CreateExplosionEvent>,
    materials: Res<Materials>,
) {
    for ev in event_reader.iter() {
        commands
            .spawn_bundle(ExplosionBundle {
                position: ev.position.clone(),
                velocity: ev.velocity.clone(),
                ..Default::default()
            })
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: materials.explosion_frames.clone(),
                ..Default::default()
            });
    }
}

fn explode_missiles_near_planets(
    mut commands: Commands,
    missiles: Query<(Entity, &Position, &Velocity), With<Missile>>,
    planets: Query<(&Position, &Transform, &Size), With<GravitySource>>,
    mut explosion_event: EventWriter<CreateExplosionEvent>,
) {
    for (missile_id, missile_pos, missile_vel) in missiles.iter() {
        'planets_loop: for (planet_pos, planet_sprite, Size(planet_size)) in planets.iter() {
            // Assumes that the planet is circular, this will need to get changed if we want lumpy asteroid type things or smth
            // tD
            let planet_radius = 50.0 * planet_size * 0.5;
            let distance_between_missile_and_planet = (planet_pos.0 - missile_pos.0).length();

            if distance_between_missile_and_planet < planet_radius {
                commands.entity(missile_id).despawn();
                explosion_event.send(CreateExplosionEvent {
                    position: missile_pos.clone(),
                    velocity: missile_vel.clone(),
                });
                break 'planets_loop;
            }
        }
    }
}

fn kill_out_of_bounds_missiles(
    mut commands: Commands,
    cam_trans_query: Query<&Transform, With<Camera>>,
    window: Res<Windows>,
    missiles: Query<(Entity, &Transform), With<Missile>>,
) {
    let cam_trans = cam_trans_query.single();
    let cam_scale = cam_trans.scale.max_element();
    let cam_offset = cam_trans.translation;

    let window = window.get_primary().unwrap();
    let (half_width, half_height) = (
        window.width() * cam_scale / 2.0,
        window.height() * cam_scale / 2.0,
    );
    for (missile_id, missile_pos) in missiles.iter() {
        if !(-half_width < missile_pos.translation.x - cam_offset.x
            && missile_pos.translation.x - cam_offset.x < half_width
            && -half_height < missile_pos.translation.y - cam_offset.y
            && missile_pos.translation.y - cam_offset.y < half_height)
        {
            commands.entity(missile_id).despawn();
        }
    }
}

fn handle_window_zoom(
    keyboard_input: ResMut<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<ShipCamera>>,
) {
    let mut cam = camera.single_mut();
    // scale vec should always have x == y == z. so if x == y == z == 1, length squared == 3
    if keyboard_input.pressed(KeyCode::Equals) && cam.scale.length_squared() > 3. {
        cam.scale /= 1.01;
    } else if keyboard_input.pressed(KeyCode::Minus) && cam.scale.length_squared() < 200. {
        cam.scale *= 1.01;
    }
}

fn follow_ship(
    mut camera: Query<&mut Transform, (With<ShipCamera>, Without<Ship>)>,
    ship: Query<&Transform, (With<Ship>, Without<ShipCamera>)>,
) {
    let ship_trans = ship.iter().next().unwrap().translation;
    camera.single_mut().translation = ship_trans;
}

fn delete_all_entities(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    entities
        .iter()
        .for_each(|e| commands.entity(e).despawn_recursive());
}
