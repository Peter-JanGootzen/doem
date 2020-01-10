use crate::data::AABB;
use specs::prelude::*;

#[derive(Clone)]
pub enum Shape {
    Init {
        tess_id: usize,
        bounding_box: AABB,
        bounding_box_tess_id: Option<usize>,
    },
    Unit {
        obj_path: String,
    },
}

impl Component for Shape {
    type Storage = VecStorage<Self>;
}
