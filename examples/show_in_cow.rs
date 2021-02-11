extern crate kiss3d;
extern crate nalgebra;
extern crate obj;
extern crate rand;
extern crate spherical_cow;

use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::window::Window;
use nalgebra::{Matrix, Point3, Translation3};
use obj::Obj;
use rand::distributions::Uniform;
use spherical_cow::util::{ray_intersection_count, trimesh_volume};
use spherical_cow::{Container, PackedVolume};
use std::path::Path;
use std::time::Instant;

#[derive(Clone)]
/// A simple struct that holds triangluar information obtained from an obj file.
struct CowBox {
    /// Triplet identfying $V_0$, $V_1$, $V_2$ vertex locations for each
    /// triangle in the object.
    triangles: Vec<(Point3<f32>, Point3<f32>, Point3<f32>)>,
}

impl Container for CowBox {
    /// Checks if sphere exists inside any given cow.
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
    println!("Loading cow object from disk...");
    let cow = Obj::load(&Path::new("examples/objects/cow.obj")).unwrap();
    let points: Vec<Point3<f32>> = cow
        .data
        .position
        .iter()
        .map(|pos| Point3::new(pos[0], pos[1], pos[2]))
        .collect();

    let mut triangles: Vec<(Point3<f32>, Point3<f32>, Point3<f32>)> = Vec::new();
    for object in cow.data.objects.iter() {
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

    // This is our bounding trimesh of a cow.
    let boundary = CowBox {
        triangles: triangles,
    };

    // Pack spheres with relatively small radii to fit in the legs and horns.
    // WARNING: This will take a while (as in 4 hours) to generate!!!
    println!("Packing spheres into cow...");
    let now = Instant::now();
    let mut sizes = Uniform::new(0.03, 0.05);
    let packed = PackedVolume::new(boundary, &mut sizes).unwrap();
    println!(
        "Done! Packing took {:.2} minutes.",
        now.elapsed().as_secs() / 60
    );

    println!("Volume Fraction: {:.2}%", packed.volume_fraction() * 100.);

    // Setup viewing environment.
    let eye = Point3::new(7., 7., 7.);
    let at = Point3::origin();

    let mut camera = ArcBall::new(eye, at);
    let mut window = Window::new_with_size("Spherical Cow: Spheres in a cow", 1920, 1080);
    window.set_light(Light::StickToCamera);

    // Populate spheres into scene.
    for sphere in packed.spheres.iter() {
        let mut scene_sphere = window.add_sphere(sphere.radius);
        scene_sphere.set_color(rand::random(), rand::random(), rand::random());
        scene_sphere.set_local_translation(Translation3::from(sphere.center.coords));
    }

    // Show result.
    while !window.should_close() {
        window.render_with_camera(&mut camera);
    }
}
