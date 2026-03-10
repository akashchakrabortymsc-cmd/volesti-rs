use crate::error::VolestiError;
use crate::geometry::hpolytope::HPolytope;
use crate::geometry::point::Point;
use crate::samplers::ball_walk::{ball_walk, BallWalkConfig};
use rand::Rng;

pub struct PortfolioSamples {
    pub weights: Vec<Vec<f64>>,
    pub n_assets: usize,
    pub n_samples: usize,
}

impl PortfolioSamples {
    /// i-th portfolio
    pub fn portfolio(&self, i: usize) -> &Vec<f64> {
        &self.weights[i]
    }

    /// Mean portfolio 
    pub fn mean_weights(&self) -> Vec<f64> {
        let mut mean = vec![0.0; self.n_assets];
        for p in &self.weights {
            for (i, w) in p.iter().enumerate() {
                mean[i] += w;
            }
        }
        mean.iter().map(|x| x / self.n_samples as f64).collect()
    }

    /// Cross-sectional score — paper er equation:
    /// rho = vol(Delta*) / vol(Delta)
    /// Delta* = portfolios with return <= target return
    /// Simple version: rank of portfolio among all samples
    pub fn cross_sectional_score(&self, returns: &[f64]) -> Vec<f64> {
        // Protita portfolio er return compute koro: R^T * w
        let portfolio_returns: Vec<f64> = self
            .weights
            .iter()
            .map(|w| w.iter().zip(returns.iter()).map(|(wi, ri)| wi * ri).sum())
            .collect();

        let n = portfolio_returns.len() as f64;

        // Rank / n = approximate cross-sectional score
        portfolio_returns
            .iter()
            .map(|&r| {
                let rank = portfolio_returns
                    .iter()
                    .filter(|&&other| other <= r)
                    .count();
                rank as f64 / n
            })
            .collect()
    }
}

/// Sample portfolios from simplex — paper er exact formulation
///
/// Paper (Section 2): "portfolio weights are non-negative and sum to 1"
/// = Standard simplex sampling via Ball Walk
///
/// # Arguments
/// * `n_assets`  - Number of assets (dimension of simplex)
/// * `n_samples` - Number of portfolios
/// * `rng`       - Random number generator
pub fn sample_portfolios<R: Rng>(
    n_assets: usize,
    n_samples: usize,
    rng: &mut R,
) -> Result<PortfolioSamples, VolestiError> {
    if n_samples == 0 {
        return Err(VolestiError::ZeroSamples);
    }

    // Paper er simplex: x[i] >= 0, sum(x) <= 1
    let polytope = HPolytope::simplex(n_assets);

    // Starting point — equal weight portfolio (1/(n+1), ...)
    // Simplex er centroid — always valid interior point
    let start_weight = 1.0 / (n_assets as f64 + 1.0);
    let start = Point::new(vec![start_weight; n_assets]);

    let config = BallWalkConfig {
        delta: None,
        burn_in: 500, // simplex e beshi burn-in
        thinning: 5,
    };

    let samples = ball_walk(&polytope, &start, n_samples, &config, rng)?;

    let weights: Vec<Vec<f64>> = samples
        .iter()
        .map(|p| p.coords.iter().cloned().collect())
        .collect();

    Ok(PortfolioSamples {
        weights,
        n_assets,
        n_samples,
    })
}
