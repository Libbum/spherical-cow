extern crate nalgebra;
extern crate rand;
extern crate serde_json;
extern crate spherical_cow;

use rand::distributions::Uniform;
use spherical_cow::shapes::Cuboid;
use spherical_cow::PackedVolume;
// NOTE: You must run this example with the "serde-1" feature enabled
// cargo run --release --features serde-1 --example serialize
fn main() {
    // We'll pack some large-ish spheres into a cuboid to show serialization
    // of both shape types, then serialize the resultant PackedVolume.
    let mut sizes = Uniform::new(0.5, 0.6);
    let cuboid = Cuboid::new(1.5, 2., 1.7).unwrap();

    let packed = PackedVolume::new(cuboid, &mut sizes).unwrap();
    let ser_packed = serde_json::to_string_pretty(&packed).unwrap();
    println!("Serialized PackedVolume:");
    println!("{}", ser_packed);
    println!("----------------------------------------------------");

    // Now, let's put it back together.
    let de_packed: PackedVolume<Cuboid> = serde_json::from_str(&ser_packed).unwrap();
    println!("Deserialized PackedVolume:");
    println!("{:?}", de_packed);
}
