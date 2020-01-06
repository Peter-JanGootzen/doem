use specs::prelude::*;
use doem_math::vector_space::Vector3;

pub struct Physics {
    pub velocity: Vector3
}

impl Component for Physics {
    type Storage = VecStorage<Self>;
}
