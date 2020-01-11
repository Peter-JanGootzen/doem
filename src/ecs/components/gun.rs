use doem_math::vector_space::Vector3;
use specs::prelude::*;

pub struct Gun {
    pub damage: f32,
    pub velocity: Vector3,
    pub despawn_bullet_on_impact: bool,
}
impl Component for Gun {
    type Storage = VecStorage<Self>;
}
