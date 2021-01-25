use crate::physics;
use crate::physics::ForceSource;
use crate::utils;
use nalgebra::Vector2;

pub struct Missile {
    pub phys: physics::PhysicsDetails,
    pub det_radius: f64,
    pub lifetime: f64,
    pub created_on: i64,
}

pub struct Ship {
    pub phys: physics::PhysicsDetails,
    pub max_accel: f64,
    pub current_accel: f64,
}

impl physics::ForceSource for Ship {
    fn calculate_force_applied_to_object(
        &self,
        _details: &physics::PhysicsDetails,
    ) -> Vector2<f64> {
        // keep n
        let accel_magnitude = utils::bound(-self.max_accel, self.current_accel, self.max_accel);
        let accel_direction = self.phys.velocity.normalize();
        let accel_vec = accel_direction * accel_magnitude;
        return accel_vec * self.phys.mass;
    }
}

pub struct CelestialObject {
    pub phys: physics::PhysicsDetails,
    pub radius: f64,
}

impl physics::ForceSource for CelestialObject {
    fn calculate_force_applied_to_object(
        &self,
        details: &physics::PhysicsDetails,
    ) -> Vector2<f64> {
        let delta_pos = details.pos - &self.phys.pos;
        let dist2 = delta_pos.dot(&delta_pos);
        let force_magnitude = physics::G * details.mass * self.phys.mass / dist2;
        let force_direction = -delta_pos;
        return force_direction.normalize() * force_magnitude;
    }
}

pub struct Simulator {
    pub sun: CelestialObject,
    pub planets: Vec<CelestialObject>,
    pub ships: Vec<Ship>,
    // pub missiles: Vec<Missile>,
}
impl Simulator {
    /// the interval is in seconds, or whatever the denominator unit of velocity is
    pub fn update(&mut self, interval: f64) {
        // Step 1: Update ships based on planets, sun
        for ship in self.ships.iter_mut() {
            let mut aggregate_force = self
                .planets
                .iter()
                .map(|planet| planet.calculate_force_applied_to_object(&ship.phys))
                .fold(Vector2::new(0.0, 0.0), |acc, force| force + acc);
            aggregate_force += self.sun.calculate_force_applied_to_object(&ship.phys);

            let vel_change = aggregate_force / ship.phys.mass * interval;

            // To have good numerical integration, we multiply the velocity change in half
            // to account for the fact that, at the beginning of this time interval, the
            // velocity change was 0. This assumes that the velocity increased linearly over time
            // which is a better estimate than assuming that it discontinuously jumps around
            ship.phys.velocity += vel_change * 0.5;
            ship.phys.pos += ship.phys.velocity * interval;
        }

        // Step 2: Update planets based on each other + sun
        for this_planet_id in 0..self.planets.len() {
            let this_planet = self.planets.get(this_planet_id).unwrap();

            // sum of force applied by fellow planets + sun
            let mut aggregate_force = self
                .planets
                .iter()
                .enumerate()
                .map(|(i, other_planet)| {
                    if i != this_planet_id {
                        return other_planet.calculate_force_applied_to_object(&this_planet.phys);
                    } 
                    return Vector2::new(0.0, 0.0);
                })
                .fold(Vector2::new(0.0, 0.0), |acc, force| force + acc);
            aggregate_force += self.sun.calculate_force_applied_to_object(&this_planet.phys);


            let vel_change = aggregate_force / this_planet.phys.mass * interval;
            let this_planet = self.planets.get_mut(this_planet_id).unwrap();
            this_planet.phys.velocity += vel_change * 0.5;
            this_planet.phys.pos += this_planet.phys.velocity * interval;
        }
    }
}