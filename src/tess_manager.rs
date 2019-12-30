use std::sync::Arc;
use luminance::tess::Tess;
use std::collections::BTreeSet;

pub struct TessManager {
    tesselations: Vec<Option<Tess>>,
    used: BTreeSet<usize>
}

impl TessManager {
    pub fn new() -> TessManager {
        TessManager {
            tesselations: Vec::<Option<Tess>>::new(),
            used: BTreeSet::<usize>::new()
        }
    }
    pub fn get_tess(&mut self, id: usize) -> Option<&Tess> {
        if id >= self.tesselations.len() { return None; }

        self.used.insert(id);
        match &self.tesselations[id] {
            Some(tess) => Some(&tess),
            None => None
        }
    }
    pub fn end(&mut self) {
        for (id, tess) in self.tesselations.iter_mut().enumerate() {
            if self.used.contains(&id) {
                *tess = None;
            }
        }
        self.used = BTreeSet::<usize>::new();
    }
}
