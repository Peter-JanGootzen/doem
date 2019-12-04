//! This program shows how to render a triangle and change its position and color on the fly by
//! updating “shader uniforms”. Those are values stored on the GPU that remain constant for the
//! whole duration of a draw call (you typically change it between each draw call to customize each
//! draw).
//!
//! This example demonstrate how to add time to your shader to start building moving and animated
//! effects.
//!
//! Press the <a>, <s>, <d>, <z> or the arrow keys to move the triangle on the screen.
//! Press <escape> to quit or close the window.
//!
//! https://docs.rs/luminance

mod common;

use crate::common::{Semantics, Vertex, GLVertex, GLVertexColor};
use luminance::context::GraphicsContext as _;
use luminance::render_state::RenderState;
use luminance::shader::program::{Program, Uniform};
use luminance::tess::{Mode, TessBuilder};
use luminance_derive::UniformInterface;
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowEvent, WindowDim, WindowOpt};
use rusty_linear_algebra::vector_space::Matrix3;
use std::time::Instant;

const VS: &'static str = include_str!("displacement-vs.glsl");
const FS: &'static str = include_str!("displacement-fs.glsl");

// Only one triangle this time.
const VERTICES: [Vertex; 2] = [
    Vertex { pos: GLVertex::new([1.0, 1.0, 0.0]), rgb: GLVertexColor::new([1., 0., 0.]) },
    Vertex { pos: GLVertex::new([0.30, 0.20, 0.0]), rgb: GLVertexColor::new([1., 0., 0.]) },
];

// Create a uniform interface. This is a type that will be used to customize the shader. In our
// case, we just want to pass the time and the position of the triangle, for instance.
//
// This macro only supports structs for now; you cannot use enums as uniform interfaces.
#[derive(Debug, UniformInterface)]
struct ShaderInterface {
    transform: Uniform<[[f32; 3]; 3]>
}

fn main() {
    let mut surface = GlfwSurface::new(
        WindowDim::Windowed(960, 540),
        "Hello, world!",
        WindowOpt::default(),
    )
    .expect("GLFW surface creation");

    // see the use of our uniform interface here as thirds type variable
    let program =
        Program::<Semantics, (), ShaderInterface>::from_strings(None, VS, None, FS)
        .expect("program creation")
        .ignore_warnings();

    let shape = TessBuilder::new(&mut surface)
        .add_vertices(VERTICES)
        .set_mode(Mode::Line)
        .build()
        .unwrap();

    let mut back_buffer = surface.back_buffer().unwrap();

    // position of the triangle
    let mut translation = Matrix3::new_from_array([
        [1.0, 0.0,  0.0],
        [0.0, 1.0,  0.0],
        [0.0, 0.0,  1.0],
    ]);
    let mut scale = Matrix3::new_from_array([
        [1.0, 0.0,  1.0],
        [0.0, 1.0,  1.0],
        [0.0, 0.0,  1.0],
    ]);


    // reference time
    let start_t = Instant::now();
    let mut resize = false;

    'app: loop {
        for event in surface.poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,

                WindowEvent::Key(Key::A, _, action, _) | WindowEvent::Key(Key::Left, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    translation.data[0][0] -= 0.1;
                    println!("{}", translation.data[0][0]);
                }

                WindowEvent::Key(Key::D, _, action, _) | WindowEvent::Key(Key::Right, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    translation.data[0][0] += 0.1;
                }

                WindowEvent::Key(Key::W, _, action, _) | WindowEvent::Key(Key::Up, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    translation.data[1][1] += 0.1;
                }

                WindowEvent::Key(Key::S, _, action, _) | WindowEvent::Key(Key::Down, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    translation.data[1][1] -= 0.1;
                }

                WindowEvent::Key(Key::K, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    scale.data[0][2] += 0.1;
                    scale.data[1][2] += 0.1;
                }
                WindowEvent::Key(Key::J, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    scale.data[0][2] -= 0.1;
                    scale.data[1][2] -= 0.1;
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


        // get the current monotonic time
        let elapsed = start_t.elapsed();
        let t64 = elapsed.as_secs() as f64 + (elapsed.subsec_millis() as f64 * 1e-3);
        let t = t64 as f32;

        surface
            .pipeline_builder()
            .pipeline(&back_buffer, [0., 0., 0., 0.], |_, mut shd_gate| {
                // notice the iface free variable, which type is &ShaderInterface
                shd_gate.shade(&program, |iface, mut rdr_gate| {
                    // update the time and triangle position on the GPU shader program
                    let transform = &translation * &scale;
                    println!("{}", transform);
                    iface.transform.update(transform.into_array());

                    rdr_gate.render(RenderState::default(), |mut tess_gate| {
                        // render the dynamically selected slice
                        tess_gate.render(&shape);
                    });
                });
            });
        surface.swap_buffers();
    }
}
