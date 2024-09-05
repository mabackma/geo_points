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
        "https://metne-test.onrender.com/geoserver/mml/ows?service=WFS&version=1.0.0&request=GetFeature&typeName=mml:rakennus&maxFeatures=2000&outputFormat=application%2Fjson&BBOX={},{},{},{},EPSG:4326",
        west, south, east, north
    );
    
    println!("{}", url);

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


/*
/* SHOWS IMAGES OF MAP TILES */
function getTile(tileParams: { type: any; z: any; x: any; y: any; format: any; offsetMultiplierX: any; offsetMultiplierY: any; }, ctx: CanvasRenderingContext2D) {
    return new Promise((resolve, reject) => {
        const { type, z, x, y, format, offsetMultiplierX, offsetMultiplierY } =
            tileParams;

        const image = new Image(TILE_WIDTH, TILE_WIDTH);
        image.crossOrigin = "anonymous";

        image.onload = function () {
            ctx.drawImage(image, TILE_WIDTH * offsetMultiplierX, TILE_WIDTH * offsetMultiplierY);
            image.style.display = "none";
            resolve(null);
        };

        image.onerror = function () {
            reject();
        };

        image.src = `https://s3.amazonaws.com/elevation-tiles-prod/${type}/${z}/${x}/${y}.${format}`;
    });
}


function getSlippyTile(
    tileParams: { format: string, type: string, z: number; x: number; y: number; offsetMultiplierX: number; offsetMultiplierY: number; },
    ctx: CanvasRenderingContext2D,
    tileType: mml_tile, tryCount: number = 0) {
    return new Promise((resolve, reject) => {



        const { z, x, y, offsetMultiplierX, offsetMultiplierY } =
            tileParams;

        const image = new Image(TILE_WIDTH, TILE_WIDTH);
        image.crossOrigin = "anonymous";
        image.onload = function () {

            ctx.drawImage(image, TILE_WIDTH * offsetMultiplierX, TILE_WIDTH * offsetMultiplierY);
            resolve(null);


        };

        image.onerror = async (error) => {

            try {

                if (tryCount > 2) throw new Error("Try amount exceeded, default to error")

                const res = await getSlippyTile(tileParams, ctx, tileType, tryCount + 1)

                resolve(res)

            } catch (error) {
                reject();
            }
        };

        const slippyY = Math.pow(2, z) - y - 1;

        image.src = `https://metne-test.onrender.com/geoserver/gwc/service/tms/1.0.0/mml:${tileType}@EPSG%3A900913@png/${z}/${x}/${slippyY}.png`;

        // Tiet
        // image.src = `https://metne-test.onrender.com/geoserver/gwc/service/tms/1.0.0/mml:tieviiva@EPSG%3A900913@png/${z}/${x}/${slippyY}.png`;

        // image.src = `http://geo.plab.fi/geoserver/gwc/service/tms/1.0.0/mml:vesi@EPSG%3A900913@png/${z}/${x}/${slippyY}.png`;

    });    
}
*/
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