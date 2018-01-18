//! Spherical Cow: A hight volume fraction sphere packing library.
//!
//! # Usage
//!
//! Once written, this crate will be [on crates.io](https://crates.io/crates/spherical-cow) and can be
//! used by adding `spherical-cow` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! spherical-cow = "0.1"
//! ```
//!
//! And this in your crate root:
//!
//! ```rust
//! extern crate spherical_cow;
//! ```
//!
//! Currently this library requires the rust nightly compiler as at depends on the `remove_item` function of `Vec`.
//!
//! # Research
//!
//! The method implemented herein is an advancing front algorithm from
//! Valera *et al.*, [Computational Particle Mechanics 2, 161 (2015)](https://doi.org/10.1007/s40571-015-0045-8).

#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![feature(vec_remove_item)]

extern crate nalgebra;
extern crate rand;

/// Contains the sphere type along with a number of helper shapes which can be used as containters.
pub mod shapes;

use nalgebra::Point3;
use nalgebra::core::Matrix;
use rand::Rng;
use rand::distributions::IndependentSample;
use std::iter::repeat;
use shapes::Sphere;

pub trait Container {
    /// Checks if a sphere exists inside some bounding geometry.
    fn contains(&self, sphere: &Sphere) -> bool;
}


/// Creates three initial spheres that are tangent pairwise. The incenter of the triangle formed
/// by verticies located at the centers of each sphere is aligned at the origin.
fn init_spheres<C: Container>(radii: &[f32; 3], container: &C) -> Vec<Sphere> {
    let mut init = Vec::new();

    //            C (x,y)
    //            ^
    //           / \
    //        b /   \ a
    //         /     \
    //        /       \
    // A (0,0)--------- B (c,0)
    //            c

    // Sphere A can sit at the origin, sphere B extends outward along the x axis
    // sphere C extends outward along the y axis and complete the triangle
    let radius_a = radii[0];
    let radius_b = radii[1];
    let radius_c = radii[2];
    let distance_c = radius_a + radius_b;
    let distance_b = radius_a + radius_c;
    let distance_a = radius_c + radius_b;

    let x = (distance_b.powi(2) + distance_c.powi(2) - distance_a.powi(2)) / (2. * distance_c);
    let y = (distance_b.powi(2) - x.powi(2)).sqrt();

    // Find incenter
    let perimeter = distance_a + distance_b + distance_c;
    let incenter_x = (distance_b * distance_c + distance_c * x) / perimeter;
    let incenter_y = (distance_c * y) / perimeter;

    // Create spheres at positions shown in the diagram above, but offset such
    // that the incenter is now the origin. This offset attempts to minimise
    // bounding box issues in the sense that c may be close to or over the 
    // bb boundary already
    init.push(Sphere::new(
        Point3::new(-incenter_x, -incenter_y, 0.),
        radius_a,
    ));
    init.push(Sphere::new(
        Point3::new(distance_c - incenter_x, -incenter_y, 0.),
        radius_b,
    ));
    init.push(Sphere::new(
        Point3::new(x - incenter_x, y - incenter_y, 0.),
        radius_c,
    ));

    //TODO: error, not assert
    assert!(init.iter().all(|sphere| container.contains(&sphere)));
    init
}

