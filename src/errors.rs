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
    /// We choose a random valuc from the `set_f` vector. `rand` returns an option and we pop
    /// the value. If it's `None` this error is thrown. Due to the contstuction of the
    /// rest of the method, it's safe to say this is unreachable.
    NoneSetF,
    /// We choose a random valuc from the `front` vector. `rand` returns an option and we pop
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
                write!(f, "Returned none when choosing value from set f")
            }
            SphericalCowError::NoneFront => {
                write!(f, "Returned none when choosing value from front")
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
