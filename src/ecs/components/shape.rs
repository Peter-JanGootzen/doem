use specs::prelude::*;
use doem_math::vector_space::Vector3;

pub struct AABB {
    middle_point: Vector3,
    half_size: Vector3,
}

pub struct Shape {
    pub tesselation_id: usize,
    pub bounding_box: AABB
}

impl Component for Shape {
    type Storage = VecStorage<Self>;
}
