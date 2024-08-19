use geo_types::{coord, Coord, LineString, Polygon};
use rand::{thread_rng, Rng};
use geo::{Contains, BoundingRect};
use image::Rgb;

use crate::forest_property::{tree::Tree, tree_stand_data::TreeStrata};

// Get minimum and maximum x and y coordinates of a polygon
pub fn get_min_max_coordinates(p: &Polygon<f64>) -> (f64, f64, f64, f64) {
    let rect = p.bounding_rect().unwrap();
    let min_x = rect.min().x;
    let max_x = rect.max().x;
    let min_y = rect.min().y;
    let max_y = rect.max().y;

    (min_x, max_x, min_y, max_y)
}

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

// Generates random points within a polygon's minimum and maximum x and y coordinates
pub fn random_poisson_disc_points(p: &Polygon, amount: i32, radius: f64) -> Vec<Coord<f64>> {
    let mut points = Vec::new();
    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&p);

    // Generate random x and y coordinates
    let mut count = 0;
    let mut rng = thread_rng();
    loop {
        let rand_x: f64 = rng.gen_range(min_x..max_x);
        let rand_y: f64 = rng.gen_range(min_y..max_y);
        let point = coord! {x: rand_x, y: rand_y};

        if p.contains(&point) && is_valid_point(&point, &points, radius) {
            points.push(point);
            count += 1;
            if count == amount {
                break;
            }
        }
    }

    points
}

// Helper function to check if a new point is at least `radius` away from existing points
fn is_valid_point(new_point: &Coord<f64>, points: &[Coord<f64>], radius: f64) -> bool {
    for point in points {
        if euclidean_distance(*new_point, *point) < radius {
            return false;
        }
    }
    true
}

// Function to calculate Euclidean distance between two points
fn euclidean_distance(p1: Coord<f64>, p2: Coord<f64>) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

// Generates random trees for all strata within a polygon's minimum and maximum x and y coordinates
pub fn generate_random_trees(p: &Polygon, strata: &TreeStrata) -> Vec<Tree> {
    let mut trees = Vec::new();

    for stratum in strata.tree_stratum.iter() {
        let amount = stratum.stem_count.unwrap_or(0);
        let random_points = generate_random_points(&p, amount as i32);

        // Generate random trees for each stratum
        for point in random_points {
            let tree = Tree::new(stratum.tree_species, stratum.mean_height, (point.x, point.y));
            trees.push(tree);
        }
    }

    trees
}