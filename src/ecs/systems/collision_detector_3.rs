use crate::ecs::components::transform::Transform;
use crate::ecs::components::shape::Shape;
use crate::ecs::components::collider::Collider;
use crate::ecs::resources::collisions::Collisions;
use crate::data::AABB;
use doem_math::vector_space::Matrix4;
use specs::prelude::*;

pub struct CollisionDetector3;

impl<'a> System<'a> for CollisionDetector3 { 
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Shape>,
        ReadStorage<'a, Collider>,
        ReadStorage<'a, Transform>,
        Write<'a, Collisions>
    );

    fn run(&mut self, (entities, shape, collider, transform, mut collisions): Self::SystemData) {
        collisions.0.clear();
        for(ent1, s1, _c1, t1) in (&*entities, &shape, &collider, &transform).join() {
            for(ent2, s2, _c2, t2) in (&*entities, &shape, &collider, &transform).join() { 
                if ent1.id() == ent2.id() {
                    continue;
                }
                let bb1: &AABB; 
                let bb2: &AABB;
                if let Shape::Init { bounding_box, .. } = s1 {
                    bb1 = bounding_box;
                    if let Shape::Init { bounding_box, .. } = s2 {
                        bb2 = bounding_box;
                        if Self::square_overlaps(bb1, t1, bb2, t2) {
                            collisions.0.insert((ent1.id(), ent2.id()));
                        }
                    }
                }
            }
        }
    }
    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        world.write_resource::<Collisions>();
    }
}

impl CollisionDetector3 {
    fn square_overlaps(box1: &AABB, t1: &Transform, box2: &AABB, t2: &Transform) -> bool {
        let box1_scaling = Matrix4::get_scaling(&t1.scale);
        let box2_scaling = Matrix4::get_scaling(&t2.scale);
        let box1_pos = &(&box1_scaling * &box1.middle_point.dimension_hop()) + &t1.position.dimension_hop();
        let box2_pos = &(&box2_scaling * &box2.middle_point.dimension_hop()) + &t2.position.dimension_hop();
        let box1_scaled_half_size = &box1_scaling * &box1.half_size.dimension_hop();
        let box2_scaled_half_size = &box2_scaling * &box2.half_size.dimension_hop();


        let min_x1 = box1_pos[0][0] - box1_scaled_half_size[0][0];
        let max_x1 = box1_pos[0][0] + box1_scaled_half_size[0][0];
        let min_x2 = box2_pos[0][0] - box2_scaled_half_size[0][0];
        let max_x2 = box2_pos[0][0] + box2_scaled_half_size[0][0];

        let min_y1 = box1_pos[1][0] - box1_scaled_half_size[1][0];
        let max_y1 = box1_pos[1][0] + box1_scaled_half_size[1][0];
        let min_y2 = box2_pos[1][0] - box2_scaled_half_size[1][0];
        let max_y2 = box2_pos[1][0] + box2_scaled_half_size[1][0];

        let min_z1 = box1_pos[2][0] - box1_scaled_half_size[2][0];
        let max_z1 = box1_pos[2][0] + box1_scaled_half_size[2][0];
        let min_z2 = box2_pos[2][0] - box2_scaled_half_size[2][0];
        let max_z2 = box2_pos[2][0] + box2_scaled_half_size[2][0];
 
        (min_x1 <= max_x2 && max_x1 >= min_x2) &&
            (min_y1 <= max_y2 && max_y1 >= min_y2) &&
                (min_z1 <= max_z2 && max_z1 >= min_z2)
    }
}
