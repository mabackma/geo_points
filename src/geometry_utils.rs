use fast_poisson::{Poisson, Poisson2D};
use geo::{polygon, BoundingRect, Contains, Scale, Within};
use geo::{Coord, LineString, Polygon};
use geo_rasterize::BinaryBuilder;
use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};

use crate::forest_property::tree::Tree;
use crate::forest_property::tree_stand_data::TreeStrata;
use crate::jittered_hexagonal_sampling::{GridOptions, JitteredHexagonalGridSampling};

// Get minimum and maximum x and y coordinates of a polygon
pub fn get_min_max_coordinates(p: &Polygon<f64>) -> (f64, f64, f64, f64) {
    let rect = p.bounding_rect().unwrap();
    let min_x = rect.min().x;
    let max_x = rect.max().x;
    let min_y = rect.min().y;
    let max_y = rect.max().y;

    (min_x, max_x, min_y, max_y)
}

/* // Create a polygon from a string of coordinates
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
} */

/* pub fn generate_poisson_disc_points(p: &Polygon<f64>, radius: f64, amount: i64) -> Vec<Coord> {
    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&p);
    let width = max_x - min_x;
    let height = max_y - min_y;

    let poisson = Poisson2D::new().with_dimensions([width, height], radius);

    let filtered: Vec<Coord> =
        poisson
            .iter()
            .fold(vec![], |mut acc: Vec<Coord>, pair: [f64; 2]| {
                let point = Coord {
                    x: pair[0] + min_x,
                    y: pair[1] + min_y,
                };
                if p.contains(&point) {
                    acc.push(point);
                }

                acc
            });



    println!("Generated {} Poisson disc samples", filtered.len());
    filtered

}
 */
/* // Pick random points from a list of points
fn pick_random_points(points: &mut Vec<Coord<f64>>, amount: usize) -> Vec<Coord<f64>> {
    let mut rng = thread_rng();
    let mut random_points = Vec::new();
    let amount_to_pick = usize::min(amount, points.len()); // Clamp amount to points length

    for _ in 0..amount_to_pick {
        let index = rng.gen_range(0..points.len());
        random_points.push(points.remove(index));
    }

    random_points
} */

/*
   def lerp(self, start, end, current, minimum, maximum):
        """."""
        result = maximum
        if current < start:
            result = minimum
        elif current < end:
            result = ((current - start) / (end - start)) * \
                (maximum - minimum) + minimum
        return result



    def __tree_dist_from_stem(self, total_stem, _area, thinning=False):
        """."""
        total_trees = self.stem_to_hectare(total_stem, _area)

        if thinning:
            ratio_fix = 2
        else:
            ratio_fix = self.lerp(0, 250, total_trees, 1.3, 1.9)

        square_to_circle_ratio = 1.273 / ratio_fix

        tree_needed_area = _area / total_trees / square_to_circle_ratio
        return math.sqrt(tree_needed_area / math.pi)


@staticmethod
    def stem_to_hectare(stem, area):
        """Convert area value from m^2 to hectare."""
        return stem * area / 10000

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
    tree_needed_area.sqrt()
}
use rayon::prelude::*;
// use ndarray::{arr3, Axis};
// Generates random trees for all strata within a polygon's minimum and maximum x and y coordinates
pub fn generate_random_trees(p: &Polygon, strata: &TreeStrata) -> Vec<Tree> {
   /*  let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&p); */

/*     let width = max_x - min_x;
    let height = max_y - min_y; */

    let total_stem_count = strata.tree_stratum.iter().fold(0, |mut acc: u32, f| {
        acc += f.stem_count;
        acc
    });

    let trees = strata
        .tree_stratum
        .par_iter()
        .map(|stratum| {
            let amount = stratum.stem_count/* .unwrap_or(0) */;
  /*           let divisor = stratum.mean_height / 2.0; // Initial divisor for Poisson disc radius */

            let radius = generate_radius(
                total_stem_count,
                stratum.basal_area,
            );

            // Poisson

            /* let trees_strata: Vec<Tree> = Poisson2D::new()
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
            .choose_multiple(&mut thread_rng(), amount as usize); */

            // Jittered Grid Version 1
            /*
            let rng = rand::thread_rng();
            let options = GridOptions {
                width,
                height,
                radius: radius.into(),
                jitter: Some(0.6666),
            };

            let mut grid = JitteredHexagonalGridSampling::new(rng, options);


            grid.fill();

            let points = grid.get_all_points();

            println!("generated points count: {}, needed: {}", points.len(), amount);

            let trees_strata: Vec<Tree> = grid.get_all_points()
            .iter()
            .filter_map(|pair: &[f64; 2]| {
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
            .choose_multiple(&mut thread_rng(), amount as usize); */

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

            println!(
                "generated points count: {}, needed: {}",
                points.len(),
                amount
            );

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
