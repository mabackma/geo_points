use geo::{coord, Coord};
use geo_types::LineString;
use crate::geometry_utils::generate_poisson_disc_points;
use geo_types::Polygon;

pub struct Bounds {
    north: f64,
    south: f64,
    east: f64,
    west: f64,
}

// Takes data_from_js as [lat, lng, north, west, east, south, world_scale, area_width_pixels] Vec<f64>
// Returns a Vec<f64> of 3D positions in the format [x1, y1, z1, x2, y2, z2, ...]
pub fn generate_area_trees(data_from_js: Vec<f64>) -> Vec<f64> {
    let bounds = Bounds {
        north: data_from_js[2],
        west: data_from_js[3],
        east: data_from_js[4],
        south: data_from_js[5],
    };
    let world_scale = data_from_js[6];
    let area_width_pixels = data_from_js[7] as u16;

    // Create a LineString from the bounds
    let exterior = LineString::from(vec![
        coord! { x: bounds.west, y: bounds.north },
        coord! { x: bounds.east, y: bounds.north },
        coord! { x: bounds.east, y: bounds.south },
        coord! { x: bounds.west, y: bounds.south },
        coord! { x: bounds.west, y: bounds.north }, // Close the polygon
    ]);

    // Create a polygon from the bounds
    let p = Polygon::new(
        exterior,
        vec![], // No holes in the polygon
    );
    
    // Generate Poisson disc points inside the polygon
    let points = generate_poisson_disc_points(&p, 10.0);

    // For each tree, calculate the 3D position
    let mut vec_3d = Vec::new();
    let mut point_3d = Vec::new();
    for point in points.iter() {
        point_3d = coord_point_to_three_vec(*point, &bounds, world_scale, area_width_pixels);
        vec_3d.push(point_3d);

    }

    vec_3d.into_iter().flatten().collect()
}

pub fn coord_point_to_three_vec(pos: Coord, bounds: &Bounds, world_scale: f64, area_width_pixels: u16) -> Vec<f64> {
    let offset = area_width_pixels / 2;
   
    let step_x = (bounds.east - bounds.west) / area_width_pixels as f64;
    let step_y = (bounds.north - bounds.south) / area_width_pixels as f64;
   
    let x = (pos.x - bounds.west) / step_x - offset as f64;
    let y = (bounds.north - pos.y) / step_y - offset as f64;
   
    vec![x * world_scale, y * world_scale, 0.0]
} 