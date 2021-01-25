pub use nalgebra;

pub mod physics {
    use nalgebra::Vector2;
    // TODO: adjust gravitational constant to acheive desired behavior
    pub const G: f64 = 0.0000000006;

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
}

pub mod utils {
    pub fn bound(lower_bound: f64, num_to_bound: f64, upper_bound: f64) -> f64 {
        return lower_bound.max(upper_bound.min(num_to_bound));
    }
}

pub mod objects;

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
        let sun = CelestialObject {
            phys: PhysicsDetails::new(1000000.0), // sun weighs 1mil kilos
            radius: 20.0,
        };

        let ship = Ship {
            phys: PhysicsDetails {
                pos: Vector2::new(0.0, 10.0),
                mass: 30.0,
                velocity: Vector2::new(-5.0, 0.0),
            }, // ship weighs 30 kilos
            current_accel: 0.0,
            max_accel: 3.0,
        };

        let simulator = Simulator {
            sun: sun,
            planets: vec![],
            ships: vec![ship],
        };
    }
}
