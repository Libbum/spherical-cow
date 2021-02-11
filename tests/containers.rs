extern crate float_cmp;
extern crate nalgebra;
extern crate obj;
extern crate spherical_cow;

use float_cmp::ApproxEqUlps;
use nalgebra::{Matrix, Point3};
use obj::Obj;
use spherical_cow::shapes::*;
use spherical_cow::util::{ray_intersection_count, trimesh_volume};
use spherical_cow::Container;
use std::f32::consts::PI;
use std::path::Path;

#[test]
fn in_sphere() {
    let container = Sphere::new(Point3::origin(), 20.0).unwrap();
    let contents = Sphere::new(Point3::new(10., 2.5, 3.8), 2.6).unwrap();

    assert!(container.contains(&contents));
}

#[test]
fn outside_sphere() {
    let container = Sphere::new(Point3::origin(), 20.0).unwrap();
    let contents = Sphere::new(Point3::new(10., 2.5, 3.8), 26.0).unwrap();

    assert!(!container.contains(&contents));
}

#[test]
fn sphere_volume() {
    let sphere = Sphere::new(Point3::new(10., 2.5, 3.8), 2.6).unwrap();

    assert!(sphere
        .volume()
        .approx_eq_ulps(&(4. / 3. * PI * 2.6_f32.powi(3)), 2));
}

#[test]
fn in_cuboid() {
    let container = Cuboid::new(5.0, 10.0, 5.0).unwrap();
    let contents = Sphere::new(Point3::new(4.0, 9.0, 3.8), 1.0).unwrap();

    assert!(container.contains(&contents));
}

#[test]
fn outside_cuboid() {
    let container = Cuboid::new(15.2, 8.0, 12.3).unwrap();
    let contents = Sphere::new(Point3::new(10., 2.5, 3.8), 26.0).unwrap();

    assert!(!container.contains(&contents));
}

#[test]
fn cuboid_volume() {
    let cuboid = Cuboid::new(15.2, 8.0, 12.3).unwrap();

    assert!(cuboid
        .volume()
        .approx_eq_ulps(&((2. * 15.2) * (2. * 8.) * (2. * 12.3)), 2));
}

struct Emerald {
    triangles: Vec<(Point3<f32>, Point3<f32>, Point3<f32>)>,
}

impl Emerald {
    fn build() -> Emerald {
        let emerald = Obj::load(&Path::new("examples/objects/emerald.obj")).unwrap();
        let points: Vec<Point3<f32>> = emerald
            .data
            .position
            .iter()
            .map(|pos| Point3::new(pos[0], pos[1], pos[2]))
            .collect();

        let mut triangles: Vec<(Point3<f32>, Point3<f32>, Point3<f32>)> = Vec::new();
        for object in emerald.data.objects.iter() {
            for group in object.groups.iter() {
                for poly in group.polys.iter() {
                    triangles.push((
                        points[poly.0[0].0],
                        points[poly.0[1].0],
                        points[poly.0[2].0],
                    ));
                }
            }
        }
        Emerald {
            triangles: triangles,
        }
    }
}

impl Container for Emerald {
    fn contains(&self, sphere: &spherical_cow::shapes::Sphere) -> bool {
        let is_even = |x: i32| x & 1 == 0;
        let o_dist = nalgebra::distance(&Point3::origin(), &sphere.center) + sphere.radius;
        let norm_dir = sphere.center.coords / Matrix::norm(&sphere.center.coords);
        let count = ray_intersection_count(&self.triangles, norm_dir, o_dist);
        is_even(count)
    }

    fn volume(&self) -> f32 {
        trimesh_volume(&self.triangles)
    }
}

#[test]
fn in_emerald() {
    // This test ultimately checks `utils::ray_intersection_count`
    let emerald = Emerald::build();
    let contents = Sphere::new(Point3::new(2.1, 1.5, 0.8), 2.0).unwrap();

    assert!(emerald.contains(&contents));
}

#[test]
fn outside_emerald() {
    // This test ultimately checks `utils::ray_intersection_count`
    let emerald = Emerald::build();
    let contents = Sphere::new(Point3::new(2.1, 6.5, 3.8), 1.0).unwrap();

    assert!(!emerald.contains(&contents));
}

#[test]
fn emerald_volume() {
    // This test ultimately checks `utils::trimesh_volume`
    let emerald = Emerald::build();

    assert!(emerald.volume().approx_eq_ulps(&2354.709, 2));
}
