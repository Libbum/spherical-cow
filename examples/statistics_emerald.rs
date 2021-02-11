extern crate nalgebra;
extern crate obj;
extern crate rand;
extern crate spherical_cow;

use nalgebra::{Matrix, Point3};
use obj::Obj;
use rand::distributions::Uniform;
use spherical_cow::util::{ray_intersection_count, trimesh_volume};
use spherical_cow::{Container, PackedVolume};
use std::path::Path;

#[derive(Clone)]
/// A simple struct that holds triangluar information obtained from an obj file.
struct Emerald {
    /// Triplet identfying $V_0$, $V_1$, $V_2$ vertex locations for each
    /// triangle in the object.
    triangles: Vec<(Point3<f32>, Point3<f32>, Point3<f32>)>,
}

impl Container for Emerald {
    /// Checks if sphere exists inside any given emerald.
    fn contains(&self, sphere: &spherical_cow::shapes::Sphere) -> bool {
        // We use a ray casting to identify the sphere inside our object:
        // shoot a ray from your point and see how many sides of the object it intersects.
        // If the number is even, your point is outside the polygon. If it's odd your, point is inside.
        let is_odd = |x: i32| x & 1 == 1;
        // Distance from origin to the sphere's center point plus its radius
        let o_dist = nalgebra::distance(&Point3::origin(), &sphere.center) + sphere.radius;
        // Unit vector in the direction of the sphere
        let norm_dir = sphere.center.coords / Matrix::norm(&sphere.center.coords);
        let count = ray_intersection_count(&self.triangles, norm_dir, o_dist);
        if is_odd(count) {
            // Sphere is outside, thus not contained
            false
        } else {
            true
        }
    }
    /// Finds the volume of our emerald.
    /// This method can be used for any given trimesh.
    fn volume(&self) -> f32 {
        trimesh_volume(&self.triangles)
    }
}

fn main() {
    // Load an object file from disk
    let emerald = Obj::load(&Path::new("examples/objects/emerald.obj")).unwrap();
    let points: Vec<Point3<f32>> = emerald
        .data
        .position
        .iter()
        .map(|pos| Point3::new(pos[0], pos[1], pos[2]))
        .collect();

    let mut indices: Vec<Point3<u32>> = Vec::new();
    let mut triangles: Vec<(Point3<f32>, Point3<f32>, Point3<f32>)> = Vec::new();
    for object in emerald.data.objects.iter() {
        for group in object.groups.iter() {
            for poly in group.polys.iter() {
                indices.push(Point3::new(
                    poly.0[0].0 as u32,
                    poly.0[1].0 as u32,
                    poly.0[2].0 as u32,
                ));
                triangles.push((
                    points[poly.0[0].0],
                    points[poly.0[1].0],
                    points[poly.0[2].0],
                ));
            }
        }
    }

    // This is our bounding mesh in the shape of an emerald.
    let boundary = Emerald {
        triangles: triangles,
    };

    // Pack spheres with radii between 0.3 and 0.5.
    let mut sizes = Uniform::new(0.3, 0.5);
    let packed = PackedVolume::new(boundary, &mut sizes).unwrap();

    println!("Volume Fraction: {:.2}%", packed.volume_fraction() * 100.);
    println!("Void Ratio         : {:.2}", packed.void_ratio());
    println!("Coordination number: {:.2}", packed.coordination_number());
    println!("Fabric Tensor      : {}", packed.fabric_tensor());
}
