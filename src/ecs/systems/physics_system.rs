use crate::ecs::components::physics::Physics;
use crate::ecs::components::transform::Transform;
use doem_math::Vector4;
use specs::prelude::*;

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Transform>, ReadStorage<'a, Physics>);

    fn run(&mut self, (mut transform, physics): Self::SystemData) {
        for (t, p) in (&mut transform, &physics).join() {
            {
                let u = Vector4::from([[-1.0], [0.0], [0.0], [1.0]]);
                let u_norm = u.normalize();
                let u_rotated = &t.orientation * &u_norm;
                let u_sped_up = &u_rotated * p.velocity[0][0];
                t.position = &t.position + &u_sped_up.dimension_hop();
            }
            {
                let u = Vector4::from([[0.0], [1.0], [0.0], [1.0]]);
                let u_norm = u.normalize();
                let u_rotated = &t.orientation * &u_norm;
                let u_sped_up = &u_rotated * p.velocity[1][0];
                t.position = &t.position + &u_sped_up.dimension_hop();
            }
            {
                let u = Vector4::from([[0.0], [0.0], [1.0], [1.0]]);
                let u_norm = u.normalize();
                let u_rotated = &t.orientation * &u_norm;
                let u_sped_up = &u_rotated * p.velocity[2][0];
                t.position = &t.position + &u_sped_up.dimension_hop();
            }
        }
    }
}
