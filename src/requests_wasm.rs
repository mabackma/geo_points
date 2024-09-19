use crate::geometry_utils::get_min_max_coordinates;
use geo::{LineString, Polygon};
use geojson::{GeoJson, Value};
use reqwest;

use reqwest::get;
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
