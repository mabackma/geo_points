use crate::geometry_utils::get_min_max_coordinates;
use geo_types::{Coord, Polygon};
use image::{Rgb, RgbImage};

pub struct ImageProcessor {
    img: RgbImage,
    width: u32,
    height: u32,
}

impl ImageProcessor {
    pub fn new(width: u32, height: u32) -> Self {
        ImageProcessor {
            img: RgbImage::new(width, height),
            width,
            height,
        }
    }

    pub fn img(&self) -> &RgbImage{
        &self.img
    }

    // Scale polygon to fit image
    pub fn scale_x_and_y(&self, p: &Polygon<f64>) -> (f64, f64, f64, f64) {
        let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&p);
        let width = max_x - min_x;
        let height = max_y - min_y;

        let scale_x = self.width as f64 / width;
        let scale_y = self.height as f64 / height;

        (scale_x, scale_y, min_x, min_y)
    }

    // Map polygon coordinates to image pixel coordinates
    pub fn map_coordinates_to_image(&self, p: &Polygon<f64>) -> Vec<(u32, u32)> {
        let (scale_x, scale_y, min_x, min_y) = self.scale_x_and_y(p);

        p.exterior()
            .points()
            .map(|point| {
                let x = ((point.x() - min_x) * scale_x).round() as u32;
                let y = (self.height as f64 - (point.y() - min_y) * scale_y).round() as u32;
                (x, y)
            })
            .collect()
    }

    // DDA Line algorithm to draw line segments
    pub fn draw_line_segment(&mut self, p1: (u32, u32), p2: (u32, u32), color: Rgb<u8>) {
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
            if x >= 0.0 && y >= 0.0 && (x as u32) < self.img.width() && (y as u32) < self.img.height() {
                self.img.put_pixel(x as u32, y as u32, color);
            }
            x += x_step;
            y += y_step;
        }
    }

    // Draw the polygon edges by connecting points
    pub fn draw_polygon_image(&mut self, coords: Vec<(u32, u32)>) {
        for i in 0..coords.len() {
            let (x0, y0) = coords[i];
            let (x1, y1) = coords[(i + 1) % coords.len()]; // Wrap around to connect the last point to the first
            self.draw_line_segment((x0, y0), (x1, y1), Rgb([0, 0, 255]));
        }
    }

    // Draws a random point
    pub fn draw_random_point(&mut self, p: &Polygon, img_width: u32, img_height: u32, point: Coord, color: Rgb<u8>) {
        let (scale_x, scale_y, min_x, min_y) = self.scale_x_and_y(&p);
        let mut x = ((point.x - min_x) * scale_x).round() as u32;
        let mut y = (img_height as f64 - (point.y - min_y) * scale_y).round() as u32;

        // Clamp x and y to ensure they are within bounds
        x = x.max(0).min(img_width - 1);
        y = y.max(0).min(img_height - 1);

        self.img.put_pixel(x, y, color);
    } 
}