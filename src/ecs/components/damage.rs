use specs::prelude::*;

pub struct Damage {
    pub damage: f32
}
impl Component for Damage {
    type Storage = VecStorage<Self>;
}