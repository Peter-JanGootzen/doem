use specs::prelude::*;

pub struct Damage {
    pub damage: f32,
    pub despawn_entity_on_impact: bool,
    pub damage_dealer: Entity,
}
impl Component for Damage {
    type Storage = VecStorage<Self>;
}
