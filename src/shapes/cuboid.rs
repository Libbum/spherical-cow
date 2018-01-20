use shapes::Sphere;
use Container;

#[derive(PartialEq, Debug, Clone)]
/// Constructs a cuboid centered at the origin in Euclidean space.
pub struct Cuboid {
    /// Central point in space where this sphere is located.
    pub half_extents: Vec<f32>,
}

impl Cuboid {
    /// Creates a new box from its `half_extents`. Half-extents are the box half-width along each
    /// axis, all of which must be greater than 0.04.
    pub fn new(extent_x: f32, extent_y: f32, extent_z: f32) -> Cuboid {
        //TODO: Should also have an orientation vector so this can be arbitrarily rotated too.
        //TODO: Errors, not assertions
        assert!(extent_x >= 0.0);
        assert!(extent_y >= 0.0);
        assert!(extent_z >= 0.0);
        Cuboid { half_extents: vec![extent_x, extent_y, extent_z] }
    }
}

impl Container for Cuboid {
    /// Checks if sphere exists inside the current bounding box.
    fn contains(&self, sphere: &Sphere) -> bool {
        // Sphere's center point plus its radius must be less than all three half_extents
        self.half_extents
            .iter()
            .zip(sphere.center.coords.iter())
            .all(|(extent, sphere_extent)| {
                sphere_extent.abs() + sphere.radius <= *extent
            })
    }

    /// Calculates the volume of this cuboid in normalised units.
    fn volume(&self) -> f32 {
        self.half_extents.iter().map(|half_extent| 2. * half_extent).product()
    }
}
