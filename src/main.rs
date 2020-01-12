mod consts;
mod data;
mod ecs;
mod gl_common;
mod obj_loader;
mod tess_manager;

#[macro_use]
extern crate lazy_static;

use crate::ecs::components::collider::Collider;
use crate::ecs::components::camera::Camera;
use crate::ecs::components::gun::Gun;
use crate::ecs::components::health::Health;
use crate::ecs::components::physics::Physics;
use crate::ecs::components::pulsate::Pulsate;
use crate::ecs::components::shape::Shape;
use crate::ecs::components::thruster::Thruster;
use crate::ecs::components::transform::Transform;
use crate::ecs::components::transformable::Transformable;
use crate::ecs::dispatcher::DoemDispatcher;
use crate::ecs::world::DoemWorld;
use clap::App;
use doem_math::vector_space::{Matrix4, Vector3};
use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowOpt};
use specs::prelude::*;
use specs::WorldExt;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    App::new("Doem")
        .version("1.0")
        .author("Bram-Boris Meerlo and Peter-Jan Gootzen")
        .about("Made using our own linear algebra crate doem-math.");

    start();
}

fn start() {
    let surface = GlfwSurface::new(WindowDim::Windowed(1600, 900), "Doem", WindowOpt::default())
        .expect("GLFW surface creation");

    let should_quit = Arc::new(Mutex::new(false));
    let mut world = DoemWorld::new();

    world
        .create_entity()
        .with(Shape::Unit {
            obj_path: consts::STARSHIP_OBJ_PATH.to_owned(),
        })
        .with(Transform {
            position: Vector3::new_from_array([[0.0], [30.0], [0.0]]),
            scale: Vector3::new_from_array([[10.0], [10.0], [10.0]]),
            orientation: Matrix4::identity(),
        })
        .with(Physics {
            velocity: Vector3::new_from_array([[0.00], [0.0], [0.0]]),
        })
        .with(Thruster {
            power: Vector3::new_from_array([[1.00], [0.0], [0.0]]),
        })
        .with(Transformable)
        .with(Camera {
            zoom_level: 10.0,
            offset: Vector3::new_from_array([[20.0], [0.0], [0.0]]),
            orientation: Matrix4::identity()
        })
        .with(Gun {
            damage: consts::STARSHIP_BULLET_DAMAGE,
            velocity: consts::STARSHIP_BULLET_VELOCITY.clone(),
            despawn_bullet_on_impact: true,
        })
        .build();

    world
        .create_entity()
        .with(Shape::Unit {
            obj_path: consts::REFERENCEPLANE_OBJ_PATH.to_owned(),
        })
        .with(Transform {
            position: Vector3::new_from_array([[1.0], [-3.0], [0.0]]),
            scale: Vector3::new_from_array([[10.0], [1.0], [10.0]]),
            orientation: Matrix4::identity(),
        })
        .build();

    world
        .create_entity()
        .with(Shape::Unit {
            obj_path: consts::NONDESCRIPTCIRCLE_OBJ_PATH.to_owned(),
        })
        .with(Transform {
            position: Vector3::new_from_array([[0.0], [700.0], [-500.0]]),
            scale: consts::NONDESCRIPTCIRCLE_SCALE.clone(),
            orientation: Matrix4::identity(),
        })
        .with(Pulsate {
            speed: consts::NONDESCRIPTCIRCLE_SPEED.clone(),
            current_direction: true,
            min_scale: consts::NONDESCRIPTCIRCLE_MIN_SCALE.clone(),
            max_scale: consts::NONDESCRIPTCIRCLE_MAX_SCALE.clone(),
        })
        .with(Collider {
            half_size: Vector3::new_from_array([[1.0], [1.0], [1.0]]),
        })
        .with(Health { health: 100.0 })
        .build();

    let mut dispatcher = DoemDispatcher::new(surface, should_quit.clone());
    dispatcher.setup(&mut world);
    'game_loop: loop {
        dispatcher.dispatch(&world);
        world.maintain();
        if *(*should_quit).lock().unwrap() {
            break 'game_loop;
        }
    }
    dispatcher.dispose(&mut world);
}
