use geo_points::forest_property::forest_property_data::ForestPropertyData;
use geo_points::main_functions::{
    create_geo_json_from_coords, 
    draw_and_save_selected_stand, 
    draw_stands_in_bbox, 
    get_bounding_box_of_map, 
    random_bbox, 
    save_geojson
};
use geo_points::geometry_utils::get_min_max_coordinates;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let mut bbox = get_bounding_box_of_map();
    bbox = random_bbox(&bbox);

    let(min_x, max_x, min_y, max_y) = get_min_max_coordinates(&bbox);
    let property = ForestPropertyData::from_xml_file("forestpropertydata.xml");

    match create_geo_json_from_coords(min_x, max_x, min_y, max_y, &property) {
        Ok(geojson) => {
            let filename = "stands_in_bbox.geojson";
            save_geojson(&geojson, filename);
            println!("GeoJson data successfully saved to {}", filename);
        }
        Err(e) => {
            eprintln!("Failed to create GeoJson data: {}", e);
            return Err(e); 
        }
    }

    println!("------------------------------------------------------------");
    match draw_stands_in_bbox(&mut bbox, &property) {
        Ok(image) => {
            image
                .img()
                .save("stands_in_bbox_image.png")
                .expect("Failed to save image");
            println!("Image saved as 'stands_in_bbox_image.png'");
        },
        Err(e) => {
            eprintln!("Failed to save selected stand: {}", e);
            return Err(e);
        }
    }

    println!("------------------------------------------------------------");
    draw_and_save_selected_stand(&property, "selected_stand_image.png");

    Ok(())
}
