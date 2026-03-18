use rand::rngs::StdRng;
use rand::SeedableRng;
use volesti_rs::polytope::hpolytope::HPolytope;
use volesti_rs::polytope::point::Point;
use volesti_rs::samplers::ball_walk::{ball_walk, BallWalkConfig};
use volesti_rs::finance::portfolio::sample_portfolios;

#[test]
fn test_hypercube_contains_origin() {
    let poly = HPolytope::unit_hypercube(5);
    let origin = Point::new(vec![0.0; 5]);
    assert!(poly.contains(&origin).unwrap());
}

#[test]
fn test_hypercube_excludes_exterior() {
    let poly = HPolytope::unit_hypercube(5);
    let outside = Point::new(vec![2.0, 0.0, 0.0, 0.0, 0.0]);
    assert!(!poly.contains(&outside).unwrap());
}

#[test]
fn test_simplex_contains_centroid() {
    let n = 4;
    let poly = HPolytope::simplex(n);
    let centroid = Point::new(vec![1.0 / (n as f64 + 1.0); n]);
    assert!(poly.contains(&centroid).unwrap());
}

#[test]
fn test_ball_walk_samples_inside_hypercube() {
    let poly = HPolytope::unit_hypercube(10);
    let start = Point::new(vec![0.0; 10]);
    let config = BallWalkConfig::default();
    let mut rng = StdRng::seed_from_u64(42);

    let samples = ball_walk(&poly, &start, 100, &config, &mut rng).unwrap();

    assert_eq!(samples.len(), 100);
    for s in &samples {
        assert!(
            poly.contains(s).unwrap(),
            "Sample outside polytope: {:?}",
            s.coords
        );
    }
}

#[test]
fn test_ball_walk_samples_inside_simplex() {
    let poly = HPolytope::simplex(5);
    let start = Point::new(vec![0.1; 5]);
    let config = BallWalkConfig::default();
    let mut rng = StdRng::seed_from_u64(123);

    let samples = ball_walk(&poly, &start, 50, &config, &mut rng).unwrap();

    for s in &samples {
        assert!(poly.contains(s).unwrap());
    }
}

#[test]
fn test_portfolio_on_simplex() {
    let mut rng = StdRng::seed_from_u64(42);
    let result = sample_portfolios(10, 100, &mut rng).unwrap();

    for portfolio in &result.weights {
        for &w in portfolio {
            assert!(w >= -1e-6, "Negative weight: {}", w);
        }
        let sum: f64 = portfolio.iter().copied().sum::<f64>();
        assert!(sum <= 1.0 + 1e-6, "Sum > 1: {}", sum);
    }
}

#[test]
fn test_cross_sectional_score() {
    let mut rng = StdRng::seed_from_u64(99);
    let result = sample_portfolios(5, 200, &mut rng).unwrap();
    let returns = vec![0.01, 0.02, -0.01, 0.03, 0.005];
    let scores = result.cross_sectional_score(&returns);

    for s in &scores {
        assert!(*s >= 0.0 && *s <= 1.0, "Score out of range: {}", s);
    }
}

use volesti_rs::finance::copula::compute_copula;

#[test]
fn test_copula_grid_sums_to_one() {
    let mut rng = StdRng::seed_from_u64(42);
    let samples = sample_portfolios(5, 1000, &mut rng).unwrap();

    let returns_1 = vec![0.01, 0.02, -0.01, 0.03, 0.005];
    let returns_2 = vec![0.02, 0.01, 0.03, -0.01, 0.015];

    let copula = compute_copula(&samples, &returns_1, &returns_2, 10);

    // Grid sum ≈ 1
    let total: f64 = copula.grid.iter().flat_map(|row| row.iter()).sum();
    assert!((total - 1.0).abs() < 0.01, "Grid sum = {}", total);
}

#[test]
fn test_crisis_indicator_returns_value() {
    let mut rng = StdRng::seed_from_u64(42);
    let samples = sample_portfolios(5, 1000, &mut rng).unwrap();

    let returns_1 = vec![0.01, 0.02, -0.01, 0.03, 0.005];
    let returns_2 = vec![0.02, 0.01, 0.03, -0.01, 0.015];

    let copula = compute_copula(&samples, &returns_1, &returns_2, 10);
    let indicator = copula.crisis_indicator(1);

    // Indicator >= 0
    assert!(indicator >= 0.0, "Negative indicator: {}", indicator);
    println!("Crisis indicator: {:.4}", indicator);
}
