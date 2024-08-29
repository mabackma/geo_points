mod forest_property;
mod geometry_utils;
mod jittered_hexagonal_sampling;
mod projection;

use std::fs::{self, File};
use forest_property::compartment::{find_stands_in_bounding_box, get_compartments_in_bounding_box, Compartment};
use forest_property::forest_property_data::ForestPropertyData;
use forest_property::image_processor::ImageProcessor;
use forest_property::tree::Tree;
use geo::{coord, Coord, Intersects, LineString, Polygon, Geometry};
use geometry_utils::{generate_random_trees, get_min_max_coordinates};
use geojson::{Feature, FeatureCollection, GeoJson, Geometry as GeoJsonGeometry, Value};
use image::Rgb;
use serde_json::json;

#[cfg(test)]
use std::fs;
use std::io::{self, Write};
use std::time::Instant;

// Get color based on species number
fn get_color_by_species(number: u8) -> Rgb<u8> {
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
        _ => Rgb([0, 0, 0]), // Black for Unknown
    }
}

// Get the bounding box of the whole map
fn get_bounding_box_of_map() -> Polygon<f64> {
    let property = ForestPropertyData::from_xml_file("forestpropertydata.xml");
    let mut all_stands = property.real_estates.real_estate[0].get_stands();

    let mut min_x = f64::MAX;
    let mut max_x = f64::MIN;
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;

    for stand in all_stands.iter_mut() {
        let polygon = stand.computed_polygon.to_owned().unwrap();
        let (p_min_x, p_max_x, p_min_y, p_max_y) = get_min_max_coordinates(&polygon);

        if p_min_x < min_x {
            min_x = p_min_x;
        }
        if p_max_x > max_x {
            max_x = p_max_x;
        }
        if p_min_y < min_y {
            min_y = p_min_y;
        }
        if p_max_y > max_y {
            max_y = p_max_y;
        }
    }
    
    let bbox = geo::Polygon::new(
        LineString(vec![
            Coord { x: min_x, y: min_y },
            Coord { x: max_x, y: min_y },
            Coord { x: max_x, y: max_y },
            Coord { x: min_x, y: max_y },
            Coord { x: min_x, y: min_y },
        ]),
        vec![],
    );

    bbox
}
/* 
/* DRAWS ENTIRE MAP */
fn main() {
    let start = Instant::now();
    let property = ForestPropertyData::from_xml_file("forestpropertydata.xml");
    let real_estate = property.real_estates.real_estate[0].clone();
    let stands = real_estate.get_stands();
    println!("Total stands: {:?}\n", stands.len());

    // Get the bounding box of the whole map
    let bbox = get_bounding_box_of_map();

    // Find compartments in the bounding box
    let compartments = get_compartments_in_bounding_box(stands, &bbox);
    println!("\nTotal compartments: {:?}", compartments.len());

    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&bbox);

    // Create an image processor with the desired image dimensions
    let img_width = ((max_x - min_x) * 100000.0) as u32;
    let img_height = ((max_y - min_y) * 100000.0) as u32;
    let mut image = ImageProcessor::new(img_width, img_height);

    let aspect_ratio_image = img_width as f64 / img_height as f64;
    let aspect_ratio_bbox = (max_x - min_x) / (max_y - min_y);
    
    println!("Aspect ratio of image: {}", aspect_ratio_image);
    println!("Aspect ratio of bounding box: {}", aspect_ratio_bbox);
    println!("Image dimensions: {} x {}", img_width, img_height);

    let scale = ImageProcessor::create_scale(min_x, max_x, min_y, max_y, img_width, img_height);

    for compartment in compartments {
        let polygon = match compartment.clip_polygon_to_bounding_box(&bbox) {
            Some(polygon) => polygon,
            None => continue,
        };
        
        let trees = compartment.trees_in_bounding_box(min_x, max_x, min_y, max_y);

        // Draw the polygon
        let mapped_coordinates = image.map_coordinates_to_image(&polygon, &scale);
        image.draw_polygon_image(&mapped_coordinates);

        // Draw the trees
        for tree in trees {
            let point = coord! {x: tree.position().0, y: tree.position().1};
            let color = get_color_by_species(tree.species());
            image.draw_random_point(&scale, img_width, img_height, point, color);
        }
    }

    image
        .img()
        .save("clipped_image.png")
        .expect("Failed to save image");

    let duration = start.elapsed();
    println!("Time elapsed in create_all_compartments is: {:?}", duration);
}
*/
/* 
pienempi
N=7369787.000, E=427754.979
N=7369564.333, E=427997.035
N kasvaa pohjoisen suuntaa, E kasvaa idän suuntaan

--->

let min_x: f64 = 427754.979;
let max_x: f64 = 427997.035;
let max_y: f64 = 7369787.000;
let min_y: f64 = 7369564.333;


isompi
N=7369959.526, E=427541.481
N=7369356.859, E=428282.985


let min_x: f64 = 427541.481;
let max_x: f64 = 428282.985;
let max_y: f64 = 7369959.526;
let min_y: f64 = 7369564.333;
*/

