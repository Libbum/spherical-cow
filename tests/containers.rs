extern crate nalgebra;
extern crate spherical_cow;

use spherical_cow::shapes::*;
use spherical_cow::Container;
use nalgebra::Point3;

#[test]
fn in_sphere() {
    let container = Sphere::new(Point3::origin(), 20.0);
    let contents = Sphere::new(Point3::new(10., 2.5, 3.8), 2.6);

    assert!(container.contains(&contents));
}

#[test]
fn outside_sphere() {
    let container = Sphere::new(Point3::origin(), 20.0);
    let contents = Sphere::new(Point3::new(10., 2.5, 3.8), 26.0);

    assert!(!container.contains(&contents));
}

#[test]
fn in_cuboid() {
    let container = Cuboid::new(5.0, 10.0, 5.0);
    let contents = Sphere::new(Point3::new(4.0, 9.0, 3.8), 1.0);

    assert!(container.contains(&contents));
}

#[test]
fn outside_cuboid() {
    let container = Cuboid::new(15.2, 8.0, 12.3);
    let contents = Sphere::new(Point3::new(10., 2.5, 3.8), 26.0);

    assert!(!container.contains(&contents));
}
