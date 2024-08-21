use crate::forest_property::tree::Tree;
use crate::forest_property::geometry::Polygon;

// Struct that represents a stand of trees
pub struct GeoTree {
    trees: Vec<Tree>,
    polygon: Polygon,
}

impl GeoTree {
    pub fn new(trees: Vec<Tree>, polygon: Polygon) -> Self {
        GeoTree {
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
}