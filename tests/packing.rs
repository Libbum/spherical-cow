extern crate float_cmp;
extern crate nalgebra;
extern crate rand;
extern crate spherical_cow;

use float_cmp::ApproxEqUlps;
use nalgebra::core::Matrix3;
use nalgebra::Point3;
use rand::distributions::Uniform;
use spherical_cow::shapes::Sphere;
use spherical_cow::*;

#[test]
fn packed_volume_create_ok() {
    let boundary = Sphere::new(Point3::origin(), 2.0).unwrap();
    let mut sizes = Uniform::new(0.3, 0.5);

    assert!(PackedVolume::new(boundary, &mut sizes).is_ok());
}

#[test]
fn packed_volume_create_err() {
    // Since the boundary is so small, this will throw an Uncontained error
    // in `init_spheres`.
    let boundary = Sphere::new(Point3::origin(), 0.001).unwrap();
    let mut sizes = Uniform::new(0.3, 0.5);

    assert!(PackedVolume::new(boundary, &mut sizes).is_err());
}

#[test]
fn packed_volume_from_vec() {
    let boundary = Sphere::new(Point3::origin(), 2.0).unwrap();
    let spheres = vec![Sphere::new(Point3::origin(), 1.0).unwrap()];
    let packed = PackedVolume::from_vec(spheres, boundary);
    assert_eq!(packed.spheres.len(), 1);
}

#[test]
fn packing_statistics() {
    let boundary = Sphere::new(Point3::origin(), 1.1).unwrap();
    let spheres = vec![
        Sphere::new(Point3::new(-0.384903, -0.24830464, 0.0), 0.38490304).unwrap(),
        Sphere::new(Point3::new(0.47370654, -0.24830464, 0.0), 0.47370654).unwrap(),
        Sphere::new(Point3::new(-0.045371085, 0.50203013, 0.0), 0.43867713).unwrap(),
        Sphere::new(
            Point3::new(-0.03869664, -0.009303231, -0.63180053),
            0.374144,
        )
        .unwrap(),
        Sphere::new(
            Point3::new(-0.031967796, -0.6622924, -0.43089586),
            0.3090857,
        )
        .unwrap(),
    ];

    let packed = PackedVolume::from_vec(spheres, boundary);

    //NOTE: This are terrible statistics! This test is just for repreducability.
    assert!(packed.volume_fraction().approx_eq_ulps(&0.24766529, 2));
    assert!(packed.void_ratio().approx_eq_ulps(&3.0377078, 2));
    assert!(packed.coordination_number().approx_eq_ulps(&3.60, 2));
    assert!(
        packed.fabric_tensor()
            == Matrix3::new(
                0.30427963,
                0.00050500076,
                -0.0121679725,
                0.00050500076,
                0.2129935,
                -0.09924155,
                -0.0121679725,
                -0.09924155,
                0.48272687,
            )
    );
}
