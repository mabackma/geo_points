use crate::forest_property::tree::Tree;

use geo::{Coord, LineString, Polygon};
use geo::Intersects;
use geo::line_string;

use super::stand::Stand;

// Struct that represents a stand of trees
pub struct Compartment {
    pub trees: Vec<Tree>,
    pub polygon: Polygon,
}

impl Compartment {
    pub fn new(trees: Vec<Tree>, polygon: Polygon) -> Self {
        Compartment {
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

    // TODO: Use this method for all Compartment structs that are found inside the bounding box
    pub fn trees_in_bounding_box(&self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Vec<&Tree> {
        self.trees.iter().filter(|tree| {
            let (x, y, _) = tree.position();
            x >= min_x && x <= max_x && y >= min_y && y <= max_y  // Keep the tree if it is inside the bounding box
        }).collect()
    }
}

pub fn find_stands_in_bounding_box(stands: &Vec<Stand>, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Option<Vec<&Stand>> {
    let b_box_line_string = line_string![
        (x: min_x, y: min_y), 
        (x: max_x, y: min_y),
        (x: max_x, y: max_y),
        (x: min_x, y: max_y),
        (x: min_x, y: min_y)  // Close the polygon
    ];

    // Collect the stands that intersect with the bounding box
    let intersecting_stands: Vec<&Stand> = stands.iter().filter(|stand| {
        let stand_coordinate_sring = stand.stand_basic_data.polygon_geometry.polygon_property.polygon.exterior.linear_ring.coordinates.clone();
        let stand_coordinates = get_coordinates_from_string(&stand_coordinate_sring);
        let stand_line_string = LineString::from(stand_coordinates);
        stand_line_string.intersects(&b_box_line_string)
    }).collect();  // Collect the stands that intersect with the bounding box

    if intersecting_stands.is_empty() {
        println!("No stands found in the bounding box");
        None
    } else {
        Some(intersecting_stands)
    }
}

fn get_coordinates_from_string(coord_string: &str) -> Vec<Coord> {
    let coordinates_str: Vec<&str> = coord_string.split(" ").collect();

    // Parse coordinates into a Vec of `(f64, f64)`
    let mut coords: Vec<Coord> = Vec::new();
    for coordinate in coordinates_str {
        let parts: Vec<&str> = coordinate.split(',').collect();
        if parts.len() == 2 {
            let x: f64 = parts[0].parse().expect("Invalid x coordinate");
            let y: f64 = parts[1].parse().expect("Invalid y coordinate");
            coords.push(Coord { x, y });
        } else {
            println!("Invalid coordinate format: {}", coordinate);
        }
    }

    coords
}