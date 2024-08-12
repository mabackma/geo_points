use std::io;
use geo_types::{coord, Coord, LineString, Polygon, Line};
use rand::{thread_rng, Rng};
use geo::{Intersects, line_string};

fn create_point() -> Coord<f64> {
    println!("Please input your x-coordinate. Type 'q' to stop entering points.");
    let mut x_coordinate = String::new();
    io::stdin()
        .read_line(&mut x_coordinate)
        .expect("Failed to read line");
    if x_coordinate.trim() == "q" {
        return coord! {x : -1.0, y: -1.0};
    }
    let x: f64 = x_coordinate.trim().parse().expect("Please type a number!");

    println!("Please input your y-coordinate. Type 'q' to stop entering points.");
    let mut y_coordinate = String::new();
    io::stdin()
        .read_line(&mut y_coordinate)
        .expect("Failed to read line");
    if y_coordinate.trim() == "q" {
        return coord! {x : -1.0, y: -1.0};
    }
    let y: f64 = y_coordinate.trim().parse().expect("Please type a number!");

    coord! { x: x, y: y }
}

// Generates random points within a polygon's minimum and maximum x and y coordinates
fn generate_random_points(p: Polygon, amount: i32) -> Vec<Coord<f64>> {
    let mut points = Vec::new();

    let min_x = p.exterior().0.iter().map(|c| c.x).fold(f64::INFINITY, |a, b| a.min(b));
    let max_x = p.exterior().0.iter().map(|c| c.x).fold(f64::NEG_INFINITY, |a, b| a.max(b));
    let min_y = p.exterior().0.iter().map(|c| c.y).fold(f64::INFINITY, |a, b| a.min(b));
    let max_y = p.exterior().0.iter().map(|c| c.y).fold(f64::NEG_INFINITY, |a, b| a.max(b));

    println!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y);
    let mut rng = thread_rng();

    // Generate random x and y coordinates
    for _ in 0..amount {
        let rand_x: f64 = rng.gen_range(min_x..max_x);
        let rand_y: f64 = rng.gen_range(min_y..max_y);

        let point = coord! {x: rand_x, y: rand_y};

        // Create a long line that extends beyond the polygon bounds
        let line_max_x = max_x + 1.0;

        let point_start = coord! { x: rand_x, y: rand_y };
        let point_end = coord! { x: line_max_x, y: rand_y };
        let point_line = line_string![point_start, point_end];  
        
        let mut intersections = 0;
        for line in p.exterior().lines() {
            if line.intersects(&point_line) {
                intersections += 1;
            }
        }

        println!("point_line: {:?} intersections: {}", point_line, intersections);

        // Check if random point is within the polygon
        if intersections % 2 == 0 && intersections != 0 {
            points.push(point);
        }
    }

    // Return random points
    points
}

fn main() {
    let mut coordinates = Vec::new();

    // Ask user to input coordinates for polygon
    loop {
        let coordinate = create_point();
        if coordinate.x == -1.0 && coordinate.y == -1.0 {
            break;
        } else {
            coordinates.push(coordinate);
        }
    }

    // Create polygon from coordinates
    let line_string = LineString::new(coordinates);
    let polygon = Polygon::new(line_string, vec![]);
    println!("polygon: {:?}", polygon);

    // Generate random points within the polygon
    let random_points = generate_random_points(polygon, 10);
    println!("random_points within polygon: {:?}", random_points);
}

