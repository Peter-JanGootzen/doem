use std::collections::BTreeSet;
use specs::world::Index;

pub struct Collisions(pub BTreeSet<(Index, Index)>);

impl Default for Collisions {
    fn default() -> Collisions {
        Collisions(BTreeSet::<(Index, Index)>::new())
    }
}
