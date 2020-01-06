use specs::prelude::*;

pub struct Transformable;

impl Component for Transformable {
    type Storage = VecStorage<Self>;
}
