use crate::components::*;
use bevy::prelude::Bundle;

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
}

#[derive(Bundle, Default)]
pub struct ShipBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub mass: Mass,

    pub size: Size,
    pub engine: EnginePhysics,

    pub ship: Ship,
}
