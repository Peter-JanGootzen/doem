use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::collections::HashMap;
use luminance::tess::{Mode, Tess, TessBuilder, TessError};
use try_guard::verify;
use luminance::context::GraphicsContext;
use wavefront_obj::obj;


use crate::gl_common::{Vertex, VertexColor, VertexPosition};

pub struct ObjLoader {
    vertices: Vec<Vertex>,
    indices: Vec<VertexIndex>
}

type VertexIndex = u32;


impl ObjLoader {
    pub fn to_tess<C>(self, ctx: &mut C) -> Result<Tess, TessError> where C: GraphicsContext {
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
          file.read_to_string(&mut content).unwrap();
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

        Ok(ObjLoader { vertices, indices })
    }
}
