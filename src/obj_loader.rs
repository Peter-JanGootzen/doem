use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::collections::HashMap;
use luminance::tess::{Mode, Tess, TessBuilder, TessError};
use luminance::context::GraphicsContext;
use wavefront_obj::obj;
use doem_math::vector_space::Vector3;

use crate::gl_common::{Vertex, VertexColor, VertexPosition};

pub struct ObjLoader {
    vertices: Vec<Vertex>,
    indices: Vec<VertexIndex>,
    pub middle_point: Vector3,
    pub x_half_size: f32,
    pub y_half_size: f32,
    pub z_half_size: f32,
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

        let mut min_x: Option<f32> = None;
        let mut min_y: Option<f32> = None;
        let mut min_z: Option<f32> = None;
        let mut max_x: Option<f32> = None;
        let mut max_y: Option<f32> = None;
        let mut max_z: Option<f32> = None;

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

                                min_x = Self::parse_min_value(min_x, p.x as f32);
                                min_y = Self::parse_min_value(min_y, p.y as f32);
                                min_z = Self::parse_min_value(min_z, p.z as f32);
                                max_x = Self::parse_max_value(max_x, p.x as f32);
                                max_y = Self::parse_max_value(max_y, p.y as f32);
                                max_z = Self::parse_max_value(max_z, p.z as f32);
                            }
                        }
                    } else {
                        return Err("unsupported non-triangle shape".to_owned());
                    }
                }
            }
        }

        let min_x = min_x.unwrap();
        let min_y = min_y.unwrap();
        let min_z = min_z.unwrap();
        let max_x = max_x.unwrap();
        let max_y = max_y.unwrap();
        let max_z = max_z.unwrap();

        let x_half_size = (max_x - min_x) / 2.0;
        let y_half_size = (max_y - min_y) / 2.0;
        let z_half_size = (max_z - min_z) / 2.0;

        let middle_point = Vector3::new_from_array([
            [ min_x + x_half_size ],
            [ min_y + y_half_size ],
            [ min_z + z_half_size ],
        ]);

        Ok(Self { vertices, indices, middle_point, x_half_size, y_half_size, z_half_size })
    }

    fn parse_min_value(old: Option<f32>, new: f32) -> Option<f32> {
        match old {
            None => Some(new),
            Some(value) => {
                if new < value {
                    Some(new)
                } else {
                    old
                }
            }
        }
    }
    fn parse_max_value(old: Option<f32>, new: f32) -> Option<f32> {
        match old {
            None => Some(new),
            Some(value) => {
                if new > value {
                    Some(new)
                } else {
                    old
                }
            }
        }
    }

    pub fn generate_aabb_tess<C>(&self, ctx: &mut C) -> Result<Tess, TessError> where C: GraphicsContext {
        let mut aabb_vertices: Vec<Vertex> = Vec::new();

        let color = VertexColor::new([0.0, 1.0, 0.0 ]);
        let min_x = self.middle_point.data[0][0] - self.x_half_size;
        let min_y = self.middle_point.data[1][0] - self.y_half_size;
        let min_z = self.middle_point.data[2][0] - self.z_half_size;
        let max_x = self.middle_point.data[0][0] + self.x_half_size;
        let max_y = self.middle_point.data[1][0] + self.y_half_size;
        let max_z = self.middle_point.data[2][0] + self.z_half_size;

        aabb_vertices.push(Vertex {
            pos: VertexPosition::new([min_x, min_y, min_z]),
            color
        });
        aabb_vertices.push(Vertex {
            pos: VertexPosition::new([max_x, min_y, min_z]),
            color
        });
        aabb_vertices.push(Vertex {
            pos: VertexPosition::new([min_x, max_y, min_z]),
            color
        });
        aabb_vertices.push(Vertex {
            pos: VertexPosition::new([min_x, min_y, max_z]),
            color
        });
        aabb_vertices.push(Vertex {
            pos: VertexPosition::new([max_x, max_y, min_z]),
            color
        });
        aabb_vertices.push(Vertex {
            pos: VertexPosition::new([min_x, max_y, max_z]),
            color
        });
        aabb_vertices.push(Vertex {
            pos: VertexPosition::new([max_x, min_y, max_z]),
            color
        });
        aabb_vertices.push(Vertex {
            pos: VertexPosition::new([max_x, max_y, max_z]),
            color
        });
        aabb_vertices.push(Vertex {
            pos: VertexPosition::new([0.0, 0.0, 0.0]),
            color
        });
        aabb_vertices.push(Vertex {
            pos: VertexPosition::new([max_x * 2.0, 0.0, 0.0]),
            color: VertexColor::new([1.0, 0.0, 0.0 ])
        });
        aabb_vertices.push(Vertex {
            pos: VertexPosition::new([0.0, max_y * 2.0, 0.0]),
            color: VertexColor::new([0.0, 1.0, 0.0 ])
        });
        aabb_vertices.push(Vertex {
            pos: VertexPosition::new([0.0, 0.0, max_z * 2.0]),
            color: VertexColor::new([0.0, 0.0, 1.0 ])
        });

        let aabb_indices: Vec<VertexIndex> = vec!(
            0, 1,
            0, 3,
            0, 2,
            1, 0,
            1, 6,
            1, 4,
            6, 3,
            6, 7,
            3, 5,
            2, 4,
            2, 5,
            7, 4,
            7, 5,
            8, 9,
            8, 10,
            8, 11
        );
        TessBuilder::new(ctx)
            .set_mode(Mode::Line)
            .add_vertices(aabb_vertices)
            .set_indices(aabb_indices)
            .build()
    }
}
