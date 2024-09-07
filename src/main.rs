mod forest_property;
mod geometry_utils;
mod geojson_utils;
mod jittered_hexagonal_sampling;
mod projection;
mod requests;
mod slippy_tile;

use forest_property::compartment::get_compartments_in_bounding_box;
use forest_property::forest_property_data::ForestPropertyData;
use geo::{Coord, LineString, MultiPolygon, Polygon};
use geojson::{Error as GeoJsonError, GeoJson};
use geometry_utils::{get_min_max_coordinates, polygon_to_wgs84};
use geojson_utils::{save_all_compartments_to_geojson, save_geojson};
use image::Rgb;
use projection::{Projection, CRS};
use requests::{fetch_buildings, fetch_buildings_as_polygons, fetch_roads, FetchError};
use std::time::Instant;
use tokio::runtime::Runtime;
use geo_clipper::Clipper;
use std::error::Error;


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

/* SAVES ENTIRE MAP TO GEOJSON FILES */
// Fetches buildings and roads as GeoJSON data
// Creates compartments in the bounding box. Compartments exclude buildings while generating trees.
// Saves all compartments, buildings, and roads to a GeoJSON file
// TODO: Exclude roads while generating trees!
fn main() {
    let start = Instant::now();
    let property = ForestPropertyData::from_xml_file("forestpropertydata.xml");
    let real_estate = property.real_estates.real_estate[0].clone();
    let stands = real_estate.get_stands();
    println!("Total stands: {:?}", stands.len());

    // Get the bounding box of the whole map
    let mut bbox = get_bounding_box_of_map();

    // Create a new Tokio runtime
    let rt = Runtime::new().unwrap();

    // Block on the async function using the runtime
    // Get buildings as polygons
    let buildings = rt.block_on(fetch_buildings_as_polygons(&bbox)).expect("Failed to get buildings");
    println!("Fetched buildings: {}", buildings.len());

    // Get GeoJson data
    let buildings_geojson = rt.block_on(fetch_buildings(&bbox)).expect("Failed to get buildings");
    let roads_geojson = rt.block_on(fetch_roads(&bbox)).expect("Failed to get roads");

    match roads_geojson {
        GeoJson::FeatureCollection(ref collection) => {
            println!("Fetched roads: {}\n", collection.features.len());
        }
        _ => {
            println!("The GeoJson for roads does not contain a FeatureCollection");
        }
    }

    // Exclude buildings from the bounding box
    let exclude_buildings = MultiPolygon::new(buildings);
    let excluded = bbox.difference(&exclude_buildings, 100000.0);
    bbox = excluded.0.first().unwrap().to_owned();

    // Create compartments in the bounding box
    let compartments = get_compartments_in_bounding_box(stands, &bbox);
    println!("\nTotal compartments: {:?}", compartments.len());

    // Save all compartments and trees to a GeoJSON file
    save_all_compartments_to_geojson(compartments, &bbox, &buildings_geojson, &roads_geojson, "stands_in_map.geojson");

    let duration = start.elapsed();
    println!("Time elapsed in create_all_compartments is: {:?}", duration);
}
