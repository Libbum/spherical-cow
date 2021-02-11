use crate::errors::SphericalCowError as Error;
use crate::Container;
use nalgebra::{distance, Point3};
use std::f32::consts::FRAC_PI_3;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
/// Constructs a sphere located at `center` in Euclidean space with a given `radius`.
pub struct Sphere {
    /// Central point in space where this sphere is located.
    pub center: Point3<f32>,
    /// Radius of current sphere.
    pub radius: f32,
}

impl Sphere {
    /// Creates a `new` sphere given the location of the spheres' `center` and its' `radius`.
    pub fn new(center: Point3<f32>, radius: f32) -> Result<Sphere, Error> {
        if radius <= 0.0 {
            Err(Error::NegativeRadius)
        } else {
            Ok(Sphere { center, radius })
        }
    }

    /// If the distance between the centers of two spheres is less than the sum of
    /// their radii, we can consider them to be overlapping. Will return `true` in this case.
    pub fn overlaps(&self, other: &Sphere) -> bool {
        distance(&self.center, &other.center) < self.radius + other.radius
    }

    /// Calculates the volume of this sphere in normalised units.
    fn volume(&self) -> f32 {
        4. * FRAC_PI_3 * self.radius.powi(3)
    }
}

impl Container for Sphere {
    /// Checks if sphere exists inside the current bounding sphere.
    fn contains(&self, sphere: &Sphere) -> bool {
        distance(&Point3::origin(), &sphere.center) + sphere.radius <= self.radius
    }
    /// Calculates the volume of this sphere in normalised units.
    fn volume(&self) -> f32 {
        self.volume()
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{{{}, {}, {}}}, {}]",
            self.center.coords.x, self.center.coords.y, self.center.coords.z, self.radius
        )
    }
}
