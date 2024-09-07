use crate::geometry_utils::get_min_max_coordinates;
use geo::{LineString, Polygon};
use geojson::{GeoJson, Value};
use image::DynamicImage;
use reqwest;

use reqwest::Error as ReqwestError;
use geojson::Error as GeoJsonError;
use std::fmt;
use std::error::Error;

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

pub async fn fetch_buildings(bbox: &Polygon) -> Result<GeoJson, FetchError> {
    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&bbox);

    let west = min_x;
    let south = min_y;
    let east = max_x;
    let north = max_y;

    let url = format!(
        "https://metne-test.onrender.com/geoserver/mml/ows?service=WFS&version=1.0.0&request=GetFeature&typeName=mml:rakennus&maxFeatures=2000&outputFormat=application%2Fjson&BBOX={},{},{},{},EPSG:4326&srsName=EPSG:4326",
        west, south, east, north
    );

    let resp = reqwest::get(&url)
        .await?
        .text()
        .await?;
    
    let geojson = resp.parse::<GeoJson>()?;

    Ok(geojson)
}

pub async fn fetch_buildings_as_polygons(bbox: &Polygon<f64>) -> Result<Vec<Polygon<f64>>, Box<dyn Error>> {
    // Fetch GeoJson data from API
    let geojson = fetch_buildings(bbox).await?;

    // Initialize a vector to store polygons
    let mut polygons = Vec::new();

    // Match on GeoJson to handle FeatureCollection
    if let GeoJson::FeatureCollection(collection) = geojson {
        for feature in collection.features {
            // Ensure we are working with a valid Feature
            if let Some(geometry) = feature.geometry {
                match geometry.value {
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


#[derive(Debug)]
pub struct TileParams {
    format: &'static str,
    z: u32,
    x: u32,
    y: u32,
    offset_multiplier_x: f32,
    offset_multiplier_y: f32
}

impl TileParams {
    pub fn new(format: &'static str, z: u32, x: u32, y: u32, offset_multiplier_x: f32, offset_multiplier_y: f32) -> Self {
        TileParams {
            format,
            z,
            x,
            y, 
            offset_multiplier_x,
            offset_multiplier_y
        }
    }
}

#[derive(Debug)]
pub enum MmlTile {
    Water,
    Roads,
}

impl MmlTile {
    pub fn as_str(&self) -> &'static str {
        match self {
            MmlTile::Water => "vesi",
            MmlTile::Roads => "tieviiva",
        }
    }
}

pub async fn get_slippy_tile(tile_params: TileParams, tile_type: &str) -> Result<DynamicImage, Box<dyn Error>> {
    let slippy_y = 2u32.pow(tile_params.z) - tile_params.y - 1;

    let url = format!(
        "https://metne-test.onrender.com/geoserver/gwc/service/tms/1.0.0/mml:{}@EPSG%3A900913@png/{}/{}/{}.png",
        tile_type,
        tile_params.z,
        tile_params.x,
        slippy_y
    );

    let resp = reqwest::get(&url).await?;

    if resp.status().is_success() {
        let img = image::load_from_memory(&resp.bytes().await?);
        Ok(img.unwrap())
    } else {
        Err("Failed to fetch image".into())
    }
}
/* EXAMPLE URL FOR MAKING REQUESTS TO GET ROADS IN GEOJSON*/
/*
    https://metne-test.onrender.com/geoserver/mml/ows?service=WFS&version=1.0.0&request=GetFeature&srsName=EPSG:3067&typeName=mml:tieviiva&maxFeatures=50&bbox=444000,7375200,445000,7378000,urn:ogc:def:crs:EPSG:3067&outputFormat=application/json



    USE THIS FORMAT:

    https://metne-test.onrender.com/geoserver/mml/ows?service=WFS&version=1.0.0&request=GetFeature&srsName=EPSG:4326&typeName=mml:tieviiva&maxFeatures=50&bbox=min_x,min_y,max_x,max_y,urn:ogc:def:crs:EPSG:4326&outputFormat=application/json

    where min_x, min_y, max_x, max_y are the bounding box coordinates after transforming the coordinates to EPSG:4326
*/

pub async fn fetch_roads(bbox: &Polygon) -> Result<GeoJson, FetchError> {
    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&bbox);

    let west = min_x;
    let south = min_y;
    let east = max_x;
    let north = max_y;

    let url = format!(
        "https://metne-test.onrender.com/geoserver/mml/ows?service=WFS&version=1.0.0&request=GetFeature&typeName=mml:tieviiva&bbox={},{},{},{},EPSG:4326&srsName=EPSG:4326&outputFormat=application/json",
        west, south, east, north
    );
    

    let resp = reqwest::get(&url)
        .await?
        .text()
        .await?;
    
    let geojson = resp.parse::<GeoJson>()?;
    
    Ok(geojson)
}
