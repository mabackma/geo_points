use std::ops::Mul;

use crate::forest_property::tree::Tree;
use crate::geometry_utils::generate_random_trees;
use super::stand::Stand;

use geo::{polygon, Coord, LineString, MultiPolygon, Polygon};
use geo::Intersects;
use geo::line_string;
use geo_clipper::Clipper;

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

    // Polygon clipping to bounding box
    pub fn clip_polygon_to_bounding_box(&self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Option<Polygon> {
        let bbox = Polygon::new(
            LineString(vec![
                Coord { x: min_x, y: min_y },
                Coord { x: max_x, y: min_y },
                Coord { x: max_x, y: max_y },
                Coord { x: min_x, y: max_y },
                Coord { x: min_x, y: min_y },
            ]),
            vec![]
        );

        let clipped = self.polygon.intersection(&bbox, 1.0);

        if clipped.0.is_empty() {
            None
        } else {
            let exterior = clipped.0.first().unwrap().exterior().clone();
            let interior = clipped.0.first().unwrap().interiors().clone();
            let polygon = Polygon::new(exterior, interior.to_vec());
            Some(polygon)
        }
    }

    // Get trees in a bounding box
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

pub fn get_compartments_in_bounding_box(all_stands: Vec<&Stand>, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Vec<Compartment> {
    let mut compartments = Vec::new();
    let mut stands = Vec::new();

    for stand in all_stands {
        stands.push(stand.clone());
    }
    println!("\nTotal stands: {:?}", stands.len());

    // Find stands in the bounding box
    let stands = find_stands_in_bounding_box(&stands, min_x, max_x, min_y, max_y);

    // If there are stands in the bounding box, generate random trees for each stand
    if !stands.is_none() {
        println!("Stands in bounding box: {:?}", stands.clone().unwrap().len());
        for stand in &stands.unwrap() {
            println!("\n\nStand number {:?}", stand.stand_basic_data.stand_number);

            let polygon = stand.create_polygon();
            let strata = stand.get_strata().expect("No treeStrata/stratums found");
            let trees = generate_random_trees(&polygon, &strata);
            
            let compartment = Compartment {
                trees,
                polygon,
            };
            
            compartments.push(compartment);
        }
    }
    compartments
}
