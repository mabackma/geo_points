mod forest_property;
mod geometry_utils;
use forest_property::{forest_property_data::ForestPropertyData, image_processor::ImageProcessor, stand, compartment::Compartment};
use geo_types::coord;
use geometry_utils::generate_random_trees;
use image::Rgb;
use crate::forest_property::compartment::find_stands_in_bounding_box;

#[cfg(test)]
use std::fs;
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
        _ => Rgb([0, 0, 0]), // Black for Unknown
    }
}


 
fn main() { 
    let property = ForestPropertyData::from_xml_file("forestpropertydata.xml");
    let stand = property.get_stand_cli();
    let polygon = stand.create_polygon();

    // Create an image for the polygon and random points
    let img_width = 800;
    let img_height = 600;
    let mut image = ImageProcessor::new(img_width, img_height);

    // Map polygon coordinates to image
    let mapped_coordinates = image.map_coordinates_to_image(&polygon);
    image.draw_polygon_image(&mapped_coordinates);

    let summary_stem_count = stand.summary_stem_count();
    let strata = stand.get_strata().expect("No treeStrata/stratums found");
    let random_trees = generate_random_trees(&polygon, &strata);

    if stand.stem_count_in_stratum() {
        println!("\nStem count is in individual stratum");

        // Draw the random points
        for tree in random_trees {
            let point = coord! {x: tree.position().0, y: tree.position().1};
            let color = get_color_by_species(tree.species());
            image.draw_random_point(&polygon, img_width, img_height, point, color);
        }
    } else {
        println!("Stem count is not in any individual stratum. Drawing random points based on tree stand summary.");

        // Draw the random points
        for tree in random_trees {
            let point = coord! {x: tree.position().0, y: tree.position().1};
            image.draw_random_point(&polygon, img_width, img_height, point, Rgb([255, 0, 0])) // Draw points in red
        }
    }
    println!("\nTotal stem count: {:?}", summary_stem_count);

    image
        .img()
        .save("polygon_image.png")
        .expect("Failed to save image");
    println!("Polygon image saved as 'polygon_image.png'");
}

#[test]
fn test_writing_to_json(){
    let test_json_path = "test_json_from_xml.json";

    let xml_property = ForestPropertyData::from_xml_file("forestpropertydata.xml");

    match xml_property.write_to_json_file("test_json_from_xml.json") {
        std::result::Result::Ok(_) => assert!(true),
        _ => assert!(false)
    }
    
    fs::remove_file(test_json_path).unwrap()

}

#[test]
fn test_parsers() {
    let xml_property = ForestPropertyData::from_xml_file("forestpropertydata.xml");
    xml_property.write_to_json_file("json_from_xml.json").expect("writing JSON failed");

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
    let stands = find_stands_in_bounding_box(&stands, 0.0, 428000.0, 0.0, 7371000.0);
    if !stands.is_none() {
        println!("Stands in bounding box: {:?}", stands.clone().unwrap().len());
        for stand in &stands.unwrap() {
            println!("Stand number {:?}", stand.stand_basic_data.stand_number);
        }
    }
}
/* TESTING TREE GENERATION FOR STANDS IN BOUNDING BOX
fn main() {
    let property = ForestPropertyData::from_xml_file("forestpropertydata.xml");
    let real_estate = property.real_estates.real_estate[0].clone();
    let all_stands = real_estate.get_stands();

    let mut compartments = Vec::new();
    let mut stands = Vec::new();
    for stand in all_stands {
        stands.push(stand.clone());
    }
    println!("\nTotal stands: {:?}", stands.len());

    // Find stands in the bounding box
    let stands = find_stands_in_bounding_box(&stands, 0.0, 428000.0, 0.0, 7371000.0);

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
}
*/