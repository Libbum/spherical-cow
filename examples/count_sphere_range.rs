extern crate nalgebra;
extern crate rand;
extern crate spherical_cow;

use spherical_cow::shapes::Sphere;
use rand::distributions::Range;
use nalgebra::Point3;

fn main() {
    // Pack spheres with radii between 0.1 and 0.2 into a spherical container of radius 2.
    let boundary = Sphere::new(Point3::origin(), 2.0).unwrap();
    let mut sizes = Range::new(0.1, 0.2);

    let spheres = spherical_cow::pack_spheres(&boundary, &mut sizes).unwrap();

    println!("Number of spheres: {}", spheres.len());
}
