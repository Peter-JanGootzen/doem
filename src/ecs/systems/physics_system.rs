use crate::ecs::components::physics::Physics;
use crate::ecs::components::transform::Transform;
use specs::prelude::*;

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Transform>, ReadStorage<'a, Physics>);

    fn run(&mut self, (mut transform, physics): Self::SystemData) {
        for (transform, physics) in (&mut transform, &physics).join() {
            transform.position[0][0] += physics.velocity[0][0];
            transform.position[1][0] += physics.velocity[1][0];
            transform.position[2][0] += physics.velocity[2][0];
        }
    }
}
