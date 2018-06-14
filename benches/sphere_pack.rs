#[macro_use]
extern crate criterion;
extern crate nalgebra;
extern crate rand;
extern crate spherical_cow;

use criterion::Criterion;
use nalgebra::Point3;
use rand::distributions::Uniform;
use spherical_cow::shapes::Sphere;

fn criterion_benchmark(c: &mut Criterion) {
    c.sample_size(10).bench_function_over_inputs(
        "sphere [2, 2.5, 3, 3.5, 4], 0.1-0.2",
        |b, &&radius| {
            b.iter(|| {
                let boundary = Sphere::new(Point3::origin(), radius).unwrap();
                let mut sizes = Uniform::new(0.1, 0.2);

                let _spheres = spherical_cow::pack_spheres(&boundary, &mut sizes).unwrap();
            })
        },
        &[2., 2.5, 3., 3.5, 4.],
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
