mod image_utils;
mod forest_property;
use crate::forest_property::real_estate::Root;
use crate::forest_property::parcel::Parcel;
use crate::forest_property::stand::Stand;
use crate::forest_property::tree_stand_data::TreeStandData;

use image::{Rgb, RgbImage};
use image_utils::*;
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

// Get stem count
fn get_stem_count(tree_stand_data: &TreeStandData) -> i64 {
    let data_date = tree_stand_data.tree_stand_data_date.last().unwrap().clone();
    let stem_count = data_date.tree_stand_summary.unwrap().stem_count;
    
    stem_count
}

// Determines if stem count is in individual stratum
fn stem_count_in_stratum(stand: &Stand) -> bool {
    if let Some(tree_stand_data) = &stand.tree_stand_data {
        let data_date = tree_stand_data.tree_stand_data_date.last().unwrap().clone();
        for stratum in data_date.tree_strata.tree_stratum.iter() {
            if stratum.stem_count.is_some() {
                return true;
            }
        }
    }

    false
}

// Returns a vector of tuples containing species and amount of trees in a stratum
fn get_stratum_info(stand: &Stand) -> Vec<(i64, i64)> {
    let mut info = Vec::new();
    
    let tree_stand_data = stand.tree_stand_data.as_ref().unwrap();
    let data_date = tree_stand_data.tree_stand_data_date.last().unwrap().clone();

    for stratum in data_date.tree_strata.tree_stratum.iter() {
        let species = stratum.tree_species.clone();

        if let Some(amount) = stratum.stem_count {
            info.push((species, amount));
        } else {
            info.push((species, 0));
        }
    }

    info
}

fn get_color_by_species(number: i64) -> Rgb<u8> {
    match number {
        // Coniferous Trees (Shades of Orange and Red)
        1 => Rgb([255, 165, 0]),    // Orange - Mänty
        2 => Rgb([255, 0, 0]),      // Red - Kuusi
        10 => Rgb([255, 140, 0]),   // DarkOrange - Douglaskuusi
        11 => Rgb([255, 99, 71]),   // Tomato - Kataja
        12 => Rgb([255, 127, 80]),  // Coral - Kontortamänty
        16 => Rgb([178, 34, 34]),   // Firebrick - Mustakuusi
        19 => Rgb([205, 92, 92]),   // IndianRed - Pihta
        22 => Rgb([139, 0, 0]),     // DarkRed - Sembramänty
        23 => Rgb([233, 150, 122]), // DarkSalmon - Serbiankuusi
        30 => Rgb([250, 128, 114]), // Salmon - Havupuu

        // Deciduous Trees (Shades of Green and Blue)
        3 => Rgb([50, 205, 50]),    // LimeGreen - Rauduskoivu
        4 => Rgb([34, 139, 34]),    // ForestGreen - Hieskoivu
        5 => Rgb([107, 142, 35]),   // OliveDrab - Haapa
        6 => Rgb([143, 188, 143]),  // DarkSeaGreen - Harmaaleppä
        7 => Rgb([46, 139, 87]),    // SeaGreen - Tervaleppä
        9 => Rgb([32, 178, 170]),   // LightSeaGreen - Muu lehtipuu
        13 => Rgb([0, 128, 128]),   // Teal - Kynäjalava
        14 => Rgb([102, 205, 170]), // MediumAquamarine - Lehtikuusi
        15 => Rgb([60, 179, 113]),  // MediumSeaGreen - Metsälehmus
        17 => Rgb([152, 251, 152]), // PaleGreen - Paju
        18 => Rgb([0, 255, 127]),   // SpringGreen - Pihlaja
        20 => Rgb([0, 250, 154]),   // MediumSpringGreen - Raita
        21 => Rgb([144, 238, 144]), // LightGreen - Saarni
        24 => Rgb([85, 107, 47]),   // DarkOliveGreen - Tammi
        25 => Rgb([154, 205, 50]),  // YellowGreen - Tuomi
        26 => Rgb([0, 255, 0]),     // Lime - Vaahtera
        27 => Rgb([173, 216, 230]), // LightBlue - Visakoivu
        28 => Rgb([72, 209, 204]),  // MediumTurquoise - Vuorijalava
        29 => Rgb([64, 224, 208]),  // Turquoise - Lehtipuu

        // Default case for any unknown tree number
        _ => Rgb([0, 0, 0]),        // Black for Unknown
    }
}

fn main() {
    // Choose a parcel and a stand
    let parcel = choose_parcel("forestpropertydata_updated.json".to_string());
    let stand = choose_stand(parcel);

    // Create a polygon from the stand's coordinates
    let coordinate_string = stand.stand_basic_data.polygon_geometry.polygon_property.polygon.exterior.linear_ring.coordinates.trim();
    let polygon = create_polygon(coordinate_string);

    // Create an image for the polygon and random points
    let img_width = 800;
    let img_height = 600;
    let mut image = RgbImage::new(img_width, img_height);

    // Map polygon coordinates to image
    let mapped_coordinates = map_coordinates_to_image(&polygon, img_width, img_height);

    // Draw the polygon
    draw_polygon_image(&mut image, mapped_coordinates.clone());

    if stem_count_in_stratum(&stand) {
        println!("\nStem count is in individual stratum");

        let stratum_info = get_stratum_info(&stand);

        for (species, amount) in stratum_info {
            println!("Species: {:?}, Amount: {:?}", species, amount);
            
            // Draw random points with different colors based on species
            let color = get_color_by_species(species);
            let random_points = generate_random_points(&polygon, amount as i32);
            for point in random_points {
                draw_random_point(&mut image, &polygon, img_width, img_height, point, color);
            }
        }
    } else {
        println!("Stem count is not in stratum");
        
        // Get stem count from tree stand summary
        let stem_count = get_stem_count(&stand.tree_stand_data.unwrap());
        println!("\nStem_count: {:?}", stem_count);

        // Generate random points within the polygon
        let random_points = generate_random_points(&polygon, stem_count as i32);

        // Draw the generated random points within the polygon
        for point in random_points {
            draw_random_point(&mut image, &polygon, img_width, img_height, point, Rgb([255, 0, 0])) // Draw points in red
        }
    }

    image.save("polygon_image.png").expect("Failed to save image");
    println!("Polygon image saved as 'polygon_image.png'");
}

