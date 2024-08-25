use fast_poisson::{Poisson, Poisson2D};
use geo::{BoundingRect, Contains, Within};
use geo::{Coord, LineString, Polygon};
use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};

use crate::forest_property::tree::Tree;
use crate::forest_property::tree_stand_data::TreeStrata;

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

fn generate_radius(mean_height: f32, divisor: f32) -> f32 {
    // Calculate the radius based on the mean height of the tree species
    mean_height / divisor
}
use rayon::prelude::*;
// Generates random trees for all strata within a polygon's minimum and maximum x and y coordinates
pub fn generate_random_trees(p: &Polygon, strata: &TreeStrata) -> Vec<Tree> {
    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&p);
    let width = max_x - min_x;
    let height = max_y - min_y;


    let trees = strata.tree_stratum.par_iter().map(|stratum| {

        
            let amount = stratum.stem_count.unwrap_or(0);
            let divisor = stratum.mean_height / 2.0; // Initial divisor for Poisson disc radius
            
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
            

        
        
            trees_strata
    }).flatten();

    trees.collect()
}
