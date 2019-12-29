use specs::prelude::*;
use doem_math::vector_space::Vector3;

pub struct Transform {
    pub position: Vector3,
    pub scale: Vector3,
    pub rotation: Vector3
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}