use geo_points::main_functions::{
    create_geo_json_from_coords, 
    draw_and_save_selected_stand, 
    draw_stands_in_bbox, get_bounding_box_of_map, 
    random_bbox, 
    save_geojson
};
use geo_points::geometry_utils::get_min_max_coordinates;

fn main() {
    let mut bbox = get_bounding_box_of_map();
    bbox = random_bbox(&bbox);

    let(min_x, max_x, min_y, max_y) = get_min_max_coordinates(&bbox);

    let map_geojson = create_geo_json_from_coords(min_x, max_x, min_y, max_y);
    let filename = "stands_in_bbox.geojson";
    save_geojson(&map_geojson, filename);

    println!("------------------------------------------------------------");
    draw_stands_in_bbox(&mut bbox);

    println!("------------------------------------------------------------");
    draw_and_save_selected_stand();
}
