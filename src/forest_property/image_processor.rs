use geo_types::{Coord, Polygon};
use image::{Rgb, RgbImage, ImageBuffer};
use geo::BoundingRect;

pub struct ImageProcessor {
    img: RgbImage,
    width: u32,
    height: u32,
}

