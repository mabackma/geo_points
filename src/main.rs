mod forest_property;
mod geometry_utils;
mod geojson_utils;
mod jittered_hexagonal_sampling;
mod projection;
mod requests;
mod slippy_tile;
mod main_functions;

use forest_property::forest_property_data::ForestPropertyData;
use geo::{Coord, LineString,  Polygon};
use geometry_utils::get_min_max_coordinates;
use geojson_utils::save_geojson;
use main_functions::{create_geo_json_for_bbox, draw_and_save_selected_stand, draw_stands_in_bbox};

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

fn main() {
    let mut bbox = get_bounding_box_of_map();
 
    let map_geojson = create_geo_json_for_bbox(&mut bbox);
    let filename = "stands_in_bbox.geojson";
    save_geojson(&map_geojson, filename);

    println!("------------------------------------------------------------");
    draw_stands_in_bbox(&mut bbox);

    println!("------------------------------------------------------------");
    draw_and_save_selected_stand();
}