#[test]
fn test_writing_to_json() {
    let test_json_path = "test_json_from_xml.json";

    let xml_property = ForestPropertyData::from_xml_file("forestpropertydata.xml");

    match xml_property.write_to_json_file(&test_json_path) {
        std::result::Result::Ok(_) => assert!(true),
        _ => assert!(false),
    }

    fs::remove_file(test_json_path).unwrap()
}

#[test]
fn test_parsers() {
    let xml_property = ForestPropertyData::from_xml_file("forestpropertydata.xml");
    xml_property
        .write_to_json_file("json_from_xml.json")
        .expect("writing JSON failed");

    let json_property = ForestPropertyData::from_json_file("json_from_xml.json");

    let xml_real_estate = xml_property.real_estates.real_estate.first().unwrap();
    let json_real_estate = json_property.real_estates.real_estate.first().unwrap();

    let xml_id = &xml_real_estate.id;
    let json_id = &json_real_estate.id;

    assert!(xml_id == json_id, "JSON and XML file parsing");

    let xml_stands = xml_real_estate.get_stands();
    let json_stands = json_real_estate.get_stands();

    for i in 0..xml_stands.iter().len() {
        assert!(
            xml_stands[i].id == json_stands[i].id,
            "stand is matches with id: {}",
            i
        )
    }
}

// Run wih `cargo test -- --nocapture` to see the print statements
#[test]
fn test_find_stands_in_bounding_box() {
    let property = ForestPropertyData::from_xml_file("forestpropertydata.xml");
    let real_estate = property.real_estates.real_estate[0].clone();
    let all_stands = real_estate.get_stands();

    let mut stands = Vec::new();
    for stand in all_stands {
        stands.push(stand.clone());
    }
    println!("\nTotal stands: {:?}", stands.len());
    let min_x = 425400.0;
    let max_x = min_x + 6000.0;
    let min_y = 7369000.0;
    let max_y = min_y + 6000.0;

    let bbox = geo::Polygon::new(
        LineString(vec![
            Coord { x: min_x, y: min_y },
            Coord { x: max_x, y: min_y },
            Coord { x: max_x, y: max_y },
            Coord { x: min_x, y: max_y },
            Coord { x: min_x, y: min_y },
        ]),
        vec![],
    );
    let stands = find_stands_in_bounding_box(&stands, &bbox);
    /*     if !stands.is_none() {
        println!(
            "Stands in bounding box: {:?}",
            stands.clone().unwrap().len()
        );
        for stand in &stands.unwrap() {
            println!("Stand number {:?}", stand.stand_basic_data.stand_number);
        }
    } */
}
 