/// Packs all habitat spheres to be as dense as possible.
/// Requires a `containter` ($G$) and a set of `all_radii` ($D$). This should be from some
/// distribution if this were a complete implementation, but we will just provide a set of known room sizes.
/// Additionally, the `container` can be arbitrary geometry, but we'll just provide a large sphere for
/// simplicity,
pub fn pack_spheres<C: Container, R: IndependentSample<f64>>(
    container: C,
    size_distribution: &mut R,
) -> Vec<Sphere> {
    // IndependentSample is already derrived for all distributions in `rand` with f64,
    // so we just downsample here instead of implementing traits on f32 for everything.
    let mut rng = rand::thread_rng();

    // Radii of three initial spheres, taken from the input distribution
    let init_radii: [f32; 3] = [
        size_distribution.ind_sample(&mut rng) as f32,
        size_distribution.ind_sample(&mut rng) as f32,
        size_distribution.ind_sample(&mut rng) as f32,
    ];

    // S := {s₁, s₂, s₃}
    let mut spheres = init_spheres(&init_radii, &container);

    // F := {s₁, s₂, s₃}
    let mut front = spheres.clone();

    // Radius of new sphere to be added to the current front, taken from the input distribution
    let mut new_radius = size_distribution.ind_sample(&mut rng) as f32;

    'outer: while !front.is_empty() {
        // s₀ := s(c₀, r₀) picked at random from F
        let curr_sphere = rng.choose(&front).unwrap().clone();
        // V := {s(c', r') ∈ S : d(c₀, c') ≤ r₀ + r' + 2r}
        let set_v = spheres
            .iter()
            .cloned()
            .filter(|s_dash| {
                s_dash != &curr_sphere &&
                    nalgebra::distance(&curr_sphere.center, &s_dash.center) <=
                        curr_sphere.radius + s_dash.radius + 2. * new_radius
            })
            .collect::<Vec<_>>();

        for (s_i, s_j) in pairs(&set_v) {
            let mut set_f = identify_f(&curr_sphere, s_i, s_j, &container, &set_v, new_radius);
            if !set_f.is_empty() {
                // Found at least one position to place the sphere,
                // choose one and move on
                let s_new = rng.choose(&set_f).unwrap();
                front.push(s_new.clone());
                spheres.push(s_new.clone());
                new_radius = size_distribution.ind_sample(&mut rng) as f32;
                continue 'outer;
            }
        }
        // NOTE: his is a nightly function only
        front.remove_item(&curr_sphere);
    }
    spheres
}

