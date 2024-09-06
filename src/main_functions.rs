mod forest_property;
mod geometry_utils;
mod geojson_utils;
mod jittered_hexagonal_sampling;
mod projection;
mod requests;
mod slippy_tile;

use std::fs::{self, File};
use forest_property::compartment::{find_stands_in_bounding_box, get_compartments_in_bounding_box, Compartment};
use forest_property::forest_property_data::ForestPropertyData;
use forest_property::image_processor::ImageProcessor;
use geo::{coord, BoundingRect, Coord, CoordsIter, Geometry, Intersects, LineString, MultiPolygon, Polygon};
use geojson::GeoJson;
use geometry_utils::{generate_random_trees, get_min_max_coordinates, polygon_to_wgs84};
use geojson_utils::{polygon_to_geojson, save_all_compartments_to_geojson};
use image::{DynamicImage, ImageFormat, Rgb};
use proj4rs::proj;
use projection::{Projection, CRS};
use requests::{fetch_buildings, fetch_buildings_as_polygons, fetch_roads, get_slippy_tile, MmlTile, TileParams};
use serde_json::json;
use std::io::{BufWriter, Write};
use std::time::Instant;
use tokio::runtime::Runtime;
use geo_clipper::Clipper;
use std::error::Error;
use slippy_tile::lon_lat_to_tile_indexes_f32;

/* 
/* DRAWS ENTIRE MAP */
fn main() {
    let start = Instant::now();
    let property = ForestPropertyData::from_xml_file("forestpropertydata.xml");
    let real_estate = property.real_estates.real_estate[0].clone();
    let stands = real_estate.get_stands();
    println!("Total stands: {:?}\n", stands.len());

    // Get the bounding box of the whole map
    let mut bbox = get_bounding_box_of_map();

    // Create a new Tokio runtime
    let rt = Runtime::new().unwrap();

    // Block on the async function using the runtime
    let buildings = rt.block_on(get_buildings(&bbox)).expect("Failed to get buildings");

    // Exclude buildings from the bounding box
    let exclude_buildings = MultiPolygon::new(buildings.clone());
    let excluded = bbox.difference(&exclude_buildings, 100000.0);
    bbox = excluded.0.first().unwrap().to_owned();

    // Find compartments in the bounding box
    let compartments = get_compartments_in_bounding_box(stands, &bbox);
    println!("\nTotal compartments: {:?}", compartments.len());

    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&bbox);

    // Create an image processor with the desired image dimensions
    let img_width = ((max_x - min_x) * 100000.0) as u32;
    let img_height = ((max_y - min_y) * 100000.0) as u32;
    let mut image = ImageProcessor::new(img_width, img_height);

    let scale = ImageProcessor::create_scale(min_x, max_x, min_y, max_y, img_width, img_height);

    for compartment in compartments {
        let polygon = match compartment.clip_polygon_to_bounding_box(&bbox) {
            Some(polygon) => polygon,
            None => continue,
        };
        
        let trees = compartment.trees_in_bounding_box(min_x, max_x, min_y, max_y);

        // Draw the polygon
        let mapped_coordinates = image.map_coordinates_to_image(&polygon, &scale);
        image.draw_polygon_image(&mapped_coordinates, Rgb([0, 0, 255]));

        // Draw the trees
        for tree in trees {
            let point = coord! {x: tree.position().0, y: tree.position().1};
            let color = get_color_by_species(tree.species());
            image.draw_random_point(&scale, img_width, img_height, point, color);
        }
    }

    // Draw the buildings
    for building in buildings.iter() {
        let mapped_building = image.map_coordinates_to_image(&building, &scale);
        image.draw_polygon_image(&mapped_building, Rgb([255, 255, 255]));
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
/* ASKS USER FOR STAND AND DRAWS STAND. SAVES STAND TO GEOJSON */
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
*/
/* 
/* GETS ROADS IN MAP BOUNDING BOX AND DRAWS PNG */
fn main() {
    let start = Instant::now();

    // Get the bounding box of the whole map
    let mut bbox = get_bounding_box_of_map();

    // Top left corner of bounding box
    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&bbox);
    println!("lon_min: {:?}, lat_max: {:?}", min_x, max_y);
    
    let zoom = 12;

    // Convert the top left corner of bounding box to slippy tile indices
    let (x_index_tl, y_index_tl) = lon_lat_to_tile_indexes_f32(
        min_x as f32,
        max_y as f32,
        zoom,
    );
    println!("Top left corner: {:?}, {:?}", x_index_tl, y_index_tl);

    // Convert the bottom right corner of bounding box to slippy tile indices
    let (x_index_br, y_index_br) = lon_lat_to_tile_indexes_f32(
        max_x as f32,
        min_y as f32,
        zoom,
    );
    println!("Bottom right corner: {:?}, {:?}", x_index_br, y_index_br);

    // Count the number of tiles in the bounding box
    let x_tiles = x_index_br - x_index_tl + 1;
    let y_tiles = y_index_br - y_index_tl + 1;
    let total_tiles = x_tiles * y_tiles;
    println!("Total tiles: {:?}", total_tiles);

    let tile_params = TileParams::new(
        "png",
        zoom,
        x_index_tl,
        y_index_tl,
        0.0,
        0.0,
    );

    // Create a new Tokio runtime
    let rt = Runtime::new().unwrap();

    // Block on the async function using the runtime
    let road_image = rt.block_on(get_slippy_tile(tile_params, MmlTile::Roads.as_str())).expect("Failed to get roads");

    // Save the image
    save_image_to_file(&road_image, "road_image.png").expect("Failed to save image");

    let duration = start.elapsed();
    println!("Time elapsed in create_all_compartments is: {:?}", duration);
}

fn save_image_to_file(img: &DynamicImage, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(file_path)?;
    let ref mut w = BufWriter::new(file);
    img.write_to(w, ImageFormat::Png)?;
    Ok(())
}
*/