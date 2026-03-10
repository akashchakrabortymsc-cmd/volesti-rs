use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::rngs::StdRng;
use rand::SeedableRng;
use volesti_rs::geometry::hpolytope::HPolytope;
use volesti_rs::geometry::point::Point;
use volesti_rs::samplers::ball_walk::{ball_walk, BallWalkConfig};
use volesti_rs::samplers::portfolio::sample_portfolios;

// ------------- Ball Walk on Hypercube -----------

fn bench_ball_walk_dimensions(c: &mut Criterion) {
    let mut group = c.benchmark_group("BallWalk_Hypercube");

    // TESTING IN DIFFERENT DIMENSIONS
    for dim in [10, 50, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(dim), dim, |b, &dim| {
            let poly = HPolytope::unit_hypercube(dim);
            let start = Point::new(vec![0.0; dim]);
            let config = BallWalkConfig {
                delta: None,
                burn_in: 0, // benchmark e burn-in off
                thinning: 1,
            };
            let mut rng = StdRng::seed_from_u64(42);

            b.iter(|| {
                ball_walk(
                    black_box(&poly),
                    black_box(&start),
                    black_box(100),
                    &config,
                    &mut rng,
                )
                .unwrap()
            });
        });
    }
    group.finish();
}

// ----------Ball Walk on Simplex -------------------

fn bench_ball_walk_simplex(c: &mut Criterion) {
    let mut group = c.benchmark_group("BallWalk_Simplex");

    for dim in [10, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(dim), dim, |b, &dim| {
            let poly = HPolytope::simplex(dim);
            let start_w = 1.0 / (dim as f64 + 1.0);
            let start = Point::new(vec![start_w; dim]);
            let config = BallWalkConfig {
                delta: None,
                burn_in: 0,
                thinning: 1,
            };
            let mut rng = StdRng::seed_from_u64(42);

            b.iter(|| {
                ball_walk(
                    black_box(&poly),
                    black_box(&start),
                    black_box(100),
                    &config,
                    &mut rng,
                )
                .unwrap()
            });
        });
    }
    group.finish();
}

// ------ Portfolio Sampling -----------

fn bench_portfolio_sampling(c: &mut Criterion) {
    let mut group = c.benchmark_group("Portfolio_Sampling");

    for n_assets in [10, 30, 50].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n_assets), n_assets, |b, &n| {
            let mut rng = StdRng::seed_from_u64(42);
            b.iter(|| sample_portfolios(black_box(n), black_box(100), &mut rng).unwrap());
        });
    }
    group.finish();
}

// ------- Single Step Throughput -----------

fn bench_samples_per_second(c: &mut Criterion) {
    let mut group = c.benchmark_group("Samples_Per_Second");

    // 50D hypercube — standard test case
    let poly = HPolytope::unit_hypercube(50);
    let start = Point::new(vec![0.0; 50]);
    let config = BallWalkConfig::default();

    group.bench_function("50D_1000_samples", |b| {
        let mut rng = StdRng::seed_from_u64(42);
        b.iter(|| {
            ball_walk(
                black_box(&poly),
                black_box(&start),
                black_box(1000),
                &config,
                &mut rng,
            )
            .unwrap()
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_ball_walk_dimensions,
    bench_ball_walk_simplex,
    bench_portfolio_sampling,
    bench_samples_per_second,
);
criterion_main!(benches);
