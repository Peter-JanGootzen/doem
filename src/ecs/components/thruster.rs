use doem_math::Vector3;
use specs::prelude::*;

pub struct Thruster {
    pub power: Vector3,
}

impl Component for Thruster {
    type Storage = VecStorage<Self>;
}
