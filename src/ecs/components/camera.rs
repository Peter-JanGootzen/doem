use doem_math::vector_space::{ Vector3, Matrix4 };
use specs::prelude::*;

pub struct Camera {
    pub zoom_level: f32,
    pub offset: Vector3,
    pub orientation: Matrix4,
}

impl Component for Camera {
    type Storage = VecStorage<Self>;
}
