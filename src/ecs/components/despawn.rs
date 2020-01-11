use specs::prelude::*;

pub struct Despawn;

impl Component for Despawn {
    type Storage = VecStorage<Despawn>;
}
