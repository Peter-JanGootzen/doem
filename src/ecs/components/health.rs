use specs::prelude::*;

pub struct Health {
    pub health: f32,
}
impl Component for Health {
    type Storage = VecStorage<Self>;
}
