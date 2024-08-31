use std::borrow::Borrow;

use super::stand;
use super::stand::Stand;
use crate::forest_property::tree::Tree;
use crate::geometry_utils::generate_random_trees;

use geo::Intersects;
use geo::Polygon;
use geo_clipper::Clipper;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

// Struct that represents a stand of trees
#[derive(Debug, Clone)]
pub struct Compartment {
    pub stand_number: String,
    pub trees: Vec<Tree>,
    pub polygon: Polygon,
}

impl Compartment {
    pub fn new(stand_number: String, trees: Vec<Tree>, polygon: Polygon) -> Self {
        Compartment {
            stand_number,
            trees,
            polygon,
        }
    }

    pub fn stand_number(&self) -> &String {
        &self.stand_number
    }

    pub fn trees(&self) -> &Vec<Tree> {
        &self.trees
    }

    pub fn polygon(&self) -> &Polygon {
        &self.polygon
    }

    // Polygon clipping to bounding box
    pub fn clip_polygon_to_bounding_box(&self, bbox: &Polygon) -> Option<Polygon> {
        let clipped = self.polygon.intersection(bbox, 100000.0);

        if clipped.0.is_empty() {
            println!("Polygon is empty");
            None
        } else {
            let p = clipped.0.first().unwrap();
            Some(p.to_owned())
        }
    }

    // Get trees in a bounding box
    pub fn trees_in_bounding_box(
        &self,
        min_x: f64,
        max_x: f64,
        min_y: f64,
        max_y: f64,
    ) -> Vec<&Tree> {
        self.trees
            .iter()
            .filter(|tree| {
                let (x, y, _) = tree.position();
                x >= min_x && x <= max_x && y >= min_y && y <= max_y // Keep the tree if it is inside the bounding box
            })
            .collect()
    }
}

// Get compartments in a bounding box.
pub fn get_compartments_in_bounding_box(all_stands: Vec<Stand>, bbox: &Polygon) -> Vec<Compartment> {
    
    // Find stands in the bounding box
    let stands_iterator = all_stands
    .into_iter()
    .filter( |stand| bbox.intersects(stand.computed_polygon.as_ref().unwrap().exterior()));
    // If there are stands in the bounding box, generate random trees for each stand

    let compartments: Vec<Compartment> = stands_iterator
        .map(|stand| {

            let polygon = stand.computed_polygon.as_ref().unwrap();
            let strata = stand.get_strata();

            let trees = generate_random_trees(&polygon, strata);

            let compartment = Compartment {
                stand_number: stand.stand_basic_data.stand_number.to_string(),
                trees,
                polygon: polygon.to_owned(),
            };

            compartment
        })
        .collect();

    compartments
}
