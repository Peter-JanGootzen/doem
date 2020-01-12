use crate::ecs::systems::camera_system::CameraSystem;
use crate::ecs::systems::collision_detector_3::CollisionDetector3;
use crate::ecs::systems::damage_system::DamageSystem;
use crate::ecs::systems::despawn_system::DespawnSystem;
use crate::ecs::systems::gl_system::GLSystem;
use crate::ecs::systems::physics_system::PhysicsSystem;
use crate::ecs::systems::pulsate_system::PulsateSystem;
use crate::ecs::systems::shoot_system::ShootSystem;
use crate::ecs::systems::thruster_system::ThrusterSystem;
use crate::ecs::systems::transformations_system::TransformationsSystem;
use luminance_glfw::GlfwSurface;
use specs::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

pub struct DoemDispatcher;

impl DoemDispatcher {
    pub fn new<'a, 'b>(surface: GlfwSurface, should_quit: Arc<Mutex<bool>>) -> Dispatcher<'a, 'b> {
        DispatcherBuilder::new()
            .with(CollisionDetector3, "collision_detector_3", &[])
            .with(PhysicsSystem, "physics_system", &[])
            .with(TransformationsSystem, "transformations_system", &[])
            .with(PulsateSystem, "pulsate_system", &[])
            .with(ShootSystem, "shoot_system", &[])
            .with(ThrusterSystem, "thruster_system", &[])
            .with(DamageSystem, "damage_system", &["collision_detector_3"])
            .with(DespawnSystem, "despawn_system", &[])
            .with(CameraSystem, "camera_system", &[])
            .with_thread_local(GLSystem::new(surface, should_quit))
            .build()
    }
}
