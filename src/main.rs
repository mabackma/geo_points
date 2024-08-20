mod geometry_utils;
mod forest_property;

use crate::forest_property::root::Root;
use crate::forest_property::image_processor::ImageProcessor;
use forest_property::forest_property_data::ForestPropertyData;
use forest_property::tree_stand_data::TreeStrata;
use geo_types::coord;
use geometry_utils::*;
use image::Rgb;
use serde::{Deserialize, Serialize};
use serde_json::Serializer;
use serde_xml_rs::{Deserializer, EventReader, ParserConfig};
use std::fs::{read_to_string, File};
use std::io::{Read, Write};
use std::path::Path;
 
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

// Get color based on species number
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

    let property = ForestPropertyData::from_xml_file("forestpropertydata.xml");
    let real_estate = property.choose_real_estate();
    let parcel = real_estate.choose_parcel();
    let stand = parcel.choose_stand();
    let coordinate_strings: Vec<String> = stand.get_coordinate_string();

    // !! Some modifications for a quick test !!

    // Choose a parcel and a stand
/*  let file_name = "forestpropertydata_updated.json".to_string();
    let root = read_json_file(file_name);
    let parcel = root.choose_parcel();
    let stand = parcel.choose_stand();

    // Create a polygon from the stand's coordinates
    let coordinate_string = stand.stand_basic_data.polygon_geometry.polygon_property.polygon.exterior.linear_ring.coordinates.trim(); */

    let coordinate_string = coordinate_strings.first().expect("No coordinates found");

    let polygon = create_polygon(&coordinate_string.as_str());

    // Create an image for the polygon and random points
    let img_width = 800;
    let img_height = 600;
    let mut image = ImageProcessor::new(img_width, img_height);

    // Map polygon coordinates to image
    let mapped_coordinates = image.map_coordinates_to_image(&polygon);

    // Draw the polygon
    image.draw_polygon_image(&mapped_coordinates);

    let summary_stem_count = stand.get_stem_count();
    if stand.stem_count_in_stratum() {
        println!("\nStem count is in individual stratum");

        // Replaced for a quick test to try out if this is working
        let stratums = stand.get_stratums();
        // Modified generate_random_trees function to take Vec<Stratum>
        let random_trees = generate_random_trees(&polygon, stratums);

        // Draw random points without using Poisson disc sampling
        for tree in random_trees {
            let point = coord! {x: tree.position().0, y: tree.position().1};
            let color = get_color_by_species(tree.species());
            image.draw_random_point(&polygon, img_width, img_height, point, color);
        }
        // TODO: Implement Poisson disc sampling for better random point distribution
    } else {
        println!("Stem count is not in any individual stratum. Drawing random points based on tree stand summary.");

        // Generate random points within the polygon
        let random_points = generate_random_points(&polygon, summary_stem_count as i32);

        // Draw the generated random points within the polygon
        for point in random_points {
            image.draw_random_point(&polygon, img_width, img_height, point, Rgb([255, 0, 0])) // Draw points in red
        }
    }
    println!("\nTotal stem count: {:?}", summary_stem_count);

    image.img().save("polygon_image.png").expect("Failed to save image");
    println!("Polygon image saved as 'polygon_image.png'");
}

