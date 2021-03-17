use bevy::math::Vec2;

pub struct FPSCount;

pub struct MissileCount;

#[derive(Debug, Default, Clone)]
pub struct Position(pub Vec2);

#[derive(Debug, Clone)]
pub struct Mass(pub f32);

impl std::default::Default for Mass {
    /// Default mass of 1 kg
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Clone)]
pub struct Team(pub String);

impl Default for Team {
    fn default() -> Self {
        Self("unknown team".to_string())
    }
}

#[derive(Debug, Default)]
pub struct Lifespan {
    /// Seconds since program startup that this component was created on
    pub created_on: f64,

    /// Number of seconds for which this item should be alive
    pub lifespan: f64,
}
