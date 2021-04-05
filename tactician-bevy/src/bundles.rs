use crate::components::*;
use bevy::{
    core::Timer,
    prelude::{Bundle, Handle, TextureAtlas},
    sprite::ColorMaterial,
};
use bevy_prototype_lyon::prelude::Geometry;
use lyon::{
    geom::euclid::default::Point2D,
    path::{path::Builder, traits::PathBuilder, Polygon},
};

#[derive(Bundle, Default)]
pub struct StarBundle {
    pub position: Position,
    pub mass: Mass,
    pub size: Size,
    pub star: Star,
    pub gravity_source: GravitySource,
}

#[derive(Bundle, Default)]
pub struct PlanetBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub mass: Mass,

    pub size: Size,

    pub planet: Planet,
    pub gravity_source: GravitySource,
    pub snail_trail: SnailTrail,
}

#[derive(Bundle, Default)]
pub struct ShipBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub mass: Mass,

    pub size: Size,
    pub engine: EnginePhysics,

    pub ship: Ship,
    pub team: Team,
    pub snail_trail: SnailTrail,
}

#[derive(Debug, Bundle, Default)]
pub struct MissileBundle {
    pub snail_trail: SnailTrail,
    pub position: Position,
    pub velocity: Velocity,
    pub size: Size,
    pub lifespan: Lifespan,
    pub missile: Missile,
    pub team: Team,
}

#[derive(Debug, Bundle)]
pub struct ExplosionBundle {
    pub explosion: Explosion,
    pub animate_once: AnimateOnce,
    pub animate_timer: Timer,
    pub position: Position,
    pub velocity: Velocity,
    pub size: Size,
}

impl Default for ExplosionBundle {
    fn default() -> Self {
        Self {
            explosion: Explosion::default(),
            animate_once: AnimateOnce::default(),
            animate_timer: Timer::from_seconds(1.0 / 60.0, true),
            position: Position::default(),
            velocity: Velocity::default(),
            size: Size::default(),
        }
    }
}

#[derive(Debug)]
pub struct SnailTrail {
    pub points: Vec<Point2D<f32>>,
    pub max_points: usize,
}

impl Default for SnailTrail {
    fn default() -> Self {
        Self {
            points: Vec::with_capacity(500),
            max_points: 7000,
        }
    }
}

impl Geometry for SnailTrail {
    fn add_geometry(&self, b: &mut Builder) {
        b.add_polygon(Polygon {
            points: self.points.as_slice(),
            closed: false,
        })
    }
}

pub struct Materials {
    pub ship_mat_handle: Handle<ColorMaterial>,
    pub planet_mat_handle: Handle<ColorMaterial>,
    pub missile_mat_handle: Handle<ColorMaterial>,
    pub explosion_spritesheet_handle: Handle<TextureAtlas>,
}

/// The snail trail component is separate from the actual ship/planet/missile
/// The entire trail is despawned and redrawn
pub struct SnailTrailEntityMarker;
