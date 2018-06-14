extern crate nalgebra;
extern crate serde_json;
extern crate spherical_cow;

use nalgebra::Point3;
use spherical_cow::shapes::*;
use spherical_cow::PackedVolume;

#[test]
fn serialize_cuboid() {
    let cube = Cuboid::new(1., 1., 1.).unwrap();
    let ser_cube = serde_json::to_string(&cube).unwrap();
    assert_eq!(
        format!("{}", ser_cube),
        format!("{{\"half_extents\":[1.0,1.0,1.0]}}")
    );

    let de_cube: Cuboid = serde_json::from_str(&ser_cube).unwrap();
    assert_eq!(cube, de_cube);
}

#[test]
fn serialize_sphere() {
    let sphere = Sphere::new(Point3::origin(), 1.0).unwrap();
    let ser_sphere = serde_json::to_string(&sphere).unwrap();
    assert_eq!(
        format!("{}", ser_sphere),
        format!("{{\"center\":[0.0,0.0,0.0],\"radius\":1.0}}")
    );

    let de_sphere: Sphere = serde_json::from_str(&ser_sphere).unwrap();
    assert_eq!(sphere, de_sphere);
}

#[test]
fn serialize_packed_volume() {
    // We'll do this one the other way since we don't want to set
    // up a major set of seeded random machinery to generate a
    // PackedVolume every time.

    let packed = "{\"spheres\":[{\"center\":[-0.5947981,-0.33411378,0.0],\"radius\":0.5947981},{\"center\":[0.56496906,-0.33411378,0.0],\"radius\":0.564969},{\"center\":[0.014838338,0.6665208,0.0],\"radius\":0.5769212},{\"center\":[0.013118923,0.0011199871,0.859528],\"radius\":0.51006985},{\"center\":[0.013821319,0.0011798441,-0.89385486],\"radius\":0.5373749},{\"center\":[0.013104752,-1.0108846,-0.6260615],\"radius\":0.50951964},{\"center\":[0.013050228,-0.9927225,0.64155054],\"radius\":0.5073961}],\"container\":{\"half_extents\":[1.5,2.0,1.7]}}".to_string();
    println!("{}", packed);
    let de_packed: PackedVolume<Cuboid> = serde_json::from_str(&packed).unwrap();
    assert_eq!(de_packed.spheres.len(), 7);
    assert_eq!(de_packed.container.half_extents, [1.5, 2.0, 1.7]);

    let ser_packed = serde_json::to_string(&de_packed).unwrap();
    assert_eq!(format!("{:?}", ser_packed), format!("{:?}", packed));
}
