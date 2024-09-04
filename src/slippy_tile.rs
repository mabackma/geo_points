use std::f32::consts::PI;

// Convert indexes to longitude and latitude
// X and Y are the tile indexes, Z is the zoom level
pub fn tile_to_lon_lat_f32(x: u32, y: u32, z: u32) -> (f32, f32, f32, f32) {
    let n = 2.0_f32.powi(z as i32);

    // Convert tile x/y to top-left corner longitude/latitude
    let lon_min = x as f32 / n * 360.0 - 180.0;
    let lat_min = ((PI * (1.0 - 2.0 * y as f32 / n)).sinh()).atan().to_degrees();

    // Convert tile (x+1)/(y+1) to bottom-right corner longitude/latitude
    let lon_max = (x + 1) as f32 / n * 360.0 - 180.0;
    let lat_max = ((PI * (1.0 - 2.0 * (y + 1) as f32 / n)).sinh()).atan().to_degrees();

    (lon_min, lat_min, lon_max, lat_max)
}

// Convert longitude and latitude to tile indexes
// Z is the zoom level
pub fn lon_lat_to_tile_indexes_f32(lon: f32, lat: f32, z: u32) -> (u32, u32) {
    let n = 2.0_f32.powi(z as i32);

    // Convert longitude to tile x index
    let x = ((lon + 180.0) / 360.0 * n).floor() as u32;

    // Convert latitude to tile y index (Mercator projection)
    let lat_rad = lat.to_radians();
    let y = ((1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / PI) / 2.0 * n).floor() as u32;

    (x, y)
}