use crate::components::{Position, Team, Velocity};

pub struct SpawnMissileFromShip {
    pub position: Position,
    pub velocity: Velocity,
    pub team: Team,
}


#[derive(Debug, Default)]
pub struct CreateExplosionEvent {
    pub position: Position,
    pub velocity: Velocity
}

