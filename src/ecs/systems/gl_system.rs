use crate::ecs::components::follow_camera::FollowCamera;
use crate::ecs::components::shape::Shape;
use crate::ecs::components::transform::Transform;
use crate::ecs::resources::doem_events::DoemEvents;
use crate::gl_common::{ShaderInterface, VertexSemantics};
use crate::tess_manager::TessManager;
use crate::obj_loader::ObjLoader;
use doem_math::vector_space::{Matrix4, Vector3, PI};
use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::TessSlice;
use luminance::texture::{Dim2, Flat};
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowEvent};
use specs::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

const VS: &str = include_str!("../../shaders/displacement-vs.glsl");
const FS: &str = include_str!("../../shaders/displacement-fs.glsl");

const FOVY: f32 = PI / 2.0;
const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = std::f32::MAX / 2.0 - 1.0;

pub struct GLSystem {
    surface: Rc<RefCell<GlfwSurface>>,
    back_buffer: Framebuffer<Flat, Dim2, (), ()>,
    tess_manager: TessManager,
    shader_program: Program<VertexSemantics, (), ShaderInterface>,
    should_quit: Arc<Mutex<bool>>,
    draw_bounding_boxes: bool
}

impl GLSystem {
    pub fn new(mut surface: GlfwSurface, should_quit: Arc<Mutex<bool>>) -> Self {
        let back_buffer = surface.back_buffer().unwrap();
        let shader_program =
            Program::<VertexSemantics, (), ShaderInterface>::from_strings(None, VS, None, FS)
                .expect("Shaders could not be initialized, bye :(")
                .ignore_warnings();
        let surface = Rc::new(RefCell::new(surface));
        let tess_manager = TessManager::new(surface.clone());
        let draw_bounding_boxes = false;
        Self {
            surface,
            back_buffer,
            tess_manager,
            shader_program,
            should_quit,
            draw_bounding_boxes
        }
    }
}

impl<'a> System<'a> for GLSystem {
    type SystemData = (
        Write<'a, DoemEvents>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Shape>,
        ReadStorage<'a, FollowCamera>,
    );

    fn run(&mut self, (mut events, transform, mut shape, camera): Self::SystemData) {
        let projection = Matrix4::get_projection(
            FOVY,
            self.surface.borrow().width() as f32 / self.surface.borrow().height() as f32,
            Z_NEAR,
            Z_FAR,
        );
        let mut view: Option<Matrix4> = None;
        for (t, c) in (&transform, &camera).join() {
            let camera_at_origin = &c.offset * c.zoom_level;
            let camera_at_origin_rotated = &t.orientation * &camera_at_origin.dimension_hop();
            let eye = &t.position + &camera_at_origin_rotated.dimension_hop();
            let look_at = &t.position;
            let up = &t.orientation * &Vector3::new_from_array([[0.0], [1.0], [0.0]]).dimension_hop();
            view = Some(Matrix4::get_view(&eye, look_at, &up.dimension_hop()));
        }
        let view = view.expect("No View was found!");

        for s in (&mut shape).join() {
            match s {
                Shape::Unit { .. } => {
                    *s = self.tess_manager.init_shape((*s).clone());
                }
                Shape::Init {
                    bounding_box,
                    bounding_box_tess_id,
                    ..
                } => {
                    if self.draw_bounding_boxes {
                        match bounding_box_tess_id {
                            None => {
                                let id = self.tess_manager.get_aabb_id(&bounding_box);
                                *bounding_box_tess_id = Some(id);
                            }
                            Some(id) => {
                                let tess = self.tess_manager.get_tess(*id);
                                if tess.is_none() {
                                    let id = self.tess_manager.get_aabb_id(&bounding_box);
                                    *bounding_box_tess_id = Some(id);
                                }
                            }
                            _ => ()
                        }
                    }
                }
            }
        }

        let shader_program = &self.shader_program;
        let view = view;
        let tess_manager = &mut self.tess_manager;
        let draw_bounding_boxes = &self.draw_bounding_boxes;
        self.surface.borrow_mut().pipeline_builder().pipeline(
            &self.back_buffer,
            &PipelineState::default(),
            |_, mut shd_gate| {
                shd_gate.shade(shader_program, |iface, mut rdr_gate| {
                    iface
                        .projection
                        .update(projection.transpose().copy_to_array());
                    iface.view.update(view.transpose().copy_to_array());

                    rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                        // Render all the tesselations with their transformations
                        iface
                            .transform
                            .update(Matrix4::identity().transpose().copy_to_array());
                        for (s, t) in (&shape, &transform).join() {
                            if let Shape::Init {
                                tess_id,
                                bounding_box_tess_id,
                                ..
                            } = s
                            {
                                let translation = Matrix4::get_translation(&t.position);
                                let scaling = Matrix4::get_scaling(&t.scale);

                                let transform = &translation * &(&t.orientation * &scaling);
                                iface
                                    .transform
                                    .update(transform.transpose().copy_to_array());

                                let tess_ref = tess_manager.get_tess(*tess_id).unwrap();
                                tess_gate.render(TessSlice::one_whole(tess_ref));
                                if *draw_bounding_boxes {
                                    if let Some(id) = bounding_box_tess_id {
                                        let bounding_box_tess_ref = tess_manager.get_tess(*id).unwrap();
                                        tess_gate.render(TessSlice::one_whole(bounding_box_tess_ref));
                                    } else {
                                        println!("ERROR, we are drawing bounding boxes, but a shape doesn't have a bounding_box_tess_id");
                                    }
                                }
                            }
                        }
                    });
                });
            },
        );

        tess_manager.end();
        self.surface.borrow_mut().swap_buffers();

        let mut resize = false;

        events.0.clear();
        for event in self.surface.borrow_mut().poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    *(*self.should_quit).lock().unwrap() = true;
                }
                WindowEvent::FramebufferSize(..) => {
                    resize = true;
                }
                WindowEvent::Key(Key::B, _, action, _)
                if action == Action::Press || action == Action::Repeat => {
                    self.draw_bounding_boxes = !self.draw_bounding_boxes;
                }
                e => {
                    (events.0).push(e);
                }
            }
        }

        if resize {
            self.back_buffer = self.surface.borrow_mut().back_buffer().unwrap();
        }
    }
    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        world.write_resource::<DoemEvents>();
    }
}
