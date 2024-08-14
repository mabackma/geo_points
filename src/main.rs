mod geometry_utils;
mod data_structures;

use data_structures::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use geometry_utils::*;
use geo_types::{Coord, LineString, Polygon};

// Read JSON file
fn read_json_file(file_name: String) -> Root {
    let path = Path::new(&file_name);
    let mut file_data = String::new();

    let mut rfile = File::open(&path).expect("Unable to open file");
    rfile.read_to_string(&mut file_data).expect("Unable to read file");

    // Deserialize directly into Root
    match serde_json::from_str::<Root>(&file_data) {
        Ok(forest_property_data) => {
            forest_property_data
        },
        Err(e) => {
            panic!("Error parsing JSON data: {}", e);
        }
    }
}

// Choose a parcel 
fn choose_parcel(file_name: String) -> Parcel {
    let root = read_json_file(file_name);
    let parcels: Vec<Parcel> = root.forest_property_data.real_estates.real_estate.parcels.parcel;
    
    println!("\nParcels:");
    for parcel in parcels.iter() {
        print!("{:?}, ", parcel.parcel_number);
    }

    println!("Choose a parcel number to view: ");
    let mut parcel_number = String::new();
    std::io::stdin().read_line(&mut parcel_number).expect("Failed to read line");
    let parcel_number: i64 = parcel_number.trim().parse().expect("Please type a number!");
    let parcel = parcels.iter().find(|&x| x.parcel_number == parcel_number).unwrap();

    parcel.clone()
}

// Choose a stand
fn choose_stand(parcel: Parcel) -> Stand {
    println!("\nStands:");
    for stand in parcel.stands.stand.iter() {
        print!("{:?}, ", stand.stand_basic_data.stand_number);
    }

    println!("Choose a stand number to view: ");
    let mut stand_number = String::new();
    std::io::stdin().read_line(&mut stand_number).expect("Failed to read line");
    let stand_number: i64 = stand_number.trim().parse().expect("Please type a number!");
    let stand = parcel.stands.stand.iter().find(|&x| x.stand_basic_data.stand_number == stand_number).unwrap();

    stand.clone()
}

// Create a polygon from a string of coordinates
fn create_polygon(coord_string: &str) -> Polygon<f64> {
    let coordinates_str: Vec<&str> = coord_string.split(" ").collect();

    // Parse coordinates into a Vec of `Coord<f64>`
    let mut coords: Vec<Coord<f64>> = Vec::new();
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

    let line_string = LineString::new(coords);
    let polygon = Polygon::new(line_string, vec![]);

    polygon
}

fn main() {
    let parcel = choose_parcel("forestpropertydata.json".to_string());
    let stand = choose_stand(parcel);
    
    let coordinate_string = stand.stand_basic_data.polygon_geometry.polygon_property.polygon.exterior.linear_ring.coordinates.trim();
    let polygon = create_polygon(coordinate_string);

    // Generate random points within the polygon
    let random_points = generate_random_points(&polygon, 10);
    println!("random_points within polygon: {:?}", random_points);  

    draw_image(&polygon, random_points);
}

