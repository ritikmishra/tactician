use bevy::{audio::Decodable, math::Vec2};

pub struct FPSCount;

#[derive(Debug, Default)]
pub struct Position(pub Vec2);

#[derive(Debug)]
pub struct Mass(pub f32);

impl std::default::Default for Mass {
    /// Default mass of 1 kg
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Debug, Default)]
pub struct Velocity(pub Vec2);

/// Component for entities that can move themselves
/// (i.e they have an engine to accelerate + decelerate)
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Size(pub f32);

impl std::default::Default for Size {
    /// Default size of 1
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Debug, Default)]
pub struct Ship;

#[derive(Debug, Default)]
pub struct Missile;

#[derive(Debug, Default)]
pub struct Star;

#[derive(Debug, Default)]
pub struct Planet;

#[derive(Debug, Default)]
pub struct GravitySource;
