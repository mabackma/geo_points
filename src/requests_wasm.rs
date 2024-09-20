use crate::geometry_utils::get_min_max_coordinates;
use crate::forest_property::forest_property_data::ForestPropertyData;
use crate::forest_property::compartment::{get_compartment_areas_in_bounding_box, CompartmentArea};
use crate::geojson_utils::all_compartment_areas_to_geojson;
use geo::{coord, LineString, Polygon, BooleanOps};
use geojson::{GeoJson, Value};
use reqwest_wasm::Client;
use reqwest::Error as ReqwestError;
use geojson::Error as GeoJsonError;
use std::fmt;
use std::error::Error;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use serde_json::Value as SerdeJsonValue;
use serde_wasm_bindgen;
use web_sys::console::log_1;

#[derive(Debug)]
pub enum FetchError {
    Reqwest(ReqwestError),
    GeoJson(GeoJsonError),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            FetchError::GeoJson(err) => write!(f, "GeoJson error: {}", err),
        }
    }
}

impl std::error::Error for FetchError {}

impl From<ReqwestError> for FetchError {
    fn from(err: ReqwestError) -> Self {
        FetchError::Reqwest(err)
    }
}

impl From<GeoJsonError> for FetchError {
    fn from(err: GeoJsonError) -> Self {
        FetchError::GeoJson(err)
    }
}

pub fn geojson_to_polygons(geojson: &GeoJson) -> Vec<Polygon<f64>> {
    // Initialize a vector to store polygons
    let mut polygons = Vec::new();

    // Match on GeoJson to handle FeatureCollection
    if let GeoJson::FeatureCollection(collection) = geojson {
        for feature in &collection.features {
            // Ensure we are working with a valid Feature
            if let Some(geometry) = &feature.geometry {
                match &geometry.value {
                    Value::Polygon(polygon) => {
                        // Convert GeoJSON Polygon to geo crate Polygon
                        let exterior = polygon[0]
                            .iter()
                            .map(|point| (point[0], point[1]))
                            .collect::<Vec<_>>();
                        
                        // Create a geo crate Polygon
                        let poly = Polygon::new(LineString::from(exterior), vec![]);
                        polygons.push(poly);
                    }
                    _ => {
                        // Handle other geometry types if necessary
                        eprintln!("Skipping non-polygon geometry");
                    }
                }
            }
        }
    } 

    polygons
}

#[wasm_bindgen]
pub async fn geo_json_from_coords(
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    xml_content: String,
) -> Result<JsValue, JsValue> {
    // Get the ForestPropertyData from the XML content
    let property = ForestPropertyData::from_xml_str(&xml_content);
    log_1(&"Got property".into());

    let mut bbox = Polygon::new(
        LineString(vec![
            coord!(x: min_x, y: min_y),
            coord!(x: max_x, y: min_y),
            coord!(x: max_x, y: max_y),
            coord!(x: min_x, y: max_y),
            coord!(x: min_x, y: min_y),
        ]),
        vec![],
    );

    let west = min_x;
    let south = min_y;
    let east = max_x;
    let north = max_y;

    let url_buildings = format!(
        "https://metne-test.onrender.com/geoserver/mml/ows?service=WFS&version=1.0.0&request=GetFeature&typeName=mml:rakennus&maxFeatures=2000&outputFormat=application%2Fjson&BBOX={},{},{},{},EPSG:4326&srsName=EPSG:4326",
        west, south, east, north
    );
    let url_roads = format!(
        "https://metne-test.onrender.com/geoserver/mml/ows?service=WFS&version=1.0.0&request=GetFeature&typeName=mml:tieviiva&bbox={},{},{},{},EPSG:4326&srsName=EPSG:4326&outputFormat=application/json",
        west, south, east, north
    );

    // Create HTTP client for async fetch
    let client = Client::new();

    // Fetch buildings GeoJSON
    let buildings_response = client.get(&url_buildings).send().await
        .map_err(|e| JsValue::from_str(&format!("Failed to fetch buildings: {}", e)))?;
    let buildings_text = buildings_response.text().await
        .map_err(|e| JsValue::from_str(&format!("Failed to read buildings response text: {}", e)))?;
    let buildings_geojson: GeoJson = serde_json::from_str(&buildings_text)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse buildings GeoJson: {}", e)))?;

    let buildings = geojson_to_polygons(&buildings_geojson);
    let buildings_count = buildings.len();
    log_1(&format!("Fetched {} buildings", buildings_count).into());

    // Exclude buildings from the bounding box
    for building in buildings.iter() {
        bbox = bbox.difference(building).0.first().unwrap().to_owned();
    }

    // Fetch roads GeoJSON
    let roads_response = client.get(&url_roads).send().await
        .map_err(|e| JsValue::from_str(&format!("Failed to fetch roads: {}", e)))?;
    let roads_text = roads_response.text().await
        .map_err(|e| JsValue::from_str(&format!("Failed to read roads response text: {}", e)))?;
    let roads_geojson: GeoJson = serde_json::from_str(&roads_text)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse roads GeoJson: {}", e)))?;

    // Get the ForestPropertyData and stands
    let real_estate = property.real_estates.real_estate[0].clone();
    let stands = real_estate.get_stands();

    // Get compartment areas in the bounding box and convert them to GeoJSON
    let compartment_areas = get_compartment_areas_in_bounding_box(stands, &bbox);
    let geojson = all_compartment_areas_to_geojson(compartment_areas, &buildings_geojson, &roads_geojson);
    log_1(&"Got geojson".into());

    // Convert the resulting GeoJSON to JsValue for returning to JavaScript
    let result = serde_wasm_bindgen::to_value(&geojson)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))?;

    Ok(result)
}