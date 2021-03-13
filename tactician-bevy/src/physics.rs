use bevy::prelude::*;

use std::ops::Add;

use crate::components::{EnginePhysics, GravitySource, Mass, Planet, Position, Star, Velocity};

/// Gravitational constant -- should probably be adjustable or something
pub const G: f32 = 0.000000001;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(apply_gravity_from_planets_to_ships.system())
            .add_system(apply_gravity_among_planets.system())
            .add_system(move_objects.system())
            .add_system(apply_engine_acceleration.system())
            .add_system(move_sprite_to_physics_pos.system())
            .add_system(rotate_sprite_for_components_with_engine.system());
    }
}

/// Makes the Position used in the physics simulation and the Transform used to render the sprite
/// refer to the same physical location
fn move_sprite_to_physics_pos(mut physics_sprite: Query<(&mut Transform, &Position)>) {
    for (mut sprite_pos, Position(physics_pos)) in physics_sprite.iter_mut() {
        sprite_pos.translation = (*physics_pos, 1.).into();
    }
}

fn rotate_sprite_for_components_with_engine(mut engine_sprite: Query<(&mut Transform, &Velocity)>) {
    for (mut sprite_transform, velocity) in engine_sprite.iter_mut() {
        sprite_transform.rotation =
            Quat::from_rotation_z(-velocity.0.angle_between(Vec2::unit_y()));
    }
}

fn apply_gravity_from_planets_to_ships(
    planets: Query<(&Position, &Mass), With<GravitySource>>,
    mut ships: Query<(&Position, &mut Velocity), Without<GravitySource>>,
    time: Res<Time>,
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

fn apply_engine_acceleration(mut objects: Query<(&mut Velocity, &EnginePhysics)>, dt: Res<Time>) {
    for (mut vel, engine) in objects.iter_mut() {
        let accel_vec = engine.current_accel * vel.0;
        vel.0 += dt.delta_seconds() * accel_vec;
    }
}

fn apply_gravity_among_planets(
    stars: Query<(Entity, &Position, &Mass), With<Star>>,
    planets: Query<(Entity, &Position, &Mass, &mut Velocity), With<Planet>>,
    time: Res<Time>,
) {
    // FIXME: uses aliased mutability :/
    unsafe {
        for mut planet in planets.iter_unsafe() {
            let mut new_accel = Vec2::zero();
            for gravity_source in planets
                .iter_unsafe()
                .map(|(a, b, c, _)| (a, b, c))
                .chain(stars.iter())
            {
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
            planet.3 .0 += new_accel * time.delta_seconds();
        }
    }
}
