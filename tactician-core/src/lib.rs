pub use nalgebra;

pub mod physics {
    use nalgebra::Vector2;
    // TODO: adjust gravitational constant to acheive desired behavior
    const G: f64 = 0.00000000006;

    pub struct PhysicsDetails {
        pub pos: Vector2<f64>,
        pub mass: f64,
        pub velocity: Vector2<f64>,
    }
    impl PhysicsDetails {
        pub fn new(mass: f64) -> PhysicsDetails {
            return PhysicsDetails {
                pos: Vector2::new(0.0, 0.0),
                mass: mass,
                velocity: Vector2::new(0.0, 0.0),
            };
        }
    }

    pub trait ForceSource {
        fn calculate_force_applied_to_object(&self, details: &PhysicsDetails) -> Vector2<f64>;
    }

    pub trait GravitationalForceSource: ForceSource {
        fn grav_const(&self) -> f64 {
            return G;
        }

        fn physics_details(&self) -> &PhysicsDetails;

        fn calculate_force_applied_to_object(&self, details: &PhysicsDetails) -> Vector2<f64> {
            let delta_pos = details.pos - &self.physics_details().pos;
            let dist2 = delta_pos.dot(&delta_pos);
            let force_magnitude =
                self.grav_const() * details.mass * self.physics_details().mass / dist2;
            let force_direction = -delta_pos;
            return force_direction.normalize() * force_magnitude;
        }
    }

    impl<T: GravitationalForceSource> ForceSource for T {
        fn calculate_force_applied_to_object(&self, details: &PhysicsDetails) -> Vector2<f64> {
            return GravitationalForceSource::calculate_force_applied_to_object(self, details);
        }
    }
}

pub mod utils {
    pub fn bound(lower_bound: f64, num_to_bound: f64, upper_bound: f64) -> f64 {
        return lower_bound.max(upper_bound.min(num_to_bound));
    }
}

pub mod objects {
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

    impl physics::GravitationalForceSource for CelestialObject {
        fn physics_details(&self) -> &physics::PhysicsDetails {
            return &self.phys;
        }
    }

    pub struct Simulator {
        pub sun: CelestialObject,
        pub ship: Ship,
    }
    impl Simulator {
        /// the interval is in seconds, or whatever the denominator unit of velocity is
        pub fn update(&mut self, interval: f64) {
            let force = self.sun.calculate_force_applied_to_object(&self.ship.phys);
            let vel_change = force / self.ship.phys.mass * interval;
            self.ship.phys.velocity += vel_change * 0.5;
            self.ship.phys.pos += self.ship.phys.velocity * interval;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::{CelestialObject, Ship, Simulator};
    use crate::physics::PhysicsDetails;
    use nalgebra::Vector2;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn planet_gravity() {
        let planet = CelestialObject {
            phys: PhysicsDetails::new(1000000.0), // planet weighs 1mil kilos
            radius: 20.0,
        };

        let ship = Ship {
            phys: PhysicsDetails {
                pos: Vector2::new(0.0, 10.0),
                mass: 30.0,
                velocity: Vector2::new(-5.0, 0.0)
            }, // ship weighs 30 kilos
            current_accel: 0.0,
            max_accel: 3.0,
        };

        let simulator = Simulator {
            sun: planet,
            ship: ship
        };
    }
}
