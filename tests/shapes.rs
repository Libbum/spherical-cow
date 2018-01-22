extern crate spherical_cow;
extern crate nalgebra;

use spherical_cow::shapes::*;
use nalgebra::Point3;

#[test]
fn sphere_negative_radius_error() {
    assert!(Sphere::new(Point3::origin(), -20.0).is_err());
}

#[test]
fn sphere_create_ok() {
    assert!(Sphere::new(Point3::origin(), 20.0).is_ok());
}

#[test]
fn sphere_overlap() {
    let one = Sphere::new(Point3::origin(), 5.0).unwrap();
    let two = Sphere::new(Point3::new(1.0, 2.0, 0.8), 2.0).unwrap();
    assert!(two.overlaps(&one));
}

#[test]
fn sphere_display() {
    let sphere = Sphere::new(Point3::new(1.0, 2.0, 0.8), 2.0).unwrap();
    assert_eq!("[{1, 2, 0.8}, 2]".to_owned(), format!("{}", sphere));
}

#[test]
fn cuboid_negative_extent_error() {
    assert!(Cuboid::new(-20.0, 2.0, 2.9).is_err());
}

#[test]
fn cuboid_create_ok() {
    assert!(Cuboid::new(20.0, 2.0, 2.9).is_ok());
}

#[test]
fn cuboid_fromvec_negative_extent_error() {
    let extents = vec![0.2, 1.2, -2.8];
    assert!(Cuboid::from_vec(extents).is_err());
}

#[test]
fn cuboid_fromvec_ok() {
    let extents = vec![0.2, 1.2, 2.8];
    assert!(Cuboid::from_vec(extents).is_ok());
}
