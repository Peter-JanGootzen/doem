use crate::ecs::components::collider::Collider;
use crate::ecs::components::follow_camera::FollowCamera;
use crate::ecs::components::physics::Physics;
use crate::ecs::components::shape::Shape;
use crate::ecs::components::transform::Transform;
use crate::ecs::components::transformable::Transformable;
use crate::ecs::components::pulsate::Pulsate;
use crate::ecs::components::damage::Damage;
use crate::ecs::components::health::Health;
use crate::ecs::components::gun::Gun;
use specs::prelude::*;

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
        world.register::<Pulsate>();
        world.register::<Health>();
        world.register::<Damage>();
        world.register::<Gun>();

        world
    }
}
