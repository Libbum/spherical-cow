extern crate nalgebra;
extern crate rand;
extern crate spherical_cow;

use spherical_cow::shapes::Sphere;
use rand::distributions::Normal;
use nalgebra::Point3;

/// Pack spheres with radii normally distributed (with a mean of 3) into a spherical container of radius 20.
fn main() {
    let boundary = Sphere::new(Point3::origin(), 20.0);
    let mut sizes = Normal::new(3.0, 1.0);

    let spheres = spherical_cow::pack_spheres(boundary, &mut sizes);

    println!("Number of spheres: {}", spheres.len());
}
