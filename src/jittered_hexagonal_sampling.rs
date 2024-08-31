use std::collections::{HashMap, HashSet};

use geo::{algorithm::contains::Contains, prelude::*, Coord, Polygon};
use geo::{coord, line_string, Coordinate, LineString};
use rand::seq::SliceRandom;
use rand::Rng;

use crate::forest_property::tree::Tree;

const HEX_SIDE: f64 = 0.8660254037844386;

const VERTICES: [[f64; 2]; 6] = [
    [0.0, -1.0],
    [HEX_SIDE, -0.5],
    [HEX_SIDE, 0.5],
    [0.0, 1.0],
    [-HEX_SIDE, 0.5],
    [-HEX_SIDE, -0.5],
];

pub struct JitteredHexagonalGridSampling<R: Rng> {
    polygon: Polygon<f64>,
    r: f64,
    jitter: f64,
    jitter_radius: f64,
    rng: R,
    max_y: f64,
    max_x: f64,
    min_x: f64,
    min_y: f64,
    sample_points: Vec<Tree>,
    point_limit: usize,
    tree_species: u8,
    mean_height: f32
}

impl<R: Rng> JitteredHexagonalGridSampling<R> {
    pub fn new(rng: R, options: GridOptions) -> Self {
        let r = options.radius;
        let jitter = options.jitter.unwrap_or(0.6666);
        let jitter_radius = r * jitter;

        let bounding_rect = options.polygon.bounding_rect().unwrap();
        let min_x = bounding_rect.min().x;
        let min_y = bounding_rect.min().y;

        let max_y = ((bounding_rect.max().y - min_y) / r).ceil() as f64;
        let max_x = ((bounding_rect.max().x - min_x) / (r * 2.0 * HEX_SIDE)).ceil() as f64;



        Self {
            polygon: options.polygon,
            r,
            jitter,
            jitter_radius,
            rng,
            max_y,
            max_x,
            min_x,
            min_y,
            sample_points: Vec::new(),
            point_limit: options.point_limit.unwrap_or(1000000),
            tree_species: options.tree_species,
            mean_height: options.mean_height
        }
    }

    pub fn get_all_points(self) -> Vec<Tree> {
        self.sample_points
    }


    pub fn generate_all_points(&mut self) {

        let mut y_range: Vec<i32> =  (0..self.max_y as i32).collect::<Vec<i32>>();
        
        y_range.shuffle(&mut self.rng);
        let y_length = y_range.len()+1;

        let max_trees_y: u32 = (self.point_limit / y_length) as u32;
        let mut trees_count_y = 0;
        

        for (i, current_y) in y_range.into_iter().enumerate() {
            let cy = self.min_y + current_y as f64 * 1.5 * self.r;

            let y_odd = current_y as usize % 2 == 0;
     
            if trees_count_y > max_trees_y {
                trees_count_y = 0;
                continue;
            }

            let mut x_range: Vec<i32> =  (0..self.max_x as i32).collect::<Vec<i32>>();
        
            x_range.shuffle(&mut self.rng);
            let x_length:usize = x_range.len()+1;
    
            let max_trees_x: u32 = (max_trees_y as usize / x_length as usize) as u32;

            
            for (j, current_x) in x_range.into_iter().enumerate() {
                
                if (trees_count_y as usize / x_length as usize) as u32 > max_trees_x {
                    continue;
                }

                let cx = self.min_x
                    + (current_x as f64 * 2.0 + if y_odd { 1.0 } else { 0.0 })
                        * self.r
                        * HEX_SIDE;

                if !self.polygon.contains(&coord! {x: cx , y: cy }) {
              
                    continue;
                }

                let mut p = self.rng.gen_range(0.0..6.0);
 
                
                let q = self.rng.gen::<f64>();
                let tri = p as usize;
                p %= 1.0;

           

                let v0 = VERTICES[tri];
                let v1 = VERTICES[(tri + 1) % 6];

     


                let (p, q) = if p + q > 1.0 {
                    (1.0 - p, 1.0 - q)
                } else {
                    (p, q)
                };

                let tree = Tree::new(self.tree_species, (self.mean_height - 2.0) + (4.0 * p) as f32 , (
                    cx + (v0[0] * p + v1[0] * q) * self.jitter_radius,
                    cy + (v0[1] * p + v1[1] * q) * self.jitter_radius,
                    0.0
                ));


                trees_count_y += 1;

                self.sample_points.push(tree);
                
            }

            trees_count_y = 0;

        }
    }

    pub fn generate_trees(&mut self) -> Vec<Tree> {
        let mut y_range: Vec<i32> = (0..self.max_y as i32).collect::<Vec<i32>>();
        y_range.shuffle(&mut self.rng);
        let y_length = y_range.len() + 1;
    
        let max_trees_y: u32 = (self.point_limit / y_length) as u32;
    
        // Use iterators to generate trees
        y_range
            .into_iter()
            .flat_map(move |current_y| {
                let cy = self.min_y + current_y as f64 * 1.5 * self.r;
                let y_odd = current_y as usize % 2 == 0;
    
                let mut trees_count_y = 0;
                if trees_count_y > max_trees_y {
                    return vec![]; // Return an empty iterator if trees_count_y exceeds the limit
                }
    
                let mut x_range: Vec<i32> = (0..self.max_x as i32).collect::<Vec<i32>>();
                x_range.shuffle(&mut self.rng);
                let x_length: usize = x_range.len() + 1;
                let max_trees_x: u32 = (max_trees_y as usize / x_length as usize) as u32;
    
                let trees = x_range.into_iter().filter_map( |current_x| {
                    if (trees_count_y as usize / x_length as usize) as u32 > max_trees_x {
                        return None; // Skip to the next iteration if the x tree count exceeds the max
                    }
    
                    let cx = self.min_x
                        + (current_x as f64 * 2.0 + if y_odd { 1.0 } else { 0.0 })
                            * self.r
                            * HEX_SIDE;
    
                    if !self.polygon.contains(&coord! {x: cx , y: cy }) {
                        return None; // Skip to the next iteration if the point is outside the polygon
                    }
    
                    let mut p = self.rng.gen_range(0.0..6.0);
                    let q = self.rng.gen::<f64>();
                    let tri = p as usize;
                    p %= 1.0;
    
                    let v0 = VERTICES[tri];
                    let v1 = VERTICES[(tri + 1) % 6];
    
                    let (p, q) = if p + q > 1.0 {
                        (1.0 - p, 1.0 - q)
                    } else {
                        (p, q)
                    };
    
                    let tree = Tree::new(
                        self.tree_species,
                        (self.mean_height - 2.0) + (4.0 * p) as f32,
                        (
                            cx + (v0[0] * p + v1[0] * q) * self.jitter_radius,
                            cy + (v0[1] * p + v1[1] * q) * self.jitter_radius,
                            0.0,
                        ),
                    );
    
                    trees_count_y += 1;
                    Some(tree) // Yield the generated tree
                }).collect::<Vec<Tree>>();

                trees
            })
            .collect::<Vec<Tree>>() // Collect the iterator into a Vec<Tree>
    }

    pub fn fill(&mut self) -> Vec<Tree> {
        self.generate_all_points();
        self.sample_points.clone()
    

 /*            let mut sampled_points = self.sample_points.clone();
            sampled_points.shuffle(&mut self.rng);
            sampled_points.truncate(self.point_limit);
            sampled_points */
 
    }
}

pub struct GridOptions {
    pub polygon: Polygon<f64>,
    pub radius: f64,
    pub jitter: Option<f64>,
    pub point_limit: Option<usize>,
    pub tree_species: u8,
    pub mean_height: f32
}
