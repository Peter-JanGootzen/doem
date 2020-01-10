use crate::ecs::components::shape::Shape;
use crate::data::AABB;
use crate::obj_loader::ObjLoader;
use doem_math::vector_space::Vector3;
use luminance::tess::Tess;
use luminance_glfw::GlfwSurface;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

pub struct TessManager {
    surface: Rc<RefCell<GlfwSurface>>,
    tesselations: Vec<Option<Tess>>,
    used: BTreeSet<usize>,
    path_index: HashMap<String, Shape>,
}

impl TessManager {
    pub fn new(surface: Rc<RefCell<GlfwSurface>>) -> Self {
        Self {
            surface,
            tesselations: Vec::<Option<Tess>>::new(),
            used: BTreeSet::<usize>::new(),
            path_index: HashMap::<String, Shape>::new(),
        }
    }
    pub fn get_tess(&mut self, id: usize) -> Option<&Tess> {
        if id >= self.tesselations.len() {
            return None;
        }

        self.used.insert(id);
        match &self.tesselations[id] {
            Some(tess) => Some(&tess),
            None => None,
        }
    }
    pub fn get_aabb_id(&mut self, aabb: &AABB) -> usize {
        let tess = ObjLoader::generate_aabb(aabb, &mut *self.surface.borrow_mut()).unwrap();
        self.tesselations.push(Some(tess));
        let id = self.tesselations.len() - 1;
        self.used.insert(id);
        id
    }
    pub fn end(&mut self) {
        for (id, tess) in self.tesselations.iter_mut().enumerate() {
            if !self.used.contains(&id) {
                *tess = None;
            }
        }
        self.used = BTreeSet::<usize>::new();
    }
    pub fn init_shape(&mut self, shape: Shape) -> Shape {
        match shape {
            Shape::Init { .. } => {
                return shape;
            }
            Shape::Unit { obj_path } => match &self.path_index.get(&obj_path) {
                Some(shape) => (*shape).clone(),
                None => {
                    let tesselation = ObjLoader::load(Path::new(&obj_path)).unwrap();
                    let bounding_box = AABB {
                        middle_point: tesselation.middle_point.clone(),
                        half_size: Vector3::new_from_array([
                            [tesselation.x_half_size],
                            [tesselation.y_half_size],
                            [tesselation.z_half_size],
                        ]),
                    };
                    let shape_tess = tesselation
                        .to_tess(&mut *self.surface.borrow_mut())
                        .unwrap();
                    self.tesselations.push(Some(shape_tess));
                    let tess_id = self.tesselations.len() - 1;

                    let shape = Shape::Init {
                        tess_id,
                        bounding_box,
                        bounding_box_tess_id: None,
                    };
                    let shape_clone = shape.clone();
                    self.path_index.insert(obj_path, shape);

                    shape_clone
                }
            },
        }
    }
}
