use crate::forest_property::tree_stand_data::TreeStrata;
use crate::forest_property::tree::Tree;

use geo_types::{Coord, LineString, Polygon};
use rand::{thread_rng, Rng};
use geo::{Contains, BoundingRect};
use fast_poisson::Poisson2D;

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

pub fn generate_poisson_disc_points(p: &Polygon<f64>, radius: f64) -> Vec<Coord<f64>> {
    let (min_x, max_x, min_y, max_y) = get_min_max_coordinates(&p);
    let width = max_x - min_x;
    let height = max_y - min_y;
    
    let poisson = Poisson2D::new().with_dimensions([width, height], radius).generate();
    println!("Generated {} Poisson disc samples", poisson.len());

    let mut points = Vec::new();
    for sample in poisson.iter() {
        // Translate the Poisson disc sample back to the polygon's coordinate space
        let point = Coord { x: sample[0] + min_x, y: sample[1] + min_y };

        if p.contains(&geo::point!(x: point.x, y: point.y)) {
            points.push(point);
        }
    }

    println!("Generated {} points inside polygon", points.len()); // Print the number of points generated
    points
}

// Pick random points from a list of points
fn pick_random_points(points: &mut Vec<Coord<f64>>, amount: usize) -> Vec<Coord<f64>> {
    let mut rng = thread_rng();
    let mut random_points = Vec::new();
    let amount_to_pick = usize::min(amount, points.len());  // Clamp amount to points length

    for _ in 0..amount_to_pick {
        let index = rng.gen_range(0..points.len());
        random_points.push(points.remove(index));
    }

    random_points
}

fn generate_radius(mean_height: f64, divisor: f64) -> f64 {
    // Calculate the radius based on the mean height of the tree species
    mean_height / divisor
}

// Generates random trees for all strata within a polygon's minimum and maximum x and y coordinates
pub fn generate_random_trees(p: &Polygon, strata: &TreeStrata) -> Vec<Tree> {
    let mut trees = Vec::new();

    for stratum in strata.tree_stratum.iter() {
        let amount = stratum.stem_count.unwrap_or(0);
        let mut divisor = stratum.mean_height / 2.0; // Initial divisor for Poisson disc radius
        let mut random_points = Vec::new();

        println!("\nSpecies: {}, Mean Height: {}, Basal Area: {}, Stem count: {}", stratum.tree_species, stratum.mean_height, stratum.basal_area.unwrap_or(0.0), amount);
        
        // Loop until enough points are generated for the stratum
        loop {
            // Calculate the radius based on the mean height of the tree species
            let radius = generate_radius(stratum.mean_height, divisor);

            // Generate Poisson disc points within the polygon
            let mut poisson_disc_points = generate_poisson_disc_points(&p, radius);

            // Pick random points from the Poisson disc points based on the stem count
            random_points = pick_random_points(&mut poisson_disc_points, amount as usize);
            println!("Picked {} random points for species {}", random_points.len(), stratum.tree_species);

            if random_points.len() == amount as usize {
                break;
            } else {
                println!("Not enough points generated for species {}. Trying again...", stratum.tree_species);
                divisor += 1.0; // Increase the divisor to generate more points
            }
        }

        // Generate random trees for each stratum
        for point in random_points {
            let tree = Tree::new(stratum.tree_species, stratum.mean_height, (point.x, point.y, 0.0));
            trees.push(tree);
        }
    }

    trees
}
