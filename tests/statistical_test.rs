use rand::rngs::StdRng;
use rand::SeedableRng;
use nalgebra::{DMatrix, DVector};
use volesti_rs::geometry::hpolytope::HPolytope;
use volesti_rs::geometry::point::Point;
use volesti_rs::samplers::ball_walk::{ball_walk, BallWalkConfig};

// ── HELPER: 10D unit cube — same as C++ generate_cube(10) ────
fn unit_cube_10d() -> HPolytope {
    let n = 10usize;
    let mut a = DMatrix::zeros(2 * n, n);
    let b = DVector::from_element(2 * n, 1.0f64);
    for i in 0..n {
        a[(i, i)] = 1.0;
        a[(n + i, i)] = -1.0;
    }
    HPolytope::new(a, b)
}

// ── HELPER: 3D unit cube for simple tests ────────────────────
fn unit_cube_3d() -> HPolytope {
    let a = DMatrix::from_row_slice(6, 3, &[
         1.0,  0.0,  0.0,
        -1.0,  0.0,  0.0,
         0.0,  1.0,  0.0,
         0.0, -1.0,  0.0,
         0.0,  0.0,  1.0,
         0.0,  0.0, -1.0,
    ]);
    let b = DVector::from_element(6, 1.0);
    HPolytope::new(a, b)
}

// ── HELPER: Univariate PSRF (R-hat) ──────────────────────────
// Direct port of volesti's univariate_psrf logic.
// Splits one chain into 4 sub-chains, computes R-hat per dimension.
// R-hat < 1.1 means the chain has converged — volesti's exact threshold.
//
// Formula:
//   W = mean of within-chain variances
//   B = variance of chain means * n
//   R-hat = sqrt((W*(n-1)/n + B/n) / W)
fn univariate_psrf(samples: &[Point]) -> f64 {
    let n_total = samples.len();
    let dim = samples[0].coords.len();
    let n_chains = 4usize;
    let chain_len = n_total / n_chains;

    if chain_len < 2 {
        panic!("Not enough samples for PSRF: need at least 4*2=8");
    }

    let mut max_rhat = 0.0f64;

    for d in 0..dim {
        // Split into 4 sub-chains
        let chains: Vec<Vec<f64>> = (0..n_chains)
            .map(|c| {
                samples[c * chain_len..(c + 1) * chain_len]
                    .iter()
                    .map(|p| p.coords[d])
                    .collect()
            })
            .collect();

        let n = chain_len as f64;

        // Within-chain variance for each chain
        let chain_means: Vec<f64> = chains
            .iter()
            .map(|ch| ch.iter().sum::<f64>() / n)
            .collect();

        let within_vars: Vec<f64> = chains
            .iter()
            .zip(chain_means.iter())
            .map(|(ch, &mean)| {
                ch.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0)
            })
            .collect();

        // W = mean within-chain variance
        let w = within_vars.iter().sum::<f64>() / n_chains as f64;

        // B = between-chain variance * n
        let grand_mean = chain_means.iter().sum::<f64>() / n_chains as f64;
        let b = n * chain_means.iter()
            .map(|&m| (m - grand_mean).powi(2))
            .sum::<f64>() / (n_chains as f64 - 1.0);

        // Variance estimate
        let var_hat = (n - 1.0) / n * w + b / n;

        let r_hat = if w < 1e-10 {
            1.0 // degenerate case
        } else {
            (var_hat / w).sqrt()
        };

        if r_hat > max_rhat {
            max_rhat = r_hat;
        }
    }

    max_rhat
}

// ── TEST 1: All samples inside polytope ──────────────────────
// Fundamental membership test — must pass before any other test.
#[test]
fn test_all_samples_inside_cube() {
    let poly = unit_cube_3d();
    let start = Point::new(vec![0.0, 0.0, 0.0]);
    let config = BallWalkConfig {
        delta: Some(0.5),
        burn_in: 1000,
        thinning: 1,
    };
    let mut rng = StdRng::seed_from_u64(42);
    let samples = ball_walk(&poly, &start, 1000, &config, &mut rng).unwrap();

    for s in &samples {
        assert!(
            poly.contains(s).unwrap(),
            "Sample outside polytope: {:?}", s.coords
        );
    }
}

// ── TEST 2: PSRF convergence on 10D cube ─────────────────────
// Direct port of C++ sampling_test.cpp Ball Walk test.
// Uses same polytope (H-cube10), same threshold (R-hat < 1.1).
// walkL=10 thinning, 10000 samples, origin start.
#[test]
fn test_psrf_ball_walk_hcube10() {
    let poly = unit_cube_10d();
    let start = Point::new(vec![0.0; 10]);

    // walkL=10 thinning, 10000 samples — matches C++ exactly
    let config = BallWalkConfig {
        delta: Some(0.5),
        burn_in: 1000,
        thinning: 10,
    };
    let mut rng = StdRng::seed_from_u64(3); // seed=3 matches C++ mt19937 seed

    let samples = ball_walk(&poly, &start, 10000, &config, &mut rng).unwrap();

    let r_hat = univariate_psrf(&samples);
    println!("Ball Walk PSRF on H-cube10: R-hat = {:.4}", r_hat);

    // volesti's exact threshold
    assert!(
        r_hat < 1.1,
        "PSRF R-hat = {:.4} >= 1.1 — chain has not converged", r_hat
    );
}

// ── TEST 3: PSRF convergence on 3D simplex ───────────────────
// Verifies Ball Walk mixes correctly on the standard simplex.
// Important for portfolio sampling correctness.
#[test]
fn test_psrf_ball_walk_simplex3() {
    let n = 3usize;
    let poly = HPolytope::simplex(n);
    let start = Point::new(vec![1.0 / (n as f64 + 1.0); n]);

    let config = BallWalkConfig {
        delta: None, // auto-compute from inradius
        burn_in: 1000,
        thinning: 10,
    };
    let mut rng = StdRng::seed_from_u64(3);

    let samples = ball_walk(&poly, &start, 10000, &config, &mut rng).unwrap();

    let r_hat = univariate_psrf(&samples);
    println!("Ball Walk PSRF on Simplex-3: R-hat = {:.4}", r_hat);

    assert!(
        r_hat < 1.1,
        "PSRF R-hat = {:.4} >= 1.1 — chain not converged on simplex", r_hat
    );
}

// ── TEST 4: Sample count is exact ────────────────────────────
// Verifies the sampler returns exactly n_samples as requested.
#[test]
fn test_sample_count_exact() {
    let poly = unit_cube_3d();
    let start = Point::new(vec![0.0, 0.0, 0.0]);
    let config = BallWalkConfig {
        delta: Some(0.5),
        burn_in: 100,
        thinning: 1,
    };
    let mut rng = StdRng::seed_from_u64(42);

    for n in [1, 10, 100, 500] {
        let samples = ball_walk(&poly, &start, n, &config, &mut rng).unwrap();
        assert_eq!(samples.len(), n, "Expected {} samples, got {}", n, samples.len());
    }
}