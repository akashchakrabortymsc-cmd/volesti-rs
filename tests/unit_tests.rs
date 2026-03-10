use volesti_rs::geometry::hpolytope::HPolytope;
use volesti_rs::geometry::point::Point;
use volesti_rs::samplers::ball_walk::{ball_walk, BallWalkConfig};
use rand::SeedableRng;
use rand::rngs::StdRng;

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
    let mut rng = StdRng::seed_from_u64(42); // fixed seed = reproducible

    let samples = ball_walk(&poly, &start, 100, &config, &mut rng).unwrap();

    assert_eq!(samples.len(), 100);

    // Protita sample polytope er bhitore ache?
    for s in &samples {
        assert!(poly.contains(s).unwrap(),
            "Sample outside polytope: {:?}", s.coords);
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