use rand::Rng;
use core::f64::consts::PI;
use geo::Contains;
use geo_types::{Point, Polygon};

#[derive(Default, Debug, Clone)]
pub struct Trees {
    trees: Vec<Tree>,
}

impl Trees {
    pub fn new(trees: Vec<Tree>) -> Self {
        Trees { trees }
    }

    fn get(&self, index: usize) -> Option<&Tree> {
        self.trees.get(index)
    }

    // Generate a list of trees with variable radius Poisson disc sampling
    pub fn variable_radius_poisson_disc_sampling(&self, p: &Polygon) -> Vec<Tree> {
        let mut final_trees = Vec::new();
        let mut active_list = Vec::new();
    
        // Start with an initial tree
        if let Some(initial_tree) = self.get(0).cloned() {
            active_list.push(initial_tree.clone());
            final_trees.push(initial_tree);
        }
    
        while !active_list.is_empty() {
            // Choose a random active tree
            let index = rand::thread_rng().gen_range(0..active_list.len());
            let tree = active_list.remove(index);
    
            // Calculate the radius of the tree
            // Other trees must remain outside this radius
            let radius = tree.calculate_radius();
            let mut found = false;
    
            for _ in 0..30 { // Number of attempts to place a new point around an active point
                let new_position = tree.generate_random_point_around(radius);
                
                let new_tree = Tree {
                    species: tree.species.clone(),
                    mean_height: tree.mean_height,
                    position: new_position,
                };
    
                if new_tree.is_valid(&final_trees, p) {
                    final_trees.push(new_tree.clone());
                    active_list.push(new_tree);
                    found = true;
                    break;
                }
            }
    
            if !found {
                active_list.retain(|t| t.position != tree.position);
            }
        }
    
        final_trees
    }
}

#[derive(Default, Debug, Clone)]
pub struct Tree {
    species: i64,
    mean_height: f64,
    position: (f64, f64),
}

impl Tree {
    pub fn new(species: i64, mean_height: f64, position: (f64, f64)) -> Self {
        Tree {
            species,
            mean_height,
            position,
        }
    }

    pub fn species(&self) -> i64 {
        self.species
    }

    pub fn position(&self) -> (f64, f64) {
        self.position
    }

    fn calculate_radius(&self) -> f64 {
        // TODO: Adjust this function to fine-tune spacing between trees
        let scaling_factor = match self.species {
            1 => 1.0, // For species 1
            2 => 10.0, // For species 2
            3 => 0.8, // For species 3
            4 => 5.5, // For species 4
            _ => 0.8, // Default scaling factor
        };
        self.mean_height * scaling_factor
    }
    
    fn generate_random_point_around(&self, radius: f64) -> (f64, f64) {
        let angle = rand::thread_rng().gen_range(0.0..2.0 * PI);
        let r = rand::thread_rng().gen_range(radius..2.0 * radius);

        // Calculate the new point's coordinates from the angle and radius
        (self.position.0 + r * angle.cos(), self.position.1 + r * angle.sin())
    }

    // New tree is valid when it is within the bounds of the image and does not overlap with existing trees
    fn is_valid(&self, trees: &[Tree], p: &Polygon<f64>) -> bool {
        // Check if the tree is inside the polygon
        if !p.contains(&Point::new(self.position.0, self.position.1)) {
            return false;
        }

        // Check if the tree overlaps with any existing trees
        for tree in trees.iter() {
            let distance = euclidean_distance(self.position, tree.position);
            if distance < tree.calculate_radius() {
                return false;
            }
        }

        true
    }
}

fn euclidean_distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}
