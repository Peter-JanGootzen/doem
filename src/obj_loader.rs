use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::collections::HashMap;
use luminance::tess::{Mode, Tess, TessBuilder, TessError};
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

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<VertexIndex> = Vec::new();


        for object in objects.into_iter() {
            for geometry in object.geometry {
                println!("loading {}", object.name);
                println!("{} vertices", object.vertices.len());
                println!("{} shapes", geometry.shapes.len());

                // build up vertices; for this to work, we remove duplicated vertices by putting them in a
                // map associating the vertex with its ID
                let mut vertex_cache: HashMap<obj::VTNIndex, VertexIndex> = HashMap::new();
                for (i, shape) in (&geometry.shapes).into_iter().enumerate() {
                    let color = i as f32 / geometry.shapes.len() as f32;
                    if let obj::Shape::Triangle(a, b, c) = shape {
                        for key in &[a, b, c] {
                            if let Some(vertex_index) = vertex_cache.get(key) {
                                indices.push(*vertex_index);
                            } else {
                                let p = object.vertices[key.0];
                                let vertex = Vertex {
                                    pos: VertexPosition::new([p.x as f32, p.y as f32, p.z as f32]),
                                    color: VertexColor::new([color, color, color ])
                                };
                                let vertex_index = vertices.len() as VertexIndex;
        
                                vertex_cache.insert(**key, vertex_index);
                                vertices.push(vertex);
                                indices.push(vertex_index);
                            }
                        }
                    } else {
                        return Err("unsupported non-triangle shape".to_owned());
                    }
                }

            }
        }

        Ok(ObjLoader { vertices, indices })
    }
}
