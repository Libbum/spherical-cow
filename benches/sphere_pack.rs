#[macro_use]
extern crate criterion;
extern crate nalgebra;
extern crate rand;
extern crate spherical_cow;

use criterion::Criterion;
use spherical_cow::shapes::Sphere;
use rand::distributions::Range;
use nalgebra::Point3;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sphere 2, 0.1-0.2", |b| {
        b.iter(|| {
            let boundary = Sphere::new(Point3::origin(), 2.0).unwrap();
            let mut sizes = Range::new(0.1, 0.2);

            let _spheres = spherical_cow::pack_spheres(&boundary, &mut sizes).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
