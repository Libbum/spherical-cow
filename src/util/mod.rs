use nalgebra::{self, Point3, Vector3};
use nalgebra::core::Matrix;

/// Find the baycentric coordinates $(u,v)$ and distance $t$ given three triangle veriticies `vert0`, `vert1`, `vert2` and the
/// unit vector `dir` ($D$) in the direction of a ray $R(t) = O + tD$ such that $R(t)$ is equivalent to a point $T(u,v)$ on
/// triangle $T$. Here, $T(u,v) = (1-u-v)V_0+uV_1+uV_2$.
pub fn ray_intersection_distance(
    vert0: &Point3<f32>,
    vert1: &Point3<f32>,
    vert2: &Point3<f32>,
    dir: &Vector3<f32>,
) -> Result<f32, String> {
    let edge1 = vert1 - vert0;
    let edge2 = vert2 - vert0;

    let pvec = Matrix::cross(&dir, &edge2);
    let det = nalgebra::dot(&edge1, &pvec);
    if det > -1e-6 && det < 1e-6 {
        return Err("det approx 0".to_string());
    }
    let inv_det = 1. / det;

    let tvec = Point3::origin() - vert0;
    let u = nalgebra::dot(&tvec, &pvec) * inv_det;
    if u < 0. || u > 1. {
        return Err("u out of bounds".to_string());
    }
    let qvec = Matrix::cross(&tvec, &edge1);
    let v = nalgebra::dot(dir, &qvec) * inv_det;
    if v < 0. || u + v > 1. {
        return Err("v out of bounds".to_string());
    }
    // Our ray R(t) = O + tD can now be calculated
    let t = nalgebra::dot(&edge2, &qvec) * inv_det;
    Ok(t)
}
