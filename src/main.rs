use std::io;
use geo_types::{coord, Coord, LineString, Polygon};
use rand::{thread_rng, Rng};
use geo::{BoundingRect, Contains};
use image::{Rgb, RgbImage};
use chrono::{DateTime, Utc}; 

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

// Get minimum and maximum x and y coordinates of a polygon
fn get_min_max_coordinates(p: &Polygon<f64>) -> (f64, f64, f64, f64) {
    let rect = p.bounding_rect().unwrap();
    let min_x = rect.min().x;
    let max_x = rect.max().x;
    let min_y = rect.min().y;
    let max_y = rect.max().y;

    (min_x, max_x, min_y, max_y)
}

// Generates random points within a polygon's minimum and maximum x and y coordinates
fn generate_random_points(p: &Polygon, amount: i32) -> Vec<Coord<f64>> {
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

    // Return random points
    points
}

// Scale polygon to fit image
fn scale_x_and_y(p: &Polygon<f64>, img_width: u32, img_height: u32) -> (f64, f64, f64, f64) {
    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&p);
    let width = max_x - min_x;
    let height = max_y - min_y;

    // Scale polygon to fit image
    let scale_x = img_width as f64 / width;
    let scale_y = img_height as f64 / height;

    (scale_x, scale_y, min_x, min_y)
}

// Map polygon coordinates to image pixel coordinates
fn map_coordinates_to_image(p: &Polygon<f64>, img_width: u32, img_height: u32) -> Vec<(u32, u32)> {
    let (scale_x, scale_y, min_x, min_y) = scale_x_and_y(p, img_width, img_height);

    p.exterior()
        .points()
        .map(|point| {
            let x = ((point.x() - min_x) * scale_x).round() as u32;
            let y = (img_height as f64 - (point.y() - min_y) * scale_y).round() as u32;
            (x, y)
        })
        .collect()
}

// DDA Line algorithm to draw line segments
fn draw_line_segment(img: &mut RgbImage, p1: (u32, u32), p2: (u32, u32), color: Rgb<u8>) {
    println!("Drawing line segment from {:?} to {:?}", p1, p2);

    let dx = p2.0 as i32 - p1.0 as i32;
    let dy = p2.1 as i32 - p1.1 as i32;

    // Choose the larger of dx and dy as the number of steps to take
    let steps = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() };
    
    // Calculate the increment in x and y for each step
    let x_step = dx as f64 / steps as f64;
    let y_step = dy as f64 / steps as f64;

    let mut x = p1.0 as f64;
    let mut y = p1.1 as f64;

    // Draw the line segment pixel by pixel
    for _ in 0..=steps {
        if x >= 0.0 && y >= 0.0 && (x as u32) < img.width() && (y as u32) < img.height() {
            img.put_pixel(x as u32, y as u32, color);
        }
        x += x_step;
        y += y_step;
    }
}

// Draws a random point
fn draw_random_point(img: &mut RgbImage, p: &Polygon, img_width: u32, img_height: u32, point: Coord, color: Rgb<u8>) {
    let (scale_x, scale_y, min_x, min_y) = scale_x_and_y(&p, img_width, img_height);
    let x = ((point.x - min_x) * scale_x).round() as u32;
    let y = (img_height as f64 - (point.y - min_y) * scale_y).round() as u32;
    img.put_pixel(x, y, color);
} 

// Draw image with polygon and random points
fn draw_image(p: &Polygon, rand_p: Vec<Coord<f64>>) {
    let img_width = 800;
    let img_height = 600;
    let mut img = RgbImage::new(img_width, img_height);

    // Map polygon coordinates to image
    let mapped_coordinates = map_coordinates_to_image(&p, img_width, img_height);

    // Draw the polygon edges by connecting points
    for i in 0..mapped_coordinates.len() {
        let (x0, y0) = mapped_coordinates[i];
        let (x1, y1) = mapped_coordinates[(i + 1) % mapped_coordinates.len()]; // Wrap around to connect the last point to the first
        draw_line_segment(&mut img, (x0, y0), (x1, y1), Rgb([0, 0, 255]));
    }

    // Draw the generated random points within the polygon
    for point in rand_p {
        draw_random_point(&mut img, &p, img_width, img_height, point, Rgb([255, 0, 0])) // Draw points in red
    }

    img.save("polygon_image.png").expect("Failed to save image");
    println!("Polygon image saved as 'polygon_image.png'");
}

struct Stand {
    stand_basic_data: StandBasicData,
    tree_stand_data: TreeStandData,
}

struct StandBasicData {
    change_state: u32,
    change_time: DateTime<Utc>, 
    complete_state: u32,
    stand_number: u32,
    stand_number_extension: String,
    main_group: u32,
    sub_group: u32,
    fertility_class: u32,
    soil_type: u32,
    drainage_state: u32,
    development_class: u32,
    stand_quality: u32,
    main_tree_species: u32,
    accessibility: u32,
    stand_basic_data_date: DateTime<Utc>, 
    area: u32,
    point_property: Coord<f64>,
    exterior_lr: Coord<f64>,
    interior_lr: Coord<f64>,
}

struct TreeStrata {
    change_state: u32,
    stratum_number: u32,
    tree_species: u32,
    storey: u32,
    age: u32,
    basal_area: f64,
    mean_diameter: u32,
    mean_height: f64,
    data_source: u32,
}

struct TreeStandSummary {
    change_state: u32,
    mean_age: u32,
    basal_area: f64,
    stem_count: u32,
    mean_diameter: u32,
    mean_height: f64,
    volume: u32,
    volume_growth: u32,
}

struct TreeStandData {
    tree_strata: TreeStrata,
    tree_stand_summary: TreeStandSummary,
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
    let random_points = generate_random_points(&polygon, 10);
    println!("random_points within polygon: {:?}", random_points);  

    draw_image(&polygon, random_points);
}

