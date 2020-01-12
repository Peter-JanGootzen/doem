use doem_math::{Matrix4, Vector3};
use specs::prelude::*;

pub struct Transform {
    pub position: Vector3,
    pub scale: Vector3,
    pub orientation: Matrix4,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
