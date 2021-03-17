use crate::components::*;
use bevy::{
    prelude::{Bundle, Handle},
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
    pub team: Team
}

#[derive(Debug, Bundle, Default)]
pub struct ExplosionBundle {
    pub explosion: Explosion,
    pub lifespan: Lifespan,
    pub position: Position,
    pub velocity: Velocity,
    pub size: Size
}

#[derive(Debug)]
pub struct SnailTrail {
    pub points: Vec<Point2D<f32>>,
    pub max_points: usize
}

impl Default for SnailTrail {
    fn default() -> Self {
        Self {
            points: Vec::with_capacity(500),
            max_points: 7000
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
}

/// The snail trail component is separate from the actual ship/planet/missile
/// The entire trail is despawned and redrawn
pub struct SnailTrailEntityMarker;