/// $f$ is as a set of spheres (or the empty set) such that they have a known `radius`,
/// are in outer contact with `s_1`, `s_2` and `s_3` simultaneously, are completely
/// contained in `container` and do not overlap with any element of `set_v`.
/// The set f has at most two elements, because there exist at most two spheres with
/// `radius` in outer contact with `s_1`, `s_2` and `s_3` simultaneously.
fn identify_f<C: Container>(
    s_1: &Sphere,
    s_2: &Sphere,
    s_3: &Sphere,
    container: &C,
    set_v: &Vec<Sphere>,
    radius: f32,
) -> Vec<Sphere> {

    //The center points of s_1, s_2, s_3 are verticies of a tetrahedron,
    //and the distances d_1, d_2, d_3 can be defined as the distances from these points to
    //a fourth vertex s_4, whose coordinates x,y,z must be found. This satisfies the equations
    // (x-x_1)^2+(y-y_1)^2+(z-z_1)^2=d_1^2 (1)
    // (x-x_2)^2+(y-y_2)^2+(z-z_2)^2=d_2^2 (2)
    // (x-x_3)^2+(y-y_3)^2+(z-z_3)^2=d_3^2 (3)

    //To solve this system, we subtract (1) from (2) & (3), to obtain the (linear) equations of two planes.
    //Coupling these planes to (1) we yield a quadratic system which takes the form
    // \vec u\cdot\vec r=a
    // \vec v\cdot\vec r=b
    // \vec r\cdot\vec r+\vec w\cdot\vec r=c

    // With a little bit of magic following https://axiomatic.neophilus.net/posts/2018-01-16-clustering-tangent-spheres.html
    // we can solve this system to identify r in the form
    // \vec r=\alpha\vec u+\beta\vec v+\gamma\vec t
    // Where \gamma has a quadratic solution identifying our two solutions.

    let distance_14 = s_1.radius + radius;
    let distance_24 = s_2.radius + radius;
    let distance_34 = s_3.radius + radius;

    let vector_u = s_1.center - s_2.center;
    let unitvector_u = vector_u / nalgebra::norm(&vector_u);
    let vector_v = s_1.center - s_3.center;
    let unitvector_v = vector_v / nalgebra::norm(&vector_v);
    let cross_uv = Matrix::cross(&vector_u, &vector_v);
    let unitvector_t = cross_uv / nalgebra::norm(&cross_uv);
    let vector_w = -2. * s_1.center.coords;

    let distance_a = (distance_24.powi(2) - distance_14.powi(2) + s_1.center.x.powi(2) +
                          s_1.center.y.powi(2) + s_1.center.z.powi(2) -
                          s_2.center.x.powi(2) -
                          s_2.center.y.powi(2) - s_2.center.z.powi(2)) /
        (2. * nalgebra::norm(&vector_u));
    let distance_b = (distance_34.powi(2) - distance_14.powi(2) + s_1.center.x.powi(2) +
                          s_1.center.y.powi(2) + s_1.center.z.powi(2) -
                          s_3.center.x.powi(2) -
                          s_3.center.y.powi(2) - s_3.center.z.powi(2)) /
        (2. * nalgebra::norm(&vector_v));
    let distance_c = distance_14.powi(2) - s_1.center.x.powi(2) - s_1.center.y.powi(2) -
        s_1.center.z.powi(2);

    let dot_uv = nalgebra::dot(&unitvector_u, &unitvector_v);
    let dot_wt = nalgebra::dot(&vector_w, &unitvector_t);
    let dot_uw = nalgebra::dot(&unitvector_u, &vector_w);
    let dot_vw = nalgebra::dot(&unitvector_v, &vector_w);

    let alpha = (distance_a - distance_b * dot_uv) / (1. - dot_uv.powi(2));
    let beta = (distance_b - distance_a * dot_uv) / (1. - dot_uv.powi(2));
    let value_d = alpha.powi(2) + beta.powi(2) + 2. * alpha * beta * dot_uv + alpha * dot_uw +
        beta * dot_vw - distance_c;
    let gamma_pos = 0.5 * (-dot_wt + (dot_wt.powi(2) - 4. * value_d).sqrt());
    let gamma_neg = 0.5 * (-dot_wt - (dot_wt.powi(2) - 4. * value_d).sqrt());

    let s_4_positive = Sphere::new(
        Point3::from_coordinates(
            alpha * unitvector_u + beta * unitvector_v + gamma_pos * unitvector_t,
        ),
        radius,
    );
    let s_4_negative = Sphere::new(
        Point3::from_coordinates(
            alpha * unitvector_u + beta * unitvector_v + gamma_neg * unitvector_t,
        ),
        radius,
    );

    let mut f = Vec::new();
    // Make sure the spheres are bounded by the containing geometry and do not overlap any spheres in V
    if container.contains(&s_4_positive) && !set_v.iter().any(|v| v.overlaps(&s_4_positive)) {
        f.push(s_4_positive);
    }
    if container.contains(&s_4_negative) && !set_v.iter().any(|v| v.overlaps(&s_4_negative)) {
        f.push(s_4_negative);
    }
    f
}

/// Calculates all possible pairs of a `set` of values.
fn pairs(set: &[Sphere]) -> Vec<(&Sphere, &Sphere)> {
    let n = set.len();

    if n == 2 {
        let mut minimal = Vec::new();
        minimal.push((&set[0], &set[1]));
        minimal
    } else {
        let mut vec_pairs = Vec::new();
        if n > 2 {
            // 0..n - m, but m = 2 and rust is non inclusive with its for loops
            for k in 0..n - 1 {
                let subset = &set[k + 1..n];
                vec_pairs.append(&mut subset
                    .iter()
                    .zip(repeat(&set[k]).take(subset.len()))
                    .collect::<Vec<_>>());
            }
        }
        vec_pairs
    }
}
