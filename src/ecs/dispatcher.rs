use specs::prelude::*;
use crate::ecs::systems::physics_system::PhysicsSystem;
use crate::ecs::systems::rendering_system::RenderingSystem;

pub struct DoemDispatcher;

impl DoemDispatcher {
    pub fn new<'a, 'b>() -> Dispatcher<'a, 'b> {
        DispatcherBuilder::new()
            .with(PhysicsSystem, "physics_system", &[])
            .with_thread_local(RenderingSystem)
            .build()
    }
}
