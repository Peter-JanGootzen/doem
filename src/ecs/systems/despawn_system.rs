use crate::consts;
use crate::ecs::components::despawn::Despawn;
use crate::ecs::components::shape::Shape;
use crate::ecs::components::transform::Transform;
use crate::ecs::systems::collision_detector_3::CollisionDetector3;
use doem_math::{Matrix4, Vector3};
use specs::prelude::*;

pub struct DespawnSystem;

impl<'a> System<'a> for DespawnSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Despawn>,
        ReadStorage<'a, Shape>,
    );

    fn run(&mut self, (entities, transform, despawn, shape): Self::SystemData) {
        let world_bounding_box = consts::WORLD_BOUNDING_BOX.clone();
        let world_transform = Transform {
            position: Vector3::origin(),
            scale: Vector3::from([[1.0], [1.0], [1.0]]),
            orientation: Matrix4::identity(),
        };
        let mut to_kill: Vec<Entity> = Vec::new();
        for (ent, t, _d, s) in (&*entities, &transform, &despawn, &shape).join() {
            // Sadly you must have a shape right now
            if let Shape::Init { bounding_box, .. } = s {
                if !CollisionDetector3::intersects(
                    &world_bounding_box,
                    &world_transform,
                    &bounding_box,
                    &t,
                ) {
                    to_kill.push(ent);
                }
            }
        }
        for e in &to_kill {
            if let Err(..) = entities.delete(*e) {
                println!(
                    "Tried to delete and entity(id: {}) in DespawnSystem, but this sadly failed",
                    e.id()
                );
            }
        }
    }
}
