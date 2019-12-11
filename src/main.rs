mod gl_common;
mod obj_loader;
mod shape;

use crate::gl_common::{VertexSemantics, ShaderInterface};
use crate::obj_loader::ObjLoader;
use crate::shape::Shape;

use std::path::Path;
use clap::{Arg, App};
use luminance::context::GraphicsContext;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};
use luminance::tess::TessSlice;
use doem_math::vector_space::{ Matrix4, PI };
use cgmath;
use cgmath::EuclideanSpace;

const VS: &str = include_str!("displacement-vs.glsl");
const FS: &str = include_str!("displacement-fs.glsl");

const FOVY: cgmath::Rad<f32> = cgmath::Rad(PI / 2.0);
const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 1000.;

fn main() {
    let matches = App::new("Rusty obj viewer")
                          .version("1.0")
                          .author("Bram-Boris Meerlo and Peter-Jan Gootzen")
                          .about("Made using our own linear algebra crate rusty_linear_algebra.")
                          .arg(Arg::with_name("model_path")
                               .short("m")
                               .long("model")
                               .value_name("MODEL_PATH")
                               .help("Sets the wavefront obj model that is going to be loaded")
                               .index(1)
                               .required(true)
                               .takes_value(true))
                          .get_matches();

    let model_path_str = matches.value_of("model_path").unwrap();
    let model_path = Path::new(model_path_str);

    start(&model_path);
}

fn start(model_path: &Path) {
    let mut surface = GlfwSurface::new(
        WindowDim::Windowed(1600, 900),
        "Doem",
        WindowOpt::default(),
    )
    .expect("GLFW surface creation");

    // see the use of our uniform interface here as thirds type variable
    let program = Program::<VertexSemantics, (), ShaderInterface>::from_strings(None, VS, None, FS)
        .expect("program creation")
        .ignore_warnings();

    let mut back_buffer = surface.back_buffer().unwrap();

    let shape_obj = ObjLoader::load(model_path).unwrap();
    let shape_aabb_tess = shape_obj.generate_aabb_tess(&mut surface).unwrap();
    let shape_tess = shape_obj.to_tess(&mut surface).unwrap();
    let shape_tesselations = vec!(shape_tess, shape_aabb_tess);
    let mut shape = Shape::new(shape_tesselations);

    let projection = cgmath::perspective(FOVY, surface.width() as f32 / surface.height() as f32, Z_NEAR, Z_FAR);
    let view = cgmath::Matrix4::<f32>::look_at(cgmath::Point3::new(10., 10., 10.), cgmath::Point3::origin(), cgmath::Vector3::unit_y());

    let mut resize = false;

    'app: loop {
        for event in surface.poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'app
                }

                WindowEvent::Key(Key::A, _, action, _)
                | WindowEvent::Key(Key::Left, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    shape.position.data[0][3] += -0.01;
                }

                WindowEvent::Key(Key::D, _, action, _)
                | WindowEvent::Key(Key::Right, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    shape.position.data[0][3] += 0.01;
                }

                WindowEvent::Key(Key::W, _, action, _)
                | WindowEvent::Key(Key::Up, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    shape.position.data[1][3] += 0.01;
                }

                WindowEvent::Key(Key::S, _, action, _)
                | WindowEvent::Key(Key::Down, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    shape.position.data[1][3] += -0.01;
                }

                WindowEvent::Key(Key::K, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    shape.scaling.data[0][0] += 0.1;
                    shape.scaling.data[1][1] += 0.1;
                    shape.scaling.data[2][2] += 0.1;
                }
                WindowEvent::Key(Key::J, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    shape.scaling.data[0][0] -= 0.1;
                    shape.scaling.data[1][1] -= 0.1;
                    shape.scaling.data[2][2] -= 0.1;
                }
                WindowEvent::Key(Key::Q, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    shape.rotate_y(-0.05);
                }
                WindowEvent::Key(Key::E, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    shape.rotate_y(0.05);
                }
                WindowEvent::Key(Key::Z, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    shape.rotate_x(-0.05);
                }
                WindowEvent::Key(Key::X, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    shape.rotate_x(0.05);
                }
                WindowEvent::Key(Key::T, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    shape.rotate_z(-0.05);
                }
                WindowEvent::Key(Key::G, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    shape.rotate_z(0.05);
                }
 
                WindowEvent::FramebufferSize(..) => {
                    resize = true;
                }

                _ => (),
            }
        }

        if resize {
            back_buffer = surface.back_buffer().unwrap();
            resize = false;
        }

        surface
            .pipeline_builder()
            .pipeline(&back_buffer, [0., 0., 0., 0.], |_, mut shd_gate| {
                // notice the iface free variable, which type is &ShaderInterface
                shd_gate.shade(&program, |iface, mut rdr_gate| {
                    iface.projection.update(projection.into());
                    iface.view.update(view.into());

                    rdr_gate.render(RenderState::default(), |mut tess_gate| {
                        iface.transform.update(shape.get_transformation()
                            .transpose().copy_to_array());
                        let tesselations = shape.get_tesselations();
                        for tess in tesselations {
                            tess_gate.render(TessSlice::one_whole(tess));
                        }
                    });
                });
            });
        surface.swap_buffers();
    }
}
