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
use geojson::GeoJson;
use geometry_utils::get_min_max_coordinates;
use geojson_utils::save_all_compartments_to_geojson;
use requests::{fetch_buildings, fetch_buildings_as_polygons, fetch_roads};
use std::time::Instant;
use tokio::runtime::Runtime;
use geo_clipper::Clipper;


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
