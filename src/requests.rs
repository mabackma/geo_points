/*
export async function fetchBuildings() {

    const mapState = useMap()

    const config = mapState.getConfig

    const options = mapState.get3DOptions

    const relativePositionData = mapState.relativePositionData!

    const url = `https://metne-test.onrender.com/geoserver/mml/ows?service=WFS&version=1.0.0&request=GetFeature&typeName=mml:rakennus&maxFeatures=2000&outputFormat=application%2Fjson&BBOX=${relativePositionData.bounds.west},${relativePositionData.bounds.south},${relativePositionData.bounds.east},${relativePositionData.bounds.north},EPSG:4326`

    const response = await fetch(url)
    const buildings = await response.json() as IBuildings

    const group = new Group()
    group.name = "buildings"

    const builtAreas: Vector2[][] = []

    buildings.features.forEach(f => {

        f.geometry.coordinates.forEach(c => {

            const vecs = c.map(cc => {

                let wgs84 = [...inverse(cc[0], cc[1])]



                return coordPointToThreeVec(wgs84, config, options, relativePositionData)

            })

            if(Math.abs(vecs[0].x) > (config.MAP_PIXEL_WIDTH / 2 * config.worldScale) || Math.abs(vecs[0].y) > (config.MAP_PIXEL_HEIGHT / 2 *  config.worldScale)){
                return
            }

            const height = getHeightByPosition(vecs[0].x, vecs[0].y, config.worldScale, config.MAP_PIXEL_WIDTH)

            const shape = new Shape(vecs)

            const extrudeSettings = {
                steps: 2,
                depth: 5 + ((f.properties?.kerrosluku ?? 1) * 5),
                bevelEnabled: true,
                bevelThickness: 1,
                bevelSize: 1,
                bevelOffset: 0,
                bevelSegments: 1
            };

            const geometry = new ExtrudeGeometry( shape, extrudeSettings );

            const color = `#1111${f.properties?.kayttotarkoitus ?? '00'}`

            const material = new MeshStandardMaterial( { color: color.length != 7 ? color + "0" : color} );
            const mesh = new Mesh( geometry, material ) ;
            mesh.rotation.x = Math.PI / 2
            mesh.position.y = height + extrudeSettings.depth



            builtAreas.push(vecs)
            group.add(mesh)
        })
    })

    return {group, builtAreas}

}
*/

use crate::geometry_utils::get_min_max_coordinates;
use geo::{LineString, Polygon};
use geojson::{GeoJson, Value};
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