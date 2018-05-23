extern crate nalgebra;
extern crate rand;
extern crate spherical_cow;

use spherical_cow::shapes::Sphere;
use spherical_cow::PackedVolume;
use rand::distributions::Uniform;
use nalgebra::Point3;

fn main() {
    // Pack spheres with radii between 0.05 and 0.1 into a spherical container of radius 2,
    // output quantitative analysis data.
    let boundary = Sphere::new(Point3::origin(), 2.0).unwrap();
    let mut sizes = Uniform::new(0.05, 0.1);

    let packed = PackedVolume::new(boundary, &mut sizes).unwrap();

    println!(
        "Volume Fraction    : {:.2}%",
        packed.volume_fraction() * 100.
    );
    println!("Void Ratio         : {:.2}", packed.void_ratio());
    println!("Coordination number: {:.2}", packed.coordination_number());
    println!("Fabric Tensor      : {}", packed.fabric_tensor());
}
