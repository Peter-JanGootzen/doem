use specs::prelude::*;
use crate::ecs::components::physics::Physics;
use crate::ecs::components::transform::Transform;
use crate::ecs::components::transformable::Transformable;
use crate::ecs::components::shape::Shape;
use crate::ecs::components::collider::Collider;
use crate::ecs::components::follow_camera::FollowCamera;

pub struct DoemWorld;
impl DoemWorld {
    pub fn new() -> World {
        let mut world = World::new();

        world.register::<Transform>();
        world.register::<Physics>();
        world.register::<Collider>();
        world.register::<Shape>();
        world.register::<FollowCamera>();
        world.register::<Transformable>();

        world
    }
}
