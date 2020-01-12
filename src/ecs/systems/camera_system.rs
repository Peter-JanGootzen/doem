use crate::ecs::components::camera::Camera;
use crate::ecs::components::shape::Shape;
use crate::ecs::resources::doem_events::DoemEvents;
use doem_math::{Matrix4, Vector4};
use luminance_glfw::{Action, Key, WindowEvent};
use specs::prelude::*;

pub struct CameraSystem;

const ZOOM_LEVEL_SENSITIVITY: f32 = 2.0;
const ROTATE_SENSITIVITY: f32 = 0.05;
const MOVE_SENSITIVITY: f32 = 2.0;

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        Read<'a, DoemEvents>,
        WriteStorage<'a, Camera>,
        ReadStorage<'a, Shape>,
    );

    fn run(&mut self, (events, mut camera, shape): Self::SystemData) {
        for e in &events.0 {
            for (c, s) in (&mut camera, &shape).join() {
                match e {
                    WindowEvent::Key(Key::PageUp, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        c.zoom_level -= ZOOM_LEVEL_SENSITIVITY;
                    }
                    WindowEvent::Key(Key::PageDown, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        c.zoom_level += ZOOM_LEVEL_SENSITIVITY;
                    }
                    WindowEvent::Key(Key::H, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        c.offset[2][0] += MOVE_SENSITIVITY;
                    }
                    WindowEvent::Key(Key::J, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        c.offset[1][0] -= MOVE_SENSITIVITY;
                    }
                    WindowEvent::Key(Key::K, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        c.offset[1][0] += MOVE_SENSITIVITY;
                    }
                    WindowEvent::Key(Key::L, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        c.offset[2][0] -= MOVE_SENSITIVITY;
                    }
                    WindowEvent::Key(Key::Left, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        if let Shape::Init { bounding_box, .. } = s {
                            let y = Vector4::from([
                                [0.0],
                                [bounding_box.middle_point[1][0] + bounding_box.half_size[1][0]],
                                [0.0],
                                [0.0],
                            ]);
                            let y_norm = y.normalize();
                            c.offset = (&Matrix4::get_rotation(&y_norm, -ROTATE_SENSITIVITY)
                                * &c.offset.dimension_hop())
                                .dimension_hop();
                        }
                    }
                    WindowEvent::Key(Key::Right, _, action, _)
                        if *action == Action::Press || *action == Action::Repeat =>
                    {
                        if let Shape::Init { bounding_box, .. } = s {
                            let y = Vector4::from([
                                [0.0],
                                [bounding_box.middle_point[2][0] + bounding_box.half_size[2][0]],
                                [0.0],
                                [0.0],
                            ]);
                            let y_norm = y.normalize();
                            c.offset = (&Matrix4::get_rotation(&y_norm, ROTATE_SENSITIVITY)
                                * &c.offset.dimension_hop())
                                .dimension_hop();
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
