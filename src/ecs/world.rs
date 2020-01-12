use crate::ecs::components::collider::Collider;
use crate::ecs::components::damage::Damage;
use crate::ecs::components::despawn::Despawn;
use crate::ecs::components::camera::Camera;
use crate::ecs::components::gun::Gun;
use crate::ecs::components::health::Health;
use crate::ecs::components::physics::Physics;
use crate::ecs::components::pulsate::Pulsate;
use crate::ecs::components::shape::Shape;
use crate::ecs::components::thruster::Thruster;
use crate::ecs::components::transform::Transform;
use crate::ecs::components::transformable::Transformable;
use specs::prelude::*;

pub struct DoemWorld;
impl DoemWorld {
    pub fn new() -> World {
        let mut world = World::new();

        world.register::<Transform>();
        world.register::<Physics>();
        world.register::<Collider>();
        world.register::<Shape>();
        world.register::<Camera>();
        world.register::<Transformable>();
        world.register::<Pulsate>();
        world.register::<Health>();
        world.register::<Damage>();
        world.register::<Gun>();
        world.register::<Thruster>();
        world.register::<Despawn>();

        world
    }
}
