use specs::prelude::*;
use doem_math::vector_space::Vector3;

pub struct FollowCamera {
    pub zoom_level: f32,
    pub offset: Vector3
}

impl Component for FollowCamera {
    type Storage = VecStorage<Self>;
}
