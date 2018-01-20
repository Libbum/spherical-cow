extern crate kiss3d;
extern crate nalgebra;
extern crate rand;
extern crate spherical_cow;

use kiss3d::camera::ArcBall;
use kiss3d::window::Window;
use kiss3d::light::Light;
use spherical_cow::shapes::Cuboid;
use rand::distributions::Range;
use nalgebra::{Point3, Translation3, UnitQuaternion, Vector3};

fn main() {
    // Setup viewing environment.
    let eye = Point3::new(3.5, 3.5, 3.5);
    let at = Point3::origin();
    let mut camera = ArcBall::new(eye, at);

    let mut window = Window::new_with_size("Spherical Cow: Spheres in a cuboid", 1920, 1080);
    window.set_light(Light::StickToCamera);

    // Pack spheres with radii between 0.05 and 0.1 into a cube with a halfspace of 1.5.
    let boundary = Cuboid::new(1.5, 1.5, 1.5);
    let mut sizes = Range::new(0.05, 0.1);

    let spheres = spherical_cow::pack_spheres(&boundary, &mut sizes);

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
