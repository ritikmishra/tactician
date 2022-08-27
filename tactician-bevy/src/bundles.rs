use crate::components::*;
use bevy::{
    prelude::{Bundle, Component},
    time::Timer,
};
use bevy_prototype_lyon::prelude::{
    tess::path::{builder::PathBuilder, path::Builder, Polygon},
    Geometry,
};
use lyon_geom::euclid::default::Point2D;

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
    pub mass: Mass,
    pub lifespan: Lifespan,
    pub missile: Missile,
    pub team: Team,
}

#[derive(Debug, Component)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    pub fn tick(&mut self, duration: std::time::Duration) {
        self.0.tick(duration);
    }

    pub fn finished(&self) -> bool {
        self.0.finished()
    }
}

#[derive(Debug, Bundle)]
pub struct ExplosionBundle {
    pub explosion: Explosion,
    pub animate_once: AnimateOnce,
    pub animate_timer: AnimationTimer,
    pub position: Position,
    pub velocity: Velocity,
    pub size: Size,
}

impl Default for ExplosionBundle {
    fn default() -> Self {
        Self {
            explosion: Explosion::default(),
            animate_once: AnimateOnce::default(),
            animate_timer: AnimationTimer(Timer::from_seconds(1.0 / 60.0, true)),
            position: Position::default(),
            velocity: Velocity::default(),
            size: Size(0.25),
        }
    }
}

#[derive(Debug, Clone, Component)]
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

/// The snail trail component is separate from the actual ship/planet/missile
/// The entire trail is despawned and redrawn
#[derive(Debug, Clone, Copy, Component)]
pub struct SnailTrailEntityMarker;
