use doem_math::vector_space::Vector3;
use specs::prelude::*;

pub struct FollowCamera {
    pub zoom_level: f32,
    pub offset: Vector3,
}

impl Component for FollowCamera {
    type Storage = VecStorage<Self>;
}
