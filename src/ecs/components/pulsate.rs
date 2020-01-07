use doem_math::vector_space::Vector3;
use specs::prelude::*;

pub struct Pulsate {
    pub speed: Vector3,
    pub current_direction: bool,
    pub max_scale: Vector3,
    pub min_scale: Vector3
}

impl Component for Pulsate {
    type Storage = VecStorage<Self>;
}

