use std::collections::BTreeSet;
use specs::prelude::*;

pub struct Collisions(pub BTreeSet<(Entity, Entity)>);

impl Default for Collisions {
    fn default() -> Collisions {
        Collisions(BTreeSet::<(Entity, Entity)>::new())
    }
}
