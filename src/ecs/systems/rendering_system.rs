use specs::prelude::*;
use crate::ecs::components::transform::Transform;
use crate::ecs::components::shape::Shape;

pub struct RenderingSystem;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (ReadStorage<'a, Transform>, ReadStorage<'a, Shape>);

    fn run(&mut self, (mut transform, shape): Self::SystemData) {
    }
}
