mod geometry_utils;

use geometry_utils::*;
use chrono::{DateTime, Utc}; 
use geo_types::{Coord, LineString, Polygon};

struct Stand {
    stand_basic_data: StandBasicData,
    tree_stand_data: TreeStandData,
}

struct StandBasicData {
    change_state: u32,
    change_time: DateTime<Utc>, 
    complete_state: u32,
    stand_number: u32,
    stand_number_extension: String,
    main_group: u32,
    sub_group: u32,
    fertility_class: u32,
    soil_type: u32,
    drainage_state: u32,
    development_class: u32,
    stand_quality: u32,
    main_tree_species: u32,
    accessibility: u32,
    stand_basic_data_date: DateTime<Utc>, 
    area: u32,
    point_property: Coord<f64>,
    exterior_lr: Coord<f64>,
    interior_lr: Coord<f64>,
}

struct TreeStrata {
    change_state: u32,
    stratum_number: u32,
    tree_species: u32,
    storey: u32,
    age: u32,
    basal_area: f64,
    mean_diameter: u32,
    mean_height: f64,
    data_source: u32,
}

struct TreeStandSummary {
    change_state: u32,
    mean_age: u32,
    basal_area: f64,
    stem_count: u32,
    mean_diameter: u32,
    mean_height: f64,
    volume: u32,
    volume_growth: u32,
}

struct TreeStandData {
    tree_strata: TreeStrata,
    tree_stand_summary: TreeStandSummary,
}

fn main() {
    let mut coordinates = Vec::new();

    // Ask user to input coordinates for polygon
    loop {
        let coordinate = create_point();
        if coordinate.x == -1.0 && coordinate.y == -1.0 {
            break;
        } else {
            coordinates.push(coordinate);
        }
    }

    // Create polygon from coordinates
    let line_string = LineString::new(coordinates);
    let polygon = Polygon::new(line_string, vec![]);
    println!("polygon: {:?}", polygon);

    // Generate random points within the polygon
    let random_points = generate_random_points(&polygon, 10);
    println!("random_points within polygon: {:?}", random_points);  

    draw_image(&polygon, random_points);
}

