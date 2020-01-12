use doem_math::Vector3;
use specs::prelude::*;

pub struct Physics {
    pub velocity: Vector3,
}

impl Component for Physics {
    type Storage = VecStorage<Self>;
}
