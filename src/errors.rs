//! Handles any errors that could occur during packing.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SphericalCowError {
    NegativeRadius,
    Uncontained,
    NoneSetF,
    NoneFront,
}

impl fmt::Display for SphericalCowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SphericalCowError::NegativeRadius => write!(f, "Supplied radius is negative."),
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
            SphericalCowError::Uncontained => "outside bounding geometry",
            SphericalCowError::NoneSetF => "none from set f",
            SphericalCowError::NoneFront => "none from front",
        }
    }
}
