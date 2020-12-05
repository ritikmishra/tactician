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
                velocity: Vector2::new(0.0, 0.0)
            };
        }
    }

    pub trait ForceSource {
        fn calculate_force_applied_to_object(&self, detalis: &PhysicsDetails) -> Vector2<f64>;
    }

    pub trait GravitationalForceSource: ForceSource {
        fn grav_const(&self) -> f64 {
            return G;
        }

        fn physics_details(&self) -> &PhysicsDetails;

        fn calculate_force_applied_to_object(&self, details: &PhysicsDetails) -> Vector2<f64> {
            let dist2 = details.pos.dot(&self.physics_details().pos);
            let force_magnitude = self.grav_const() * details.mass * self.physics_details().mass / dist2;
            let force_direction = details.pos - self.physics_details().pos;
            return force_direction.normalize() * force_magnitude;
        }
    }

    impl<T: GravitationalForceSource> ForceSource for T {
        fn calculate_force_applied_to_object(&self, detalis: &PhysicsDetails) -> Vector2<f64> {
            return GravitationalForceSource::calculate_force_applied_to_object(self, detalis);
        }
    }
}

pub mod objects {
    use crate::physics::GravitationalForceSource;
use crate::physics::PhysicsDetails;

    pub struct Missile {
        pub phys: PhysicsDetails,
        pub det_radius: f64,
        pub lifetime: f64,
        pub created_on: i64,
    }

    pub struct Ship {
        pub phys: PhysicsDetails,
        pub max_decel: f64,
    }

    pub struct CelestialObject {
        pub phys: PhysicsDetails,
        pub radius: f64,
    }

    
    impl GravitationalForceSource for CelestialObject {
        fn physics_details(&self) -> &PhysicsDetails {
            return &self.phys;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::physics::PhysicsDetails;
use crate::objects::CelestialObject;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn planet_gravity() {
        let planet = CelestialObject {
            phys: PhysicsDetails::new(3.0),
            radius: 20.0
        };
    }
}
