extern crate nalgebra;
extern crate spherical_cow;

use spherical_cow::shapes::Sphere;
use spherical_cow::Container;
use nalgebra::Point3;

#[test]
fn sphere_in_sphere() {
    let container = Sphere::new(Point3::origin(), 20.0);
    let contents = Sphere::new(Point3::new(10., 2.5, 3.8), 2.6);

    assert!(container.contains(&contents));
}

#[test]
fn sphere_not_in_sphere() {
    let container = Sphere::new(Point3::origin(), 20.0);
    let contents = Sphere::new(Point3::new(10., 2.5, 3.8), 26.0);

    assert!(!container.contains(&contents));
}
