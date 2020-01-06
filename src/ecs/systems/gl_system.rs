use specs::prelude::*;
use doem_math::vector_space::{ PI, Matrix4 };
use std::sync::Arc;
use std::rc::Rc;
use std::sync::Mutex;
use std::cell::RefCell;
use cgmath;
use cgmath::EuclideanSpace;
use luminance_glfw::{Action, Key, GlfwSurface, Surface, WindowEvent};
use luminance::shader::program::Program;
use luminance::texture::{Dim2,Flat};
use luminance::framebuffer::Framebuffer;
use luminance::context::GraphicsContext;
use luminance::render_state::RenderState;
use luminance::tess::TessSlice;
use crate::gl_common::{VertexSemantics, ShaderInterface};
use crate::ecs::components::transform::Transform;
use crate::ecs::components::shape::Shape;
use crate::tess_manager::TessManager;

const VS: &str = include_str!("../../shaders/displacement-vs.glsl");
const FS: &str = include_str!("../../shaders/displacement-fs.glsl");

const FOVY: cgmath::Rad<f32> = cgmath::Rad(PI / 2.0);
const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 1000.;

pub struct DoemEvents(Vec<WindowEvent>);

impl Default for DoemEvents {
    fn default() -> DoemEvents {
        DoemEvents(Vec::<WindowEvent>::new())
    }
}

pub struct GLSystem {
    surface: Rc<RefCell<GlfwSurface>>,
    back_buffer: Framebuffer<Flat, Dim2, (), ()>,
    tess_manager: TessManager,
    shader_program: Program::<VertexSemantics, (), ShaderInterface>,
    projection: cgmath::Matrix4<f32>,
    view: cgmath::Matrix4<f32>,
    should_quit: Arc<Mutex<bool>>
}

impl GLSystem {
    pub fn new(mut surface: GlfwSurface, should_quit: Arc<Mutex<bool>>) -> Self {
        let back_buffer = surface.back_buffer().unwrap();
        let shader_program = Program::<VertexSemantics, (), ShaderInterface>::from_strings(None, VS, None, FS)
            .expect("Shaders could not be initialized, bye :(")
            .ignore_warnings();
        let projection = cgmath::perspective(FOVY, surface.width() as f32 / surface.height() as f32, Z_NEAR, Z_FAR);
        let view = cgmath::Matrix4::<f32>::look_at(cgmath::Point3::new(0.0, 0.0, 100.0), cgmath::Point3::origin(), cgmath::Vector3::unit_y());
        let surface = Rc::new(RefCell::new(surface));
        let tess_manager = TessManager::new(surface.clone());
        Self {
            surface,
            back_buffer,
            tess_manager,
            shader_program,
            projection,
            view,
            should_quit
        }
    }
}

impl<'a> System<'a> for GLSystem {
    type SystemData = (Write<'a, DoemEvents>,
                       WriteStorage<'a, Transform>,
                       WriteStorage<'a, Shape>);

    fn run(&mut self, (mut events, transform, mut shape): Self::SystemData) {
        for s in (&mut shape).join() {
            if let Shape::Unit { .. } = s {
                *s = self.tess_manager.init_shape((*s).clone());
            }
        }

        let shader_program = &self.shader_program;
        let projection = &self.projection;
        let view = &self.view;
        let tess_manager = &mut self.tess_manager;
        self.surface.borrow_mut()
            .pipeline_builder()
            .pipeline(&self.back_buffer, [0., 0., 0., 0.], |_, mut shd_gate| {
                shd_gate.shade(shader_program, |iface, mut rdr_gate| {
                    iface.projection.update((*projection).into());
                    iface.view.update((*view).into());

                    rdr_gate.render(RenderState::default(), |mut tess_gate| {
                        // Render all the tesselations with their transformations
                        iface.transform.update(Matrix4::identity().transpose().copy_to_array());
                        for (s, t) in (&shape, &transform).join() {
                            if let Shape::Init { tess_id, bounding_box: _, bounding_box_tess_id } = s {
                                let translation = Matrix4::get_translation(&t.position);
                                let scaling = Matrix4::get_scaling(&t.scale);
                                let orientation = &t.orientation;

                                let transform = &translation * &(orientation * &scaling);
                                println!("{}", &transform);
                                iface.transform.update(transform.transpose().copy_to_array());

                                let tess_ref = tess_manager.get_tess(*tess_id).unwrap();
                                tess_gate.render(TessSlice::one_whole(tess_ref));
                                let bounding_box_tess_ref = tess_manager.get_tess(*bounding_box_tess_id).unwrap();
                                tess_gate.render(TessSlice::one_whole(bounding_box_tess_ref));
                            }
                        }
                    });
                });
            });
        tess_manager.end();
        self.surface.borrow_mut().swap_buffers();

        let mut resize = false;

        for event in self.surface.borrow_mut().poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    *(*self.should_quit).lock().unwrap() = true;
                }
                WindowEvent::FramebufferSize(..) => {
                    resize = true;
                }
                e => {
                    (events.0).push(e);
                },
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
