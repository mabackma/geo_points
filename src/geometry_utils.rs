use crate::forest_property::tree_stand_data::TreeStrata;
use crate::forest_property::tree::Tree;
use crate::jittered_hexagonal_sampling::{GridOptions, JitteredHexagonalGridSampling};

use geo_types::{Coord, Polygon};
use rand::{seq::IteratorRandom, thread_rng};
use geo::{BoundingRect, Within};
use fast_poisson::Poisson2D;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use core::f32::consts::PI;

// Get minimum and maximum x and y coordinates of a polygon
pub fn get_min_max_coordinates(p: &Polygon<f64>) -> (f64, f64, f64, f64) {
    let rect = p.bounding_rect().unwrap();
    let min_x = rect.min().x;
    let max_x = rect.max().x;
    let min_y = rect.min().y;
    let max_y = rect.max().y;

    (min_x, max_x, min_y, max_y)
}
/* 
fn generate_radius(mean_height: f32, divisor: f32) -> f32 {
    // Calculate the radius based on the mean height of the tree species
    mean_height / divisor
}

// Generates random trees for all strata with jittered poisson disc sampling
pub fn generate_random_trees(p: &Polygon, strata: &TreeStrata) -> Vec<Tree> {
    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&p);
    let width = max_x - min_x;
    let height = max_y - min_y;

    let trees = strata.tree_stratum.par_iter().map(|stratum| {
        let amount = stratum.stem_count;
        let mut divisor = stratum.mean_height / 2.0; // Initial divisor for Poisson disc radius
        let mut generated_trees: Vec<Tree> = Vec::new();

        loop {
            let radius = generate_radius(stratum.mean_height, divisor);

            let trees_strata: Vec<Tree> = Poisson2D::new()
                .with_samples(10)
                .with_dimensions([width, height], radius.into())
                .iter()
                .filter_map(|pair: [f64; 2]| {
                    let point = Coord {
                        x: pair[0] + min_x,
                        y: pair[1] + min_y,
                    };

                    if point.is_within(p) {
                        return Some(Tree::new(
                            stratum.tree_species,
                            stratum.mean_height,
                            (point.x, point.y, 0.0),
                        ))
                    }

                    None
                })
                .into_iter()
                .choose_multiple(&mut thread_rng(), amount as usize);

            if trees_strata.len() < amount as usize {
                println!("\tNeeded {} trees for stratum {} Generated only {}", amount, stratum.tree_species, trees_strata.len());
                divisor += 1.0;
            } else {
                generated_trees = trees_strata;
                break;
            }
        }
        
        generated_trees
    }).flatten();

    trees.collect()
}
*/
fn generate_radius(total_stem_count: u32, area: f32) -> f32 {
    let total_trees = total_stem_count as f32 * area / 10000.0;

    let mut ratio_fix = 1.3;

    if total_trees < 250.0 {
        ratio_fix = ((total_trees / 250.0) * 0.6) + 1.3;
    }
    let square_to_circle_ratio = 1.273 / ratio_fix;

    let tree_needed_area = area / total_trees / square_to_circle_ratio;
    // Calculate the radius based on the mean height of the tree species
    (tree_needed_area / PI).sqrt()
}

// Generates random trees for all strata with jittered grid sampling
pub fn generate_random_trees(p: &Polygon, strata: &TreeStrata) -> Vec<Tree> {
    let total_stem_count = strata.tree_stratum.iter().fold(0, |mut acc: u32, f| {
        acc += f.stem_count;
        acc
    });

    let trees = strata
        .tree_stratum
        .par_iter()
        .map(|stratum| {
            let amount = stratum.stem_count;

            let radius = generate_radius(
                total_stem_count,
                stratum.basal_area
            );

            // Jittered Grid Version 2
            let rng = rand::thread_rng();
            let options = GridOptions {
                polygon: p.to_owned(),
                radius: (radius).into(),
                jitter: Some(0.6666),
                point_limit: Some(amount as usize),
            };

            let mut grid = JitteredHexagonalGridSampling::new(rng, options);

            let points =  grid.fill();

            if points.len() == 0 {
                println!("No trees generated for stratum with basal area {}, stem count {}, mean height {}", stratum.basal_area, stratum.stem_count, stratum.mean_height);
            }

            let trees_strata: Vec<Tree> = points.iter().map(|pair: &[f64; 2]| {
                Tree::new(
                    stratum.tree_species,
                    stratum.mean_height,
                    (pair[0], pair[1], 0.0),
                )
            }).collect();
            trees_strata
        })
        .flatten();

    trees.collect()
}