/* ASKS USER FOR STAND AND DRAWS STAND */
fn main() {
    let property = ForestPropertyData::from_xml_file("forestpropertydata.xml");
    let mut stand = property.get_stand_cli();
    let polygon = stand.create_polygon();

    // Create an image for the polygon and random points
    let img_width = 800;
    let img_height = 600;
    let mut image = ImageProcessor::new(img_width, img_height);

    // Get the minimum and maximum x and y coordinates of the polygon
    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&polygon);
    let scale = ImageProcessor::create_scale(min_x, max_x, min_y, max_y, img_width, img_height);

    // Map polygon coordinates to image
    let mapped_coordinates = image.map_coordinates_to_image(&polygon, &scale);
    image.draw_polygon_image(&mapped_coordinates);

    let summary_stem_count = stand.summary_stem_count();
    let strata = stand.get_strata().expect("No treeStrata/stratums found");
    let random_trees = generate_random_trees(&polygon, &strata);

    // Convert the Polygon and the trees to GeoJSON
    let geojson = polygon_to_geojson(&polygon, &random_trees);

    // Serialize GeoJson to a String
    let geojson_string = serde_json::to_string_pretty(&geojson).expect("Failed to serialize GeoJson");
    
    // Write GeoJson to a file
    let mut file = File::create("single_stand.geojson").expect("Failed to create file");
    file.write_all(geojson_string.as_bytes()).expect("Failed to write to file");

    // Read the GeoJSON file contents back into a string
    let file_geojson_string = fs::read_to_string("single_stand.geojson")
        .expect("Failed to read file");

    println!("Read GeoJSON string from file:\n{}", file_geojson_string);
    
    if stand.stem_count_in_stratum() {
        println!("\nStem count is in individual stratum");

        // Draw the random points
        for tree in random_trees {
            let point = coord! {x: tree.position().0, y: tree.position().1};
            let color = get_color_by_species(tree.species());
            image.draw_random_point(&scale, img_width, img_height, point, color);
        }
    } else {
        println!("Stem count is not in any individual stratum. Drawing random points based on tree stand summary.");

        // Draw the random points
        for tree in random_trees {
            let point = coord! {x: tree.position().0, y: tree.position().1};
            image.draw_random_point(&scale, img_width, img_height, point, Rgb([255, 0, 0])) // Draw points in red
        }
    }
    println!("\nTotal stem count: {:?}", summary_stem_count);

    image
        .img()
        .save("polygon_image.png")
        .expect("Failed to save image");
    println!("Polygon image saved as 'polygon_image.png'");
}

fn polygon_to_geojson(polygon: &Polygon<f64>, trees: &Vec<Tree>) -> GeoJson {
    // Convert the Polygon to GeoJSON coordinates
    let exterior_coords: Vec<Vec<f64>> = polygon.exterior().points()
        .map(|point| vec![point.x(), point.y()])
        .collect();

    // Create the GeoJSON Polygon Geometry
    let polygon_geometry = GeoJsonGeometry {
        bbox: None,
        value: Value::Polygon(vec![exterior_coords]),
        foreign_members: None,
    };

    // Create a GeoJSON Feature for the Polygon
    let polygon_feature = Feature {
        geometry: Some(polygon_geometry),
        properties: None, // or Some(some_properties) if you have properties
        id: None,
        bbox: None,
        foreign_members: None,
    };

    // Create GeoJSON Features for each tree
    let tree_features: Vec<Feature> = trees.iter().map(|tree| {
        let point = vec![tree.position().0, tree.position().1];
        let point_geometry = GeoJsonGeometry {
            bbox: None,
            value: Value::Point(point),
            foreign_members: None,
        };

        let mut properties = serde_json::Map::new();
        properties.insert("species".to_string(), serde_json::json!(tree.species()));

        Feature {
            geometry: Some(point_geometry),
            properties: Some(properties),
            id: None,
            bbox: None,
            foreign_members: None,
        }
    }).collect();

    // Combine polygon and tree features
    let mut features = vec![polygon_feature];
    features.extend(tree_features);

    // Create a GeoJSON FeatureCollection
    let feature_collection = FeatureCollection {
        features,
        bbox: None,
        foreign_members: None,
    };

    // Create a GeoJson object
    GeoJson::FeatureCollection(feature_collection)
}