use crate::ecs::systems::gl_system::GLSystem;
use crate::ecs::systems::physics_system::PhysicsSystem;
use crate::ecs::systems::transformations_system::TransformationsSystem;
use crate::ecs::systems::pulsate_system::PulsateSystem;
use crate::ecs::systems::shoot_system::ShootSystem;
use luminance_glfw::GlfwSurface;
use specs::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

pub struct DoemDispatcher;

impl DoemDispatcher {
    pub fn new<'a, 'b>(surface: GlfwSurface, should_quit: Arc<Mutex<bool>>) -> Dispatcher<'a, 'b> {
        DispatcherBuilder::new()
            .with(PhysicsSystem, "physics_system", &[])
            .with(TransformationsSystem, "transformations_system", &[])
            .with(PulsateSystem, "pulsate_system", &[])
            .with(ShootSystem, "shoot_system", &[])
            .with_thread_local(GLSystem::new(surface, should_quit))
            .build()
    }
}
