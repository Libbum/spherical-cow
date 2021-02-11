use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use nalgebra::Point3;
use rand::distributions::Uniform;
use spherical_cow::shapes::Sphere;

fn sphere_benchmark(c: &mut Criterion) {
    let mut sphere = c.benchmark_group("sphere 0.1-0.2");
    for radius in [2., 2.5, 3., 3.5, 4.].iter() {
        sphere.bench_with_input(BenchmarkId::from_parameter(radius), radius, |b, &radius| {
            b.iter(|| {
                let boundary = Sphere::new(Point3::origin(), radius).unwrap();
                let mut sizes = Uniform::new(0.1, 0.2);

                let _spheres = spherical_cow::pack_spheres(&boundary, &mut sizes).unwrap();
            });
        });
    }
    sphere.finish();
}

criterion_group!(benches, sphere_benchmark);
criterion_main!(benches);
