use doem_math::vector_space::Vector3;
use specs::prelude::*;

#[derive(Clone)]
pub struct AABB {
    pub middle_point: Vector3,
    pub half_size: Vector3,
}

#[derive(Clone)]
pub enum Shape {
    Init {
        tess_id: usize,
        bounding_box: AABB,
        bounding_box_tess_id: usize,
    },
    Unit {
        obj_path: String,
    },
}

impl Component for Shape {
    type Storage = VecStorage<Self>;
}
