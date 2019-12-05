mod common;

use crate::common::{GLVertex, GLVertexColor, Semantics, Vertex};
use luminance::context::GraphicsContext as _;
use luminance::render_state::RenderState;
use luminance::shader::program::{Program, Uniform};
use luminance::tess::{Mode, TessBuilder};
use luminance_derive::UniformInterface;
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};
use rusty_linear_algebra::vector_space::Matrix4;

const VS: &str = include_str!("displacement-vs.glsl");
const FS: &str = include_str!("displacement-fs.glsl");

const COORDINATE_SYSTEM_X_UPPER: [Vertex; 22] = [
    Vertex {
        pos: GLVertex::new([1.0, 0.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, 0.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, 0.1]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, 0.1]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, 0.2]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, 0.2]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, 0.3]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, 0.3]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, 0.4]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, 0.4]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, 0.5]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, 0.5]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, 0.6]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, 0.6]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, 0.7]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, 0.7]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, 0.8]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, 0.8]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, 0.9]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, 0.9]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
];
const COORDINATE_SYSTEM_X_LOWER: [Vertex; 20] = [
    Vertex {
        pos: GLVertex::new([1.0, -0.1]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, -0.1]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, -0.2]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, -0.2]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, -0.3]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, -0.3]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, -0.4]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, -0.4]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, -0.5]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, -0.5]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, -0.6]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, -0.6]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, -0.7]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, -0.7]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, -0.8]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, -0.8]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, -0.9]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, -0.9]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
];
const COORDINATE_SYSTEM_Y_UPPER: [Vertex; 22] = [
    Vertex {
        pos: GLVertex::new([0.0, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.0, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.1, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.1, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.2, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.2, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.3, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.3, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.4, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.4, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.5, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.5, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.6, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.6, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.7, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.7, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.8, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.8, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.9, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.9, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([1.0, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
];
const COORDINATE_SYSTEM_Y_LOWER: [Vertex; 20] = [
    Vertex {
        pos: GLVertex::new([-0.1, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.1, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.2, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.2, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.3, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.3, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.4, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.4, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.5, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.5, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.6, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.6, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.7, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.7, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.8, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.8, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.9, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-0.9, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, 1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
    Vertex {
        pos: GLVertex::new([-1.0, -1.0]),
        rgb: GLVertexColor::new([0., 1., 0.]),
    },
];

// Only one triangle this time.
const VERTICES: [Vertex; 4] = [
    Vertex {
        pos: GLVertex::new([0.0, 0.0]),
        rgb: GLVertexColor::new([1., 0., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.3, 0.5]),
        rgb: GLVertexColor::new([1., 0., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.0, 0.0]),
        rgb: GLVertexColor::new([1., 0., 0.]),
    },
    Vertex {
        pos: GLVertex::new([0.5, 0.3]),
        rgb: GLVertexColor::new([1., 0., 0.]),
    },
];

// Create a uniform interface. This is a type that will be used to customize the shader. In our
// case, we just want to pass the time and the position of the triangle, for instance.
//
// This macro only supports structs for now; you cannot use enums as uniform interfaces.
#[derive(Debug, UniformInterface)]
struct ShaderInterface {
    transform: Uniform<[[f32; 4]; 4]>,
}

fn main() {
    let mut surface = GlfwSurface::new(
        WindowDim::Windowed(960, 540),
        "Hello, world!",
        WindowOpt::default(),
    )
    .expect("GLFW surface creation");

    // see the use of our uniform interface here as thirds type variable
    let program = Program::<Semantics, (), ShaderInterface>::from_strings(None, VS, None, FS)
        .expect("program creation")
        .ignore_warnings();

    let shape = TessBuilder::new(&mut surface)
        .add_vertices(VERTICES)
        .set_mode(Mode::Line)
        .build()
        .unwrap();

    let coordinate_system_x_lower = TessBuilder::new(&mut surface)
        .add_vertices(COORDINATE_SYSTEM_X_LOWER)
        .set_mode(Mode::Line)
        .build()
        .unwrap();
    let coordinate_system_x_upper = TessBuilder::new(&mut surface)
        .add_vertices(COORDINATE_SYSTEM_X_UPPER)
        .set_mode(Mode::Line)
        .build()
        .unwrap();
    let coordinate_system_y_upper = TessBuilder::new(&mut surface)
        .add_vertices(COORDINATE_SYSTEM_Y_UPPER)
        .set_mode(Mode::Line)
        .build()
        .unwrap();
    let coordinate_system_y_lower = TessBuilder::new(&mut surface)
        .add_vertices(COORDINATE_SYSTEM_Y_LOWER)
        .set_mode(Mode::Line)
        .build()
        .unwrap();

    let mut back_buffer = surface.back_buffer().unwrap();

    let identity = Matrix4::identity();
    let mut translation = Matrix4::identity();
    let mut scale = Matrix4::identity();

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
                    translation.data[0][3] += -0.01;
                }

                WindowEvent::Key(Key::D, _, action, _)
                | WindowEvent::Key(Key::Right, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    translation.data[0][3] += 0.01;
                }

                WindowEvent::Key(Key::W, _, action, _)
                | WindowEvent::Key(Key::Up, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    translation.data[1][3] += 0.01;
                }

                WindowEvent::Key(Key::S, _, action, _)
                | WindowEvent::Key(Key::Down, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    translation.data[1][3] += -0.01;
                }

                WindowEvent::Key(Key::K, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    scale.data[0][0] += 0.1;
                    scale.data[1][1] += 0.1;
                }
                WindowEvent::Key(Key::J, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    scale.data[0][0] -= 0.1;
                    scale.data[1][1] -= 0.1;
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

        let transform = &translation * &scale;
        println!("{}", transform);

        surface
            .pipeline_builder()
            .pipeline(&back_buffer, [0., 0., 0., 0.], |_, mut shd_gate| {
                // notice the iface free variable, which type is &ShaderInterface
                shd_gate.shade(&program, |iface, mut rdr_gate| {
                    // update the time and triangle position on the GPU shader program
                    rdr_gate.render(RenderState::default(), |mut tess_gate| {
                        iface.transform.update(transform.copy_to_array());
                        tess_gate.render(&shape);
                    });

                    rdr_gate.render(RenderState::default(), |mut tess_gate| {
                        iface.transform.update(identity.copy_to_array());
                        tess_gate.render(&coordinate_system_x_upper);
                        tess_gate.render(&coordinate_system_x_lower);
                        tess_gate.render(&coordinate_system_y_lower);
                        tess_gate.render(&coordinate_system_y_upper);
                    });
                });
            });
        surface.swap_buffers();
    }
}
