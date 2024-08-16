use geo_types::{Coord, Polygon};
use geo::BoundingRect;
use image::{Rgb, RgbImage, ImageBuffer};

// Get minimum and maximum x and y coordinates of a polygon
pub fn get_min_max_coordinates(p: &Polygon<f64>) -> (f64, f64, f64, f64) {
    let rect = p.bounding_rect().unwrap();
    let min_x = rect.min().x;
    let max_x = rect.max().x;
    let min_y = rect.min().y;
    let max_y = rect.max().y;

    (min_x, max_x, min_y, max_y)
}

// Scale polygon to fit image
pub fn scale_x_and_y(p: &Polygon<f64>, img_width: u32, img_height: u32) -> (f64, f64, f64, f64) {
    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&p);
    let width = max_x - min_x;
    let height = max_y - min_y;

    let scale_x = img_width as f64 / width;
    let scale_y = img_height as f64 / height;

    (scale_x, scale_y, min_x, min_y)
}

// Map polygon coordinates to image pixel coordinates
pub fn map_coordinates_to_image(p: &Polygon<f64>, img_width: u32, img_height: u32) -> Vec<(u32, u32)> {
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
pub fn draw_line_segment(img: &mut RgbImage, p1: (u32, u32), p2: (u32, u32), color: Rgb<u8>) {
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

// Draw the polygon edges by connecting points
pub fn draw_polygon_image(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, coords: Vec<(u32, u32)>) {
    for i in 0..coords.len() {
        let (x0, y0) = coords[i];
        let (x1, y1) = coords[(i + 1) % coords.len()]; // Wrap around to connect the last point to the first
        draw_line_segment(img, (x0, y0), (x1, y1), Rgb([0, 0, 255]));
    }
}

// Draws a random point
pub fn draw_random_point(img: &mut RgbImage, p: &Polygon, img_width: u32, img_height: u32, point: Coord, color: Rgb<u8>) {
    let (scale_x, scale_y, min_x, min_y) = scale_x_and_y(&p, img_width, img_height);
    let mut x = ((point.x - min_x) * scale_x).round() as u32;
    let mut y = (img_height as f64 - (point.y - min_y) * scale_y).round() as u32;

    // Clamp x and y to ensure they are within bounds
    x = x.max(0).min(img_width - 1);
    y = y.max(0).min(img_height - 1);

    img.put_pixel(x, y, color);
} 