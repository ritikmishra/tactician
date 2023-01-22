use std::num::NonZeroU32;

use bevy::{math::Vec2, prelude::Component};

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct FPSCount;
#[derive(Debug, Default, Clone, Copy, Component)]
pub struct MissileCount;

#[derive(Debug, Default, Clone, Component)]
pub struct Position(pub Vec2);

#[derive(Debug, Clone, Component)]
pub struct Mass(pub f32);

impl std::default::Default for Mass {
    /// Default mass of 1 kg
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Debug, Default, Clone, Component)]
pub struct Velocity(pub Vec2);

/// Component for entities that can move themselves
/// (i.e they have an engine to accelerate + decelerate)
#[derive(Debug, Component)]
pub struct EnginePhysics {
    // m/s^2
    pub max_accel: f32,

    // m/s^2
    pub current_accel: f32,
}

impl std::default::Default for EnginePhysics {
    fn default() -> Self {
        Self {
            max_accel: 10.,
            current_accel: 0.
        }
    }
}


/// Component for entities that should be displayed at a certain size
/// These circles should also have physics pos
#[derive(Debug, Component)]
pub struct Size(pub f32);

impl std::default::Default for Size {
    /// Default size of 1
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Debug, Default, Component)]
pub struct Ship;

#[derive(Debug, Default, Component)]
pub struct Missile;

#[derive(Debug, Default, Component)]
pub struct Explosion;

#[derive(Debug, Default, Component)]
pub struct Star;

#[derive(Debug, Default, Component)]
pub struct Planet;

#[derive(Debug, Default, Component)]
pub struct GravitySource;

#[derive(Debug, Clone, Default, PartialEq, Eq, Component)]
pub struct Team(pub Option<NonZeroU32>);

#[derive(Debug, Default, Component)]
pub struct Lifespan {
    /// Seconds since program startup that this component was created on
    pub created_on: f64,

    /// Number of seconds for which this item should be alive
    pub lifespan: f64,
}

#[derive(Debug, Default, Component)]
pub struct AnimateOnce;

#[derive(Debug, Clone, Copy, Component)]
pub struct ShipCamera;
