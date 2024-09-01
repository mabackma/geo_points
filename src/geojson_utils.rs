use crate::{forest_property::{compartment::Compartment, tree::Tree}, geometry_utils::get_min_max_coordinates, requests::fetch_buildings};

use std::{fs::File, io::Write};
use geo::Polygon;
use geojson::{Feature, FeatureCollection, GeoJson, Geometry as GeoJsonGeometry, Value};

// Function to convert a Polygon into a GeoJSON Feature
fn convert_polygon_to_feature(polygon: &Polygon<f64>) -> Feature {
    let exterior_coords: Vec<Vec<f64>> = polygon.exterior().points()
        .map(|point| vec![point.x(), point.y()])
        .collect();

    let geometry = GeoJsonGeometry {
        bbox: None,
        value: Value::Polygon(vec![exterior_coords]),
        foreign_members: None,
    };

    Feature {
        geometry: Some(geometry),
        properties: None,
        id: None,
        bbox: None,
        foreign_members: None,
    }
}

// Function to convert a Tree into a GeoJSON Feature
fn convert_tree_to_feature(tree: &Tree) -> Feature {
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
}

pub async fn save_all_compartments_to_geojson(compartments: Vec<Compartment>, bbox: &Polygon<f64>, filename: &str) {
    let mut all_features = Vec::new();

    for compartment in compartments {
        // Clip the compartment's polygon to the bounding box
        let polygon = match compartment.clip_polygon_to_bounding_box(&bbox) {
            Some(polygon) => polygon,
            None => continue,
        };
        
        // Get the trees within the bounding box
        let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&bbox);
        let trees = compartment.trees_in_bounding_box(min_x, max_x, min_y, max_y);

        // Convert the compartment (polygon) to a GeoJSON feature
        let polygon_feature = convert_polygon_to_feature(&compartment.polygon);
        let tree_features: Vec<Feature> = trees.iter().map(|tree| convert_tree_to_feature(tree)).collect();

        // Add the polygon feature and tree features to the list
        all_features.push(polygon_feature);
        all_features.extend(tree_features);
        
        // Get the buildings within the bounding box
        match fetch_buildings(&bbox).await {
            Ok(geojson) => {
                if let GeoJson::FeatureCollection(buildings) = geojson {
                    all_features.extend(buildings.features);
                }
            },
            Err(e) => {
                eprintln!("Failed to fetch buildings: {}", e);
            }
        }
    }

    // Create the GeoJSON FeatureCollection
    let feature_collection = FeatureCollection {
        features: all_features,
        bbox: None,
        foreign_members: None,
    };

    // Create a GeoJson object
    let geojson = GeoJson::FeatureCollection(feature_collection);

    // Serialize the GeoJson object to a string
    let geojson_string = serde_json::to_string_pretty(&geojson).expect("Failed to serialize GeoJson");

    // Save the GeoJSON string to a file
    let mut file = File::create("stands_in_map.geojson").expect("Failed to create file");
    file.write_all(geojson_string.as_bytes()).expect("Failed to write to file");

    println!("GeoJSON saved to {}", "stands_in_map.geojson");
}

pub fn polygon_to_geojson(polygon: &Polygon<f64>, trees: &Vec<Tree>) -> GeoJson {
    let mut all_features = Vec::new();

    // Convert the compartment (polygon) to a GeoJSON feature
    let polygon_feature = convert_polygon_to_feature(&polygon);
    let tree_features: Vec<Feature> = trees.iter().map(|tree| convert_tree_to_feature(tree)).collect();

    // Add the polygon feature and tree features to the list
    all_features.push(polygon_feature);
    all_features.extend(tree_features);

    // Create the GeoJSON FeatureCollection
    let feature_collection = FeatureCollection {
        features: all_features,
        bbox: None,
        foreign_members: None,
    };

    // Return a GeoJson object
    GeoJson::FeatureCollection(feature_collection)
}