use specs::prelude::*;
use crate::ecs::components::transform::Transform;
use crate::ecs::components::physics::Physics;

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Transform>, ReadStorage<'a, Physics>);

    fn run(&mut self, (mut transform, physics): Self::SystemData) {
        for (transform, physics) in (&mut transform, &physics).join() {
            transform.position[0][0] += physics.velocity[0][0];
            transform.position[0][1] += physics.velocity[0][1];
            transform.position[0][2] += physics.velocity[0][2];
        }
    }
}
