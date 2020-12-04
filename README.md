# tactician

Orbital battle simulator


## Design

### Structs

PhysicsDetails
 * position
 * velocity
 * forces

Missile
* PhysicsDetails
* det radius
* lifetime
* when the missile was created

Ship
* PhysicsDetails
* deltav
* method to decide to accelerate/decelerate

### Traits

ForceSource
 * apply force to physicsdetails

