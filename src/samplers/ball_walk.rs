use crate::error::VolestiError;
use crate::polytope::hpolytope::HPolytope;
use crate::polytope::point::Point;
use rand::Rng;

/// Ball Walk configuration
pub struct BallWalkConfig {
    /// Step size (delta)
    /// None hole auto-compute hobe polytope theke
    pub delta: Option<f64>,

    /// Burn-in steps — prothome eta steps discard koro
    /// Chain converge korte time lage
    pub burn_in: usize,

    /// Thinning — protita k-th sample nao
    /// Autocorrelation komabe
    pub thinning: usize,
}

impl Default for BallWalkConfig {
    fn default() -> Self {
        BallWalkConfig {
            delta: None,  // auto-compute
            burn_in: 100, // 100 steps discard
            thinning: 1,  // protita step e sample
        }
    }
}

/// C++ er compute_delta() er exact port:
/// delta = (4 * inner_ball_radius) / sqrt(dimension)
fn compute_delta(polytope: &HPolytope) -> f64 {
    let n = polytope.dim() as f64;
    let r = polytope.inner_ball_radius();
    (4.0 * r) / n.sqrt()
}

/// Polytope er modhye ekta random point sample koro
/// Ball theke random displacement
/// C++ er GetPointInDsphere() er port
fn sample_from_ball<R: Rng>(dim: usize, delta: f64, rng: &mut R) -> Point {
    // n-dimensional Gaussian sample koro
    // tahole normalize kore delta scale koro
    // ei technique uniform ball sampling er standard method

    // Step 1: Gaussian vector
    let gaussian: Vec<f64> = (0..dim)
        .map(|_| {
            // Box-Muller transform diye standard normal
            let u1: f64 = rng.gen_range(1e-10..1.0);
            let u2: f64 = rng.gen_range(0.0..1.0);
            (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
        })
        .collect();

    // Step 2: Norm compute koro
    let norm: f64 = gaussian.iter().map(|x| x * x).sum::<f64>().sqrt();

    // Step 3: Uniform radius — r = delta * u^(1/n)
    let u: f64 = rng.gen::<f64>();
    let r = delta * u.powf(1.0 / dim as f64);

    // Step 4: Scale koro
    let coords: Vec<f64> = gaussian.iter().map(|x| x / norm * r).collect();

    Point::new(coords)
}

/// Main Ball Walk function
/// C++ er apply() er port
///
/// # Arguments
/// * `polytope` - H-Polytope (A*x <= b)
/// * `start`    - Starting point (polytope er bhitore thakte hobe)
/// * `n_samples`- Koto samples chai
/// * `config`   - BallWalkConfig (delta, burn_in, thinning)
/// * `rng`      - Random number generator
///
/// # Returns
/// Vec<Point> — sampled points
pub fn ball_walk<R: Rng>(
    polytope: &HPolytope,
    start: &Point,
    n_samples: usize,
    config: &BallWalkConfig,
    rng: &mut R,
) -> Result<Vec<Point>, VolestiError> {
    // Validation
    if n_samples == 0 {
        return Err(VolestiError::ZeroSamples);
    }

    if !polytope.contains(start)? {
        return Err(VolestiError::InfeasiblePolytope);
    }

    // Delta compute koro
    let delta = config.delta.unwrap_or_else(|| compute_delta(polytope));

    let dim = polytope.dim();
    let mut current = start.clone();
    let mut samples = Vec::with_capacity(n_samples);

    // Burn-in — prothome eta steps discard koro
    for _ in 0..config.burn_in {
        let displacement = sample_from_ball(dim, delta, rng);
        // current + displacement
        let candidate = Point::new(
            current
                .coords
                .iter()
                .zip(displacement.coords.iter())
                .map(|(a, b)| a + b)
                .collect(),
        );
        if polytope.contains(&candidate)? {
            current = candidate; // accept
        }
        // reject hole current unchanged thake
    }

    // Actual sampling
    let mut step = 0usize;
    while samples.len() < n_samples {
        // Random displacement — Ball theke
        let displacement = sample_from_ball(dim, delta, rng);

        // Candidate point: current + displacement
        let candidate = Point::new(
            current
                .coords
                .iter()
                .zip(displacement.coords.iter())
                .map(|(a, b)| a + b)
                .collect(),
        );

        // Membership check — C++ er is_in() er equivalent
        if polytope.contains(&candidate)? {
            current = candidate; // accept
        }
        // reject hole current e thako

        step += 1;

        // Thinning — protita `thinning` step e 1ta sample nao
        if step % config.thinning == 0 {
            samples.push(current.clone());
        }
    }

    Ok(samples)
}
