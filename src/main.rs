mod image_utils;
mod data_structures;

use image_utils::*;
use data_structures::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use geo_types::{coord, Coord, LineString, Polygon};
use rand::{thread_rng, Rng};
use geo::Contains;
 
// Read JSON file
fn read_json_file(file_name: String) -> Root {
    let path = Path::new(&file_name);
    let mut rfile = File::open(&path).expect("Unable to open file");
    let mut file_data = String::new();

    // Read file data into the string `file_data`
    rfile.read_to_string(&mut file_data).expect("Unable to read file");

    // Deserialize directly into struct `Root`
    // Root is the top-level struct that contains all the data
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
    let mut parcel_number = String::new();
    
    println!("\nParcels:");
    for parcel in parcels.iter() {
        print!("{:?}, ", parcel.parcel_number);
    }

    println!("Choose a parcel number to view: ");

    // Read parcel number from user input into String `parcel_number`
    std::io::stdin().read_line(&mut parcel_number).expect("Failed to read line");

    // Shadowing `parcel_number` to convert it to an integer
    let parcel_number: i64 = parcel_number.trim().parse().expect("Please type a number!");
    let parcel = parcels.iter().find(|&x| x.parcel_number == parcel_number).unwrap();

    parcel.clone()
}

// Choose a stand
fn choose_stand(parcel: Parcel) -> Stand {
    let mut stand_number = String::new();

    println!("\nStands:");
    for stand in parcel.stands.stand.iter() {
        if stand.tree_stand_data.is_some() {
            print!("{:?}, ", stand.stand_basic_data.stand_number);
        }
    }

    println!("Choose a stand number to view: ");

    // Read stand number from user input into String `stand_number`
    std::io::stdin().read_line(&mut stand_number).expect("Failed to read line");

    // Shadowing `stand_number` to convert it to an integer
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

// Get stem count
fn get_stem_count(tree_stand_data: &TreeStandData) -> i64 {
    let data_date = tree_stand_data.tree_stand_data_date.last().unwrap().clone();
    let stem_count = data_date.tree_stand_summary.unwrap().stem_count;
    
    stem_count
}

// Generates random points within a polygon's minimum and maximum x and y coordinates
pub fn generate_random_points(p: &Polygon, amount: i32) -> Vec<Coord<f64>> {
    let mut points = Vec::new();
    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&p);

    // Generate random x and y coordinates
    let mut count = 0;
    let mut rng = thread_rng();
    loop {
        let rand_x: f64 = rng.gen_range(min_x..max_x);
        let rand_y: f64 = rng.gen_range(min_y..max_y);
        let point = coord! {x: rand_x, y: rand_y};

        if p.contains(&point) {
            points.push(point);
            count += 1;
            if count == amount {
                break;
            }
        }
    }

    points
}

fn main() {
    let exist_stem_counts = stem_counts_in_stratum(0, 5).unwrap();
    println!("Stem counts in all strata: {}", exist_stem_counts);

     /* 
    // Choose a parcel and a stand
    let parcel = choose_parcel("forestpropertydata.json".to_string());
    let stand = choose_stand(parcel);

    // Create a polygon from the stand's coordinates
    let coordinate_string = stand.stand_basic_data.polygon_geometry.polygon_property.polygon.exterior.linear_ring.coordinates.trim();
    let polygon = create_polygon(coordinate_string);

    // Get stem count
    let stem_count = get_stem_count(&stand.tree_stand_data.unwrap());
    println!("\nStem_count: {:?}", stem_count);

    // Generate random points within the polygon
    let random_points = generate_random_points(&polygon, stem_count as i32);

    // Draw the polygon and random points
    draw_image(&polygon, random_points);
    */
}


use serde_json::{Value, from_str};
use std::fs;

fn stem_counts_in_stratum(parcel_index: usize, stand_index: usize) -> Result<bool, Box<dyn std::error::Error>> {
    // Read JSON from file
    let data = fs::read_to_string("forestpropertydata.json")?;
    
    // Parse JSON
    let json_value: Value = from_str(&data)?;

    // Extract the specific parcel and stand
    if let Some(parcel) = json_value
        .get("ForestPropertyData")
        .and_then(|v| v.get("RealEstates"))
        .and_then(|v| v.get("RealEstate"))
        .and_then(|v| v.get("Parcels"))
        .and_then(|v| v.get("Parcel"))
        .and_then(|v| v.as_array())
        .and_then(|v| v.get(parcel_index)) // Use the parcel_index to get the specific parcel
    {
        if let Some(stand) = parcel.get("Stands")
            .and_then(|v| v.get("Stand"))
            .and_then(|v| v.as_array())
            .and_then(|v| v.get(stand_index)) // Use the stand_index to get the specific stand
        {
            // Check TreeStratum in the last TreeStandDataDate entry
            if let Some(tree_stand_data) = stand.get("TreeStandData") {
                if let Some(tree_stand_data_dates) = tree_stand_data.get("TreeStandDataDate").and_then(|v| v.as_array()) {
                    if let Some(last_tree_stand_data_date) = tree_stand_data_dates.last() { // Get the last TreeStandDataDate
                        if let Some(tree_strata) = last_tree_stand_data_date.get("TreeStrata") {
                            if let Some(tree_stratum_array) = tree_strata.get("TreeStratum").and_then(|v| v.as_array()) {
                                for stratum in tree_stratum_array {
                                    if !stratum.get("StemCount").is_some() {
                                        return Ok(false); // StemCount is missing in at least one stratum
                                    }
                                }
                                return Ok(true); // All TreeStratum objects contain a StemCount
                            } else {
                                return Ok(false); // TreeStratum array is missing
                            }
                        } else {
                            return Ok(false); // TreeStrata is missing
                        }
                    } else {
                        return Ok(false); // TreeStandDataDate array is empty
                    }
                } else {
                    return Ok(false); // TreeStandDataDate field not found or not an array
                }
            } else {
                return Ok(false); // TreeStandData field not found
            }
        } else {
            return Ok(false); // Stand not found or not an array
        }
    } else {
        return Ok(false); // Parcel not found or not an array
    }
}

