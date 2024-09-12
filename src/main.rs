use geo_points::main_functions::{
    get_bounding_box_of_map, 
    random_bbox, 
    create_geo_json_for_bbox, 
    save_geojson, 
    draw_stands_in_bbox, 
    draw_and_save_selected_stand
};

fn main() {
    let mut bbox = get_bounding_box_of_map();
    bbox = random_bbox(&bbox);

    let map_geojson = create_geo_json_for_bbox(&mut bbox);
    let filename = "stands_in_bbox.geojson";
    save_geojson(&map_geojson, filename);

    println!("------------------------------------------------------------");
    draw_stands_in_bbox(&mut bbox);

    println!("------------------------------------------------------------");
    draw_and_save_selected_stand();
}
