use specs::prelude::*;
use std::collections::BTreeSet;

pub struct Collisions(pub BTreeSet<(Entity, Entity)>);

impl Default for Collisions {
    fn default() -> Collisions {
        Collisions(BTreeSet::<(Entity, Entity)>::new())
    }
}
