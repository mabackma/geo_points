use crate::image_utils::get_min_max_coordinates;

use geo_types::{coord, Coord, LineString, Polygon};
use rand::{thread_rng, Rng};
use geo::Contains;

// Create a polygon from a string of coordinates
pub fn create_polygon(coord_string: &str) -> Polygon<f64> {
    let coordinates_str: Vec<&str> = coord_string.split(" ").collect();

    // Parse coordinates into a Vec of `Coord<f64>`
    let mut coords: Vec<Coord<f64>> = Vec::new();
    for coordinate in coordinates_str {
        let parts: Vec<&str> = coordinate.split(',').collect();
        if parts.len() == 2 {
            let x: f64 = parts[0].parse().expect("Invalid x coordinate");
            let y: f64 = parts[1].parse().expect("Invalid y coordinate");
            coords.push(Coord { x, y });
        } else {
            println!("Invalid coordinate format: {}", coordinate);
        }
    }

    let line_string = LineString::new(coords);
    let polygon = Polygon::new(line_string, vec![]);

    polygon
}

// Generates random points within a polygon's minimum and maximum x and y coordinates
pub fn generate_random_points(p: &Polygon, amount: i32) -> Vec<Coord<f64>> {
    let mut points = Vec::new();
    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&p);

    // Generate random x and y coordinates
    let mut count = 0;
    let mut rng = thread_rng();
    loop {
        let rand_x: f64 = rng.gen_range(min_x..max_x);
        let rand_y: f64 = rng.gen_range(min_y..max_y);
        let point = coord! {x: rand_x, y: rand_y};

        if p.contains(&point) {
            points.push(point);
            count += 1;
            if count == amount {
                break;
            }
        }
    }

    points
}