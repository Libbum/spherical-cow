extern crate nalgebra;
extern crate rand;
extern crate spherical_cow;

use nalgebra::Point3;
use rand_distr::Normal;
use spherical_cow::shapes::Sphere;

/// Pack spheres with radii normally distributed (with a mean of 3) into a spherical container of radius 20.
fn main() {
    let boundary = Sphere::new(Point3::origin(), 20.0).unwrap();
    let mut sizes = Normal::new(3.0, 1.0).unwrap();

    let spheres = spherical_cow::pack_spheres(&boundary, &mut sizes).unwrap();

    println!("Number of spheres: {}", spheres.len());
}
