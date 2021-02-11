//! Useful helper functions such as a fast ray casting method and volume finder for use with arbitrary shaped triangular meshes.

use nalgebra::{Matrix, Point3, Vector3};

/// Find the baycentric coordinates `(u,v)` and distance `t` given three triangle veriticies `vert0`, `vert1`, `vert2` and the
/// unit vector `dir` (`D`) in the direction of a ray `R(t) = O + tD` such that `R(t)` is equivalent to a point `T(u,v)` on
/// triangle `T`. Here, `T(u,v) = (1-u-v)V_0+uV_1+uV_2`. If distance `t` is less than the distance of our sphere from the
/// origin (`sphere_dist`), then add one to the count.
pub fn ray_intersection_count(
    triangles: &[(Point3<f32>, Point3<f32>, Point3<f32>)],
    dir: Vector3<f32>,
    o_dist: f32,
) -> i32 {
    triangles
        .iter()
        .map(|triangle| {
            let &(vert0, vert1, vert2) = triangle;
            let edge1 = vert1 - vert0;
            let edge2 = vert2 - vert0;
            let pvec = Matrix::cross(&dir, &edge2);
            let det = Matrix::dot(&edge1, &pvec);
            if det > -1e-6 && det < 1e-6 {
                return 0;
            }
            let inv_det = 1. / det;
            let tvec = Point3::origin() - vert0;
            let u = Matrix::dot(&tvec, &pvec) * inv_det;
            if u < 0. || u > 1. {
                return 0;
            }
            let qvec = Matrix::cross(&tvec, &edge1);
            let v = Matrix::dot(&dir, &qvec) * inv_det;
            if v < 0. || u + v > 1. {
                return 0;
            }
            // Our ray R(t) = O + tD can now be calculated
            let t = Matrix::dot(&edge2, &qvec) * inv_det;
            if t.is_sign_positive() && t < o_dist {
                1
            } else {
                0
            }
        })
        .sum()
}

/// Identify the volume of a trimesh which contains the cartesian origin (0, 0, 0).
/// Consider the tetrahedron created by any triangle and the origin OABC and sum thier volumes.
pub fn trimesh_volume(triangles: &[(Point3<f32>, Point3<f32>, Point3<f32>)]) -> f32 {
    let sixth = 1. / 6.;
    triangles
        .iter()
        .map(|triangle| {
            let &(a, b, c) = triangle;
            sixth
                * (-c.x * b.y * a.z + b.x * c.y * a.z + c.x * a.y * b.z
                    - a.x * c.y * b.z
                    - b.x * a.y * c.z
                    + a.x * b.y * c.z)
        })
        .sum()
}
