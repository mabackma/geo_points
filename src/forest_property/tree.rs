#[derive(Default, Debug, Clone)]
pub struct Tree {
    species: i64,
    mean_height: f64,
    position: (f64, f64, f64),
}

impl Tree {
    pub fn new(species: i64, mean_height: f64, position: (f64, f64, f64)) -> Self {
        Tree {
            species,
            mean_height,
            position,
        }
    }

    pub fn species(&self) -> i64 {
        self.species
    }

    pub fn position(&self) -> (f64, f64, f64) {
        self.position
    }
}