extern crate kiss3d;
extern crate nalgebra;
extern crate rand;
extern crate spherical_cow;

use kiss3d::camera::ArcBall;
use kiss3d::window::Window;
use kiss3d::light::Light;
use spherical_cow::shapes::Sphere;
use rand::distributions::Range;
use nalgebra::{Point3, Translation3, UnitQuaternion, Vector3};

fn main() {
    // Setup viewing environment.
    let eye = Point3::new(3.5, 3.5, 3.5);
    let at = Point3::origin();
    let mut camera = ArcBall::new(eye, at);

    let mut window = Window::new_with_size("Spherical Cow: Spheres in a sphere", 1920, 1080);
    window.set_light(Light::StickToCamera);

    // Pack spheres with radii between 0.1 and 0.2 into a spherical container of radius 2.
    let boundary = Sphere::new(Point3::origin(), 2.0).unwrap();
    let mut sizes = Range::new(0.1, 0.2);

    let spheres = spherical_cow::pack_spheres(&boundary, &mut sizes).unwrap();

    // Populate spheres into scene.
    for sphere in spheres.iter() {
        let mut scene_sphere = window.add_sphere(sphere.radius);
        scene_sphere.set_color(rand::random(), rand::random(), rand::random());
        scene_sphere.set_local_translation(Translation3::from_vector(sphere.center.coords));
    }

    // Show result.
    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    while !window.should_close() {
        window.render_with_camera(&mut camera);
        window.scene_mut().prepend_to_local_rotation(&rot);
    }
}
