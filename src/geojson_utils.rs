use crate::{forest_property::{compartment::Compartment, tree::Tree}, geometry_utils::get_min_max_coordinates, requests::fetch_buildings};

use std::{fs::File, io::Write};
use geo::{Coord, LineString, MultiPolygon, Polygon};
use geo_types::{LineString as GeoLineString, Polygon as GeoPolygon};
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

pub fn save_all_compartments_to_geojson(
        compartments: Vec<Compartment>, 
        bbox: &Polygon<f64>, 
        buildings: &GeoJson, 
        roads: &GeoJson,
        filename: &str) {
        
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
    }

    // Add building features to the list, ensuring the GeoJson is a FeatureCollection
    if let GeoJson::FeatureCollection(building_collection) = buildings {
        println!("Added buildings to geojson: {}", building_collection.features.len());
        for building_feature in &building_collection.features {
            all_features.push(building_feature.clone());
        }
    } else {
        println!("Buildings GeoJson is not a FeatureCollection");
    }

    // Add road features to the list, ensuring the GeoJson is a FeatureCollection
    if let GeoJson::FeatureCollection(road_collection) = roads {
        println!("Added roads to geojson: {}", road_collection.features.len());
        for road_feature in &road_collection.features {
            all_features.push(road_feature.clone());
        }
    } else {
        println!("Roads GeoJson is not a FeatureCollection");
    }

    // Create the GeoJSON FeatureCollection
    let feature_collection = FeatureCollection {
        features: all_features,
        bbox: None,
        foreign_members: None,
    };

    // Create a GeoJson object
    let geojson = GeoJson::FeatureCollection(feature_collection);

    save_geojson(&geojson, filename);
}

// Function to save a GeoJson object to a file
pub fn save_geojson(geojson: &GeoJson, filename: &str) {
    // Serialize the GeoJson object to a string
    let geojson_string = serde_json::to_string_pretty(&geojson).expect("Failed to serialize GeoJson");

    // Save the GeoJSON string to a file
    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(geojson_string.as_bytes()).expect("Failed to write to file");

    println!("\nGeoJSON saved to {}", filename);
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

// Convert meters to degrees for latitude
fn meters_to_degrees_latitude(meters: f64) -> f64 {
    meters / 111_000.0
}

// Convert meters to degrees for longitude based on latitude
fn meters_to_degrees_longitude(meters: f64, latitude: f64) -> f64 {
    meters / (111_000.0 * latitude.to_radians().cos())
}

// Calculate the perpendicular offset vector
fn perpendicular_offset(vector: (f64, f64), offset: f64) -> (f64, f64) {
    let (x, y) = vector;
    let length = (x * x + y * y).sqrt();
    let (x, y) = (x / length, y / length);
    (-y * offset, x * offset)
}

// Create a polygon from LineString with a given width
fn line_to_polygon_with_width(line: &LineString<f64>, width: f64) -> GeoPolygon<f64> {
    let mut left_points = Vec::new();
    let mut right_points = Vec::new();
    
    for i in 0..line.0.len() - 1 {
        let p1 = line.0[i];
        let p2 = line.0[i + 1];
        
        // Calculate width in degrees based on latitude
        let width_lat = meters_to_degrees_latitude(width);
        let width_lon = meters_to_degrees_longitude(width, p1.y);
        
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        
        let offset = perpendicular_offset((dx, dy), width_lat / 2.0);
        let left1 = (p1.x + offset.0, p1.y + offset.1);
        let left2 = (p2.x + offset.0, p2.y + offset.1);
        let right1 = (p1.x - offset.0, p1.y - offset.1);
        let right2 = (p2.x - offset.0, p2.y - offset.1);
        
        left_points.push(left1);
        right_points.push(right1);
        left_points.push(left2);
        right_points.push(right2);
    }
    
    left_points.reverse();
    let coordinates = left_points.into_iter().chain(right_points).collect::<Vec<_>>();
    
    // Create a closed Polygon by adding the first point at the end
    let closed_coordinates: Vec<Coord<f64>> = coordinates.into_iter()
        .map(|(x, y)| Coord { x, y })
        .collect();
    GeoPolygon::new(GeoLineString(closed_coordinates), vec![])
}

// Function to extract roads from GeoJSON and combine them into a MultiPolygon
pub fn roads_to_multipolygon(geojson_data: &GeoJson) -> MultiPolygon<f64> {
    let mut all_road_polygons = Vec::new(); // Store all LineString geometries

    if let GeoJson::FeatureCollection(collection) = geojson_data {
        for feature in &collection.features {
            if let Some(geometry) = &feature.geometry {
                match &geometry.value {
                    Value::LineString(coords) => {
                        // Convert the LineString coordinates from GeoJSON to geo::Polygon
                        let linestring: LineString<f64> = coords
                            .iter()
                            .map(|coord| (coord[0], coord[1])) // (x, y) without z
                            .collect();
                        let p = line_to_polygon_with_width(&linestring, 0.0001);
                        all_road_polygons.push(p);
                    },
                    Value::MultiLineString(coords_set) => {
                        // Convert MultiLineString coordinates from GeoJSON to geo::MultiPolygon
                        for coords in coords_set {
                            let linestring: LineString<f64> = coords
                                .iter()
                                .map(|coord| (coord[0], coord[1]))
                                .collect();
                            let p = line_to_polygon_with_width(&linestring, 0.0001);
                            all_road_polygons.push(p);
                        }
                    }
                    _ => {
                        // Ignore non-LineString types
                        continue;
                    }
                }
            }
        }
    }

    // Combine all the extracted Polygons into a MultiPolygon
    MultiPolygon(all_road_polygons)
}
