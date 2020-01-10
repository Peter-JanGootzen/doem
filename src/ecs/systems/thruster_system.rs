use doem_math::vector_space::{ Vector3, Vector4, Matrix4 };
use crate::ecs::resources::doem_events::DoemEvents;
use crate::ecs::components::physics::Physics;
use crate::ecs::components::thruster::Thruster;
use specs::prelude::*;
use luminance_glfw::{Action, Key, WindowEvent};

pub struct ThrusterSystem;

impl<'a> System<'a> for ThrusterSystem {
    type SystemData = (
        Read<'a, DoemEvents>,
        WriteStorage<'a, Physics>,
        ReadStorage<'a, Thruster>,
    );

    fn run(&mut self , (events, mut physics, thruster): Self::SystemData) {
        for e in &events.0 {
            match e {
                WindowEvent::Key(Key::LeftShift, _, action, _)
                if *action == Action::Press || *action == Action::Repeat => {
                    for (p, t) in (&mut physics, &thruster).join() {
                        p.velocity = &p.velocity + &t.power;
                    }
                }
                WindowEvent::Key(Key::LeftControl, _, action, _)
                if *action == Action::Press || *action == Action::Repeat => {
                    for (p, t) in (&mut physics, &thruster).join() {
                        p.velocity = &p.velocity - &t.power;
                    }
                }
                WindowEvent::Key(Key::N, _, action, _)
                if *action == Action::Press || *action == Action::Repeat => {
                    for (p, t) in (&mut physics, &thruster).join() {
                        p.velocity = Vector3::new_from_array([[0.0],[0.0],[0.0]]);
                    }
                }
                _ => ()
            }
        }
    }
}
