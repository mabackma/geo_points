use crate::forest_property::geometry::Polygon;

use super::tree::Tree;


// Struct that represents a stand of trees
pub struct StandTrees {
    trees: Vec<Tree>,
    polygon: Polygon,
}

impl StandTrees {
    pub fn new(trees: Vec<Tree>, polygon: Polygon) -> Self {
        StandTrees {
            trees,
            polygon,
        }
    }

    pub fn trees(&self) -> &Vec<Tree> {
        &self.trees
    }

    pub fn polygon(&self) -> &Polygon {
        &self.polygon
    }

    // TODO: Use this method for all StandTrees structs that are found inside the bounding box
    pub fn trees_in_bounding_box(&self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Vec<&Tree> {
        self.trees.iter().filter(|tree| {
            let (x, y, _) = tree.position();
            x >= min_x && x <= max_x && y >= min_y && y <= max_y  // Keep the tree if it is inside the bounding box
        }).collect()
    }
}