//! Handles any errors that could occur during packing.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
/// All errors thrown by the library
pub enum SphericalCowError {
    /// If a sphere is given a negative radius.
    NegativeRadius,
    /// If a cuboid is given a negative half extent.
    NegativeExtents,
    /// If a sphere is created but is not confined by the `Container`.
    /// This happens quite a lot and is generally handled silently. This error
    /// is only thrown by the `init_spheres` method. Usually this means the geometry
    /// of the container is perhaps not aligned to the origin, it is scaled too small,
    /// or the spheres you're attempting to pack are too large.
    Uncontained,
    /// We choose a random value from the `set_f` vector. `rand` returns an option and we pop
    /// the value. If it's `None` this error is thrown. Due to the contstuction of the
    /// rest of the method, it's safe to say this is unreachable.
    NoneSetF,
    /// We choose a random value from the `front` vector. `rand` returns an option and we pop
    /// the value. If it's `None` this error is thrown. Due to the contstuction of the
    /// rest of the method, it's safe to say this is unreachable.
    NoneFront,
}

impl fmt::Display for SphericalCowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SphericalCowError::NegativeRadius => write!(f, "Supplied radius is negative."),
            SphericalCowError::NegativeExtents => write!(f, "A supplied half extent is negative."),
            SphericalCowError::Uncontained => {
                write!(f, "Sphere is not contained within bounding geometry.")
            }
            SphericalCowError::NoneSetF => {
                write!(f, "Returned none when choosing value from set f.")
            }
            SphericalCowError::NoneFront => {
                write!(f, "Returned none when choosing value from front.")
            }
        }
    }
}

impl Error for SphericalCowError {
    fn description(&self) -> &str {
        match *self {
            SphericalCowError::NegativeRadius => "negative radius",
            SphericalCowError::NegativeExtents => "negative half extent",
            SphericalCowError::Uncontained => "outside bounding geometry",
            SphericalCowError::NoneSetF => "none from set f",
            SphericalCowError::NoneFront => "none from front",
        }
    }
}

#[test]
fn error_display_negative_radius() {
    use nalgebra::Point3;
    use shapes::Sphere;

    if let Err(err) = Sphere::new(Point3::origin(), -1.) {
        assert_eq!(format!("{} {}", err, err.description()), format!("Supplied radius is negative. negative radius"));
    } else {
        panic!("Sending a negative radius to the sphere creation function yielded an Ok result");
    }
}

#[test]
fn error_display_negative_extent() {
    use shapes::Cuboid;

    if let Err(err) = Cuboid::new(1., 1., -1.) {
        assert_eq!(format!("{} {}", err, err.description()), format!("A supplied half extent is negative. negative half extent"));
    } else {
        panic!("Sending a negative half extent to the cuboid creation function yielded an Ok result");
    }
}

#[test]
fn error_display_containment() {
    use nalgebra::Point3;
    use shapes::Sphere;
    use init_spheres;

    let container = Sphere::new(Point3::origin(), 0.1).unwrap();

    if let Err(err) = init_spheres(&[10., 15., 20.], &container) {
        assert_eq!(format!("{} {}", err, err.description()), format!("Sphere is not contained within bounding geometry. outside bounding geometry"));
    } else {
        panic!("Testing a sphere which is not contained in a containter yielded an Ok result");
    }
}

#[test]
fn error_display_empty_values() {
    use rand::{Rng, thread_rng};
    use shapes::Sphere;

    let empty_vec: Vec<Sphere> = Vec::new();
    let mut rng = thread_rng();

    if let Err(err) = rng.choose(&empty_vec).ok_or(SphericalCowError::NoneSetF) {
        assert_eq!(format!("{} {}", err, err.description()), format!("Returned none when choosing value from set f. none from set f"));
    } else {
        panic!("set f was empty when it should not have been");
    }

    if let Err(err) = rng.choose(&empty_vec).ok_or(SphericalCowError::NoneFront) {
        assert_eq!(format!("{} {}", err, err.description()), format!("Returned none when choosing value from front. none from front"));
    } else {
        panic!("front was empty when it should not have been");
    }
}
