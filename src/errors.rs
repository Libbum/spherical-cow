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

impl Error for SphericalCowError {}

#[test]
fn error_display_negative_radius() {
    use crate::shapes::Sphere;
    use nalgebra::Point3;

    let err = Sphere::new(Point3::origin(), -1.).unwrap_err();
    assert_eq!(format!("{}", err), format!("Supplied radius is negative."));
}

#[test]
fn error_display_negative_extent() {
    use crate::shapes::Cuboid;

    let err = Cuboid::new(1., 1., -1.).unwrap_err();
    assert_eq!(
        format!("{}", err),
        format!("A supplied half extent is negative.")
    );
}

#[test]
fn error_display_containment() {
    use crate::init_spheres;
    use crate::shapes::Sphere;
    use nalgebra::Point3;

    let container = Sphere::new(Point3::origin(), 0.1).unwrap();

    let err = init_spheres(&[10., 15., 20.], &container).unwrap_err();
    assert_eq!(
        format!("{}", err),
        format!("Sphere is not contained within bounding geometry.")
    );
}

#[test]
fn error_display_empty_values() {
    use crate::shapes::Sphere;
    use rand::prelude::SliceRandom;
    use rand::thread_rng;

    let empty_vec: Vec<Sphere> = Vec::new();
    let mut rng = thread_rng();

    let mut err = empty_vec
        .choose(&mut rng)
        .ok_or(SphericalCowError::NoneSetF)
        .unwrap_err();
    assert_eq!(
        format!("{}", err),
        format!("Returned none when choosing value from set f.")
    );

    err = empty_vec
        .choose(&mut rng)
        .ok_or(SphericalCowError::NoneFront)
        .unwrap_err();
    assert_eq!(
        format!("{}", err),
        format!("Returned none when choosing value from front.")
    );
}
