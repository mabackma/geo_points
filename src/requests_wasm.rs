use crate::geometry_utils::get_min_max_coordinates;
use crate::forest_property::forest_property_data::ForestPropertyData;
use crate::forest_property::compartment::get_compartments_in_bounding_box;
use crate::geojson_utils::all_compartments_to_geojson;
use geo::{coord, LineString, Polygon};
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
/* 
#[wasm_bindgen]
pub async fn fetch_buildings(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Result<JsValue, JsValue> {
    let west = min_x;
    let south = min_y;
    let east = max_x;
    let north = max_y;

    let url = format!(
        "https://metne-test.onrender.com/geoserver/mml/ows?service=WFS&version=1.0.0&request=GetFeature&typeName=mml:rakennus&maxFeatures=2000&outputFormat=application%2Fjson&BBOX={},{},{},{},EPSG:4326&srsName=EPSG:4326",
        west, south, east, north
    );

    // Perform the HTTP request
    let response = reqwest::get(&url).await.map_err(|e| JsValue::from_str(&format!("Failed to fetch buildings: {}", e)))?;

    // Convert response text to GeoJson
    let resp_text = response.text().await.map_err(|e| JsValue::from_str(&format!("Failed to read response text: {}", e)))?;
    let geojson: SerdeJsonValue = serde_json::from_str(&resp_text).map_err(|e| JsValue::from_str(&format!("Failed to parse GeoJson: {}", e)))?;
    
    Ok(serde_wasm_bindgen::to_value(&geojson).map_err(|e| JsValue::from_str(&format!("Failed to serialize GeoJson: {}", e)))?)
}

/* 
#[wasm_bindgen]
pub fn buildings_as_polygons(geojson: &GeoJson) -> Result<Vec<Polygon<f64>>, Box<dyn Error>> {
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
    } else {
        return Err("GeoJson is not a FeatureCollection.".into());
    }

    Ok(polygons)
}
*/

#[wasm_bindgen]
pub async fn fetch_roads(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Result<JsValue, JsValue> {
    let west = min_x;
    let south = min_y;
    let east = max_x;
    let north = max_y;

    let url = format!(
        "https://metne-test.onrender.com/geoserver/mml/ows?service=WFS&version=1.0.0&request=GetFeature&typeName=mml:tieviiva&bbox={},{},{},{},EPSG:4326&srsName=EPSG:4326&outputFormat=application/json",
        west, south, east, north
    );

    // Perform the HTTP request
    let response = reqwest::get(&url).await.map_err(|e| JsValue::from_str(&format!("Failed to fetch buildings: {}", e)))?;

    // Convert response text to GeoJson
    let resp_text = response.text().await.map_err(|e| JsValue::from_str(&format!("Failed to read response text: {}", e)))?;
    let geojson: SerdeJsonValue = serde_json::from_str(&resp_text).map_err(|e| JsValue::from_str(&format!("Failed to parse GeoJson: {}", e)))?;
    
    Ok(serde_wasm_bindgen::to_value(&geojson).map_err(|e| JsValue::from_str(&format!("Failed to serialize GeoJson: {}", e)))?)
}
*/
/* 
#[wasm_bindgen]
pub async fn create_geo_json_from_coords(
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
) -> Result<JsValue, JsValue> {
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

    // Fetch buildings GeoJSON
    let buildings_response = reqwest::get(&url_buildings).await.map_err(|e| JsValue::from_str(&format!("Failed to fetch buildings: {}", e)))?;
    let buildings_text = buildings_response.text().await.map_err(|e| JsValue::from_str(&format!("Failed to read buildings response text: {}", e)))?;
    let buildings_geojson: GeoJson = serde_json::from_str(&buildings_text).map_err(|e| JsValue::from_str(&format!("Failed to parse buildings GeoJson: {}", e)))?;

    // Fetch roads GeoJSON
    let roads_response = reqwest::get(&url_roads).await.map_err(|e| JsValue::from_str(&format!("Failed to fetch buildings: {}", e)))?;
    let roads_text = roads_response.text().await.map_err(|e| JsValue::from_str(&format!("Failed to read roads response text: {}", e)))?;
    let roads_geojson: GeoJson = serde_json::from_str(&roads_text).map_err(|e| JsValue::from_str(&format!("Failed to parse roads GeoJson: {}", e)))?;

    let property = ForestPropertyData::from_xml_file("forestpropertydata.xml");
    let real_estate = property.real_estates.real_estate[0].clone();
    let stands = real_estate.get_stands();

    let bbox = geo::Polygon::new(
        LineString(vec![
            coord!(x: min_x, y: min_y),
            coord!(x: max_x, y: min_y),
            coord!(x: max_x, y: max_y),
            coord!(x: min_x, y: max_y),
            coord!(x: min_x, y: min_y),
        ]),
        vec![],
    );

    let compartments = get_compartments_in_bounding_box(stands, &bbox);
    let geojson = all_compartments_to_geojson(compartments, &bbox, &buildings_geojson, &roads_geojson);

    // Convert processed data to JsValue for returning
    let result = serde_wasm_bindgen::to_value(&geojson).map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))?;

    Ok(result)
}
*/
#[wasm_bindgen]
pub async fn geo_json_from_coords(
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    xml_content: String,
) -> Result<JsValue, JsValue> {
    // Timer
    let start = Instant::now();

    let property = ForestPropertyData::from_xml_str(&xml_content);
    web_sys::console::log_1(&"Got property".into());

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

    let bbox = Polygon::new(
        LineString(vec![
            coord!(x: min_x, y: min_y),
            coord!(x: max_x, y: min_y),
            coord!(x: max_x, y: max_y),
            coord!(x: min_x, y: max_y),
            coord!(x: min_x, y: min_y),
        ]),
        vec![],
    );

    // Get compartments in the bounding box and convert them to GeoJSON
    let compartments = get_compartments_in_bounding_box(stands, &bbox);
    let geojson = all_compartments_to_geojson(compartments, &bbox, &buildings_geojson, &roads_geojson);
    web_sys::console::log_1(&"Got geojson".into());
    
    // Convert the resulting GeoJSON to JsValue for returning to JavaScript
    let result = serde_wasm_bindgen::to_value(&geojson)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))?;

    // Stop the timer and log the duration
    let duration = start.elapsed();
    let duration_str = format!("Time elapsed in geo_json_from_coords is: {:?}", duration);
    console::log_1(&JsValue::from_str(&duration_str));

    Ok(result)
}