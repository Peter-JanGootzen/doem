mod gl_common;

use crate::gl_common::{Vertex, VertexColor, VertexSemantics, VertexPosition};

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::collections::HashMap;
use luminance::context::GraphicsContext;
use luminance::render_state::RenderState;
use luminance::shader::program::{Program, Uniform};
use luminance::tess::{Mode, Tess, TessBuilder, TessError, TessSliceIndex};
use luminance_derive::UniformInterface;
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};
use rusty_linear_algebra::vector_space::{ Matrix4, PI };
use cgmath;
use cgmath::EuclideanSpace;
use wavefront_obj::{obj};
use try_guard::verify;

const VS: &str = include_str!("displacement-vs.glsl");
const FS: &str = include_str!("displacement-fs.glsl");

const FOVY: cgmath::Rad<f32> = cgmath::Rad(PI / 2.0);
const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 10.;

// Create a uniform interface. This is a type that will be used to customize the shader. In our
// case, we just want to pass the time and the position of the triangle, for instance.
//
// This macro only supports structs for now; you cannot use enums as uniform interfaces.
#[derive(Debug, UniformInterface)]
struct ShaderInterface {
    transform: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    projection: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    view: Uniform<[[f32; 4]; 4]>,
}

type VertexIndex = u32;

struct Obj {
    vertices: Vec<Vertex>,
    indices: Vec<VertexIndex>
}
impl Obj {
    fn to_tess<C>(self, ctx: &mut C) -> Result<Tess, TessError> where C: GraphicsContext {
      TessBuilder::new(ctx)
        .set_mode(Mode::Triangle)
        .add_vertices(self.vertices)
        .set_indices(self.indices)
        .build()
    }
    pub fn load<P>(path: P) -> Result<Self, String> where P: AsRef<Path> {
        let file_content = {
          let mut file = File::open(path).map_err(|e| format!("cannot open file: {}", e))?;
          let mut content = String::new();
          file.read_to_string(&mut content);
          content
        };
        let obj_set = obj::parse(file_content).map_err(|e| format!("cannot parse: {:?}", e))?;
        let objects = obj_set.objects;

        verify!(objects.len() == 1).ok_or("expecting a single object".to_owned())?;

        let object = objects.into_iter().next().unwrap();

        verify!(object.geometry.len() == 1).ok_or("expecting a single geometry".to_owned())?;

        let geometry = object.geometry.into_iter().next().unwrap();

        println!("loading {}", object.name);
        println!("{} vertices", object.vertices.len());
        println!("{} shapes", geometry.shapes.len());

        // build up vertices; for this to work, we remove duplicated vertices by putting them in a
        // map associating the vertex with its ID
        let mut vertex_cache: HashMap<obj::VTNIndex, VertexIndex> = HashMap::new();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<VertexIndex> = Vec::new();
        for shape in geometry.shapes {
          if let obj::Shape::Triangle(a, b, c) = shape {
            for key in &[a, b, c] {
              if let Some(vertex_index) = vertex_cache.get(key) {
                indices.push(*vertex_index);
              } else {
                let p = object.vertices[key.0];
                let vertex = Vertex {
                    pos: VertexPosition::new([p.x as f32, p.y as f32, p.z as f32]),
                    color: VertexColor::new([1.0, 1.0, 0.0 ])
                };
                let vertex_index = vertices.len() as VertexIndex;

                vertex_cache.insert(*key, vertex_index);
                vertices.push(vertex);
                indices.push(vertex_index);
              }
            }
          } else {
            return Err("unsupported non-triangle shape".to_owned());
          }
        }

        Ok(Obj { vertices, indices })
    }
}

fn main() {
    let mut surface = GlfwSurface::new(
        WindowDim::Windowed(960, 540),
        "Hello, world!",
        WindowOpt::default(),
    )
    .expect("GLFW surface creation");

    // see the use of our uniform interface here as thirds type variable
    let program = Program::<VertexSemantics, (), ShaderInterface>::from_strings(None, VS, None, FS)
        .expect("program creation")
        .ignore_warnings();

    let mut back_buffer = surface.back_buffer().unwrap();
    
    let mut angle = 0.0;
    let identity = Matrix4::identity();
    let mut translation = Matrix4::identity();
    let mut scale = Matrix4::identity();

    let shape = Obj::load("suzanne.obj").unwrap();
    let shape_tess = shape.to_tess(&mut surface).unwrap();

    let projection = cgmath::perspective(FOVY, surface.width() as f32 / surface.height() as f32, Z_NEAR, Z_FAR);
    let view = cgmath::Matrix4::<f32>::look_at(cgmath::Point3::new(2., 2., 2.), cgmath::Point3::origin(), cgmath::Vector3::unit_y());

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
                WindowEvent::Key(Key::Q, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    angle -= 0.05;
                }
                WindowEvent::Key(Key::E, _, action, _)
                    if action == Action::Press || action == Action::Repeat =>
                {
                    angle += 0.05;
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

        let mut rotation = Matrix4::new_2d_rotation(angle);
        let transform = &(&translation * &scale) * &rotation;
        println!("{}", transform);

        surface
            .pipeline_builder()
            .pipeline(&back_buffer, [1., 0., 0., 0.], |_, mut shd_gate| {
                // notice the iface free variable, which type is &ShaderInterface
                shd_gate.shade(&program, |iface, mut rdr_gate| {
                    iface.projection.update(projection.into());
                    iface.view.update(view.into());
                    // update the time and triangle position on the GPU shader program
                    rdr_gate.render(RenderState::default(), |mut tess_gate| {
                        iface.transform.update(transform.transpose().copy_to_array());
                        tess_gate.render(shape_tess.slice(..));
                    });

                    rdr_gate.render(RenderState::default(), |mut tess_gate| {
                        iface.transform.update(identity.transpose().copy_to_array());
                    });
                });
            });
        surface.swap_buffers();
    }
}
