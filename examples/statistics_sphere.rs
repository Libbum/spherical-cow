extern crate nalgebra;
extern crate rand;
extern crate spherical_cow;

use spherical_cow::shapes::Sphere;
use spherical_cow::PackedVolume;
use rand::distributions::Range;
use nalgebra::Point3;

fn main() {
    // Pack spheres with radii between 0.05 and 0.1 into a spherical container of radius 2,
    // output quantitative analysis data.
    let boundary = Sphere::new(Point3::origin(), 2.0);
    let mut sizes = Range::new(0.05, 0.1);

    let packed = PackedVolume::new(boundary, &mut sizes);

    println!("Volume Fraction: {:.2}%", packed.volume_fraction() * 100.);
}
