use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::rngs::StdRng;
use rand::SeedableRng;
use volesti_rs::geometry::hpolytope::HPolytope;
use volesti_rs::geometry::point::Point;
use volesti_rs::samplers::ball_walk::{ball_walk, BallWalkConfig};
use volesti_rs::samplers::portfolio::sample_portfolios;

// ── Ball Walk on Hypercube across dimensions ──────────────────
// Matches volesti's own benchmark: bench_ball_walk_hypercube
// Target: within 2x of C++ volesti speed (per project doc)
fn bench_ball_walk_dimensions(c: &mut Criterion) {
    let mut group = c.benchmark_group("BallWalk_Hypercube");

    // Reduce sample size for criterion — it runs many iterations
    group.sample_size(20);

    for &dim in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(dim),
            &dim,
            |b, &dim| {
                let poly = HPolytope::unit_hypercube(dim);
                let start = Point::new(vec![0.0; dim]);
                let config = BallWalkConfig {
                    delta: None,
                    // burn_in=0 here: we're measuring raw sampling throughput
                    // not statistical quality — burn-in tested separately
                    burn_in: 0,
                    thinning: 1,
                };

                b.iter(|| {
                    // Fresh RNG each iteration = independent measurements
                    let mut rng = StdRng::seed_from_u64(42);
                    ball_walk(
                        black_box(&poly),
                        black_box(&start),
                        black_box(100),
                        &config,
                        &mut rng,
                    )
                    .unwrap()
                });
            },
        );
    }
    group.finish();
}

// ── Ball Walk on Simplex ──────────────────────────────────────
// Simplex is harder than hypercube — more rejections near boundaries
fn bench_ball_walk_simplex(c: &mut Criterion) {
    let mut group = c.benchmark_group("BallWalk_Simplex");
    group.sample_size(20);

    for &dim in [10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(dim),
            &dim,
            |b, &dim| {
                let poly = HPolytope::simplex(dim);
                let start_w = 1.0 / (dim as f64 + 1.0);
                let start = Point::new(vec![start_w; dim]);
                let config = BallWalkConfig {
                    delta: None,
                    burn_in: 0,
                    thinning: 1,
                };

                b.iter(|| {
                    let mut rng = StdRng::seed_from_u64(42);
                    ball_walk(
                        black_box(&poly),
                        black_box(&start),
                        black_box(100),
                        &config,
                        &mut rng,
                    )
                    .unwrap()
                });
            },
        );
    }
    group.finish();
}

// ── Portfolio Sampling ────────────────────────────────────────
// Finance API benchmark — target: 10000 portfolios < 5 seconds
// (per project doc: bench_portfolio_sampler_full)
fn bench_portfolio_sampling(c: &mut Criterion) {
    let mut group = c.benchmark_group("Portfolio_Sampling");
    group.sample_size(20);

    for &n_assets in [10, 30, 50].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(n_assets),
            &n_assets,
            |b, &n| {
                b.iter(|| {
                    // Fresh RNG each iteration = independent measurements
                    let mut rng = StdRng::seed_from_u64(42);
                    sample_portfolios(black_box(n), black_box(100), &mut rng).unwrap()
                });
            },
        );
    }
    group.finish();
}

// ── Throughput: samples per second at fixed dimension ────────
// This is the number that goes in your README and proposal.
// volesti target: within 2x of C++ on 50D hypercube
fn bench_throughput_50d(c: &mut Criterion) {
    let mut group = c.benchmark_group("Throughput");
    group.sample_size(20);

    let poly = HPolytope::unit_hypercube(50);
    let start = Point::new(vec![0.0; 50]);
    let config = BallWalkConfig {
        delta: None,
        burn_in: 0,
        thinning: 1,
    };

    // 1000 samples — matches project doc target
    group.bench_function("50D_hypercube_1000_samples", |b| {
        b.iter(|| {
            let mut rng = StdRng::seed_from_u64(42);
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

    // 100D hypercube — high dimension target
    let poly_100 = HPolytope::unit_hypercube(100);
    let start_100 = Point::new(vec![0.0; 100]);

    group.bench_function("100D_hypercube_1000_samples", |b| {
        b.iter(|| {
            let mut rng = StdRng::seed_from_u64(42);
            ball_walk(
                black_box(&poly_100),
                black_box(&start_100),
                black_box(1000),
                &config,
                &mut rng,
            )
            .unwrap()
        });
    });

    group.finish();
}

// ── Acceptance rate measurement ───────────────────────────────
// Measures how often proposals are accepted at different dimensions.
// Low acceptance = poor mixing = need better delta tuning.
// Not a criterion benchmark — just prints stats.
#[allow(dead_code)]
fn print_acceptance_rates() {
    for &dim in [10, 50, 100, 500].iter() {
        let poly = HPolytope::unit_hypercube(dim);
        let start = Point::new(vec![0.0; dim]);
        let config = BallWalkConfig {
            delta: None,
            burn_in: 0,
            thinning: 1,
        };
        let mut rng = StdRng::seed_from_u64(42);
        let samples = ball_walk(&poly, &start, 1000, &config, &mut rng).unwrap();
        println!("dim={}: {} samples returned", dim, samples.len());
    }
}

criterion_group!(
    benches,
    bench_ball_walk_dimensions,
    bench_ball_walk_simplex,
    bench_portfolio_sampling,
    bench_throughput_50d,
);
criterion_main!(benches);