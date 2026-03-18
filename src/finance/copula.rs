use crate::finance::portfolio::{sample_portfolios, PortfolioSamples};
use rand::Rng;

/// Copula grid — n×n matrix of probabilities
pub struct Copula {
    /// grid[i][j] = probability mass at cell (i,j)cargo
    pub grid: Vec<Vec<f64>>,
    pub num_slices: usize,
}

impl Copula {
    /// down_diagonal / up_diagonal
    /// > 1 means CRISIS
    pub fn crisis_indicator(&self, band_width: usize) -> f64 {
        let n = self.num_slices;
        let mut up_diag = 0.0; // normal market
        let mut down_diag = 0.0; // crisis

        for i in 0..n {
            for j in 0..n {
                // Up diagonal: low return + low volatility
                // (i + j) close to (n-1) → up diagonal
                let up_dist = ((i + j) as i64 - (n - 1) as i64).unsigned_abs() as usize;

                // Down diagonal: low return + high volatility
                // i close to j → down diagonal
                let down_dist = (i as i64 - j as i64).unsigned_abs() as usize;

                if up_dist <= band_width {
                    up_diag += self.grid[i][j];
                }
                if down_dist <= band_width {
                    down_diag += self.grid[i][j];
                }
            }
        }

        if up_diag < 1e-10 {
            return 0.0;
        }
        down_diag / up_diag
    }

    /// Indicator > 1 = CRISIS
    pub fn is_crisis(&self, band_width: usize) -> bool {
        self.crisis_indicator(band_width) > 1.0
    }
}

pub fn compute_copula(
    samples: &PortfolioSamples,
    returns_1: &[f64], // pl1 in C++
    returns_2: &[f64], // pl2 in C++
    num_slices: usize,
) -> Copula {
    let num = samples.n_samples;

    // C++: sum1 += p[j] * pl1[j]
    let scores_1: Vec<f64> = samples
        .weights
        .iter()
        .map(|w| w.iter().zip(returns_1.iter()).map(|(wi, ri)| wi * ri).sum())
        .collect();

    let scores_2: Vec<f64> = samples
        .weights
        .iter()
        .map(|w| w.iter().zip(returns_2.iter()).map(|(wi, ri)| wi * ri).sum())
        .collect();

    // C++: std::sort(vec1.begin(), vec1.end())
    let mut sorted_1 = scores_1.clone();
    let mut sorted_2 = scores_2.clone();
    sorted_1.sort_by(|a, b| a.partial_cmp(b).unwrap());
    sorted_2.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // C++: Zs1.push_back(vec1[floor(i * pos * num)])
    let pos = 1.0 / num_slices as f64;
    let boundaries_1: Vec<f64> = (1..num_slices)
        .map(|i| sorted_1[(i as f64 * pos * num as f64).floor() as usize])
        .collect();
    let boundaries_2: Vec<f64> = (1..num_slices)
        .map(|i| sorted_2[(i as f64 * pos * num as f64).floor() as usize])
        .collect();

    // Step 4: Grid fill koro
    // C++: Matrix[row][col]++
    let mut matrix = vec![vec![0usize; num_slices]; num_slices];

    for idx in 0..num {
        let s1 = scores_1[idx];
        let s2 = scores_2[idx];

        // Col find
        let col = boundaries_1
            .iter()
            .position(|&b| s1 < b)
            .unwrap_or(num_slices - 1);

        // Row find
        let row = boundaries_2
            .iter()
            .position(|&b| s2 < b)
            .unwrap_or(num_slices - 1);

        matrix[row][col] += 1;
    }

    // Step 5: Normalize
    // C++: pos_Matrix[i][j] = Matrix[i][j] / num
    let grid: Vec<Vec<f64>> = matrix
        .iter()
        .map(|row| row.iter().map(|&c| c as f64 / num as f64).collect())
        .collect();

    Copula { grid, num_slices }
}

/// High level API — detect crisis in a market
///
/// # Arguments
/// * `n_assets`  - number of assets
/// * `returns`   - current period returns
/// * `past_returns` - previous period returns  
/// * `rng`       - random number generator
pub fn detect_crisis<R: Rng>(
    n_assets: usize,
    returns: &[f64],
    past_returns: &[f64],
    rng: &mut R,
) -> Result<bool, crate::error::VolestiError> {
    // Step 1: Portfolio sample koro
    let samples = sample_portfolios(n_assets, 10_000, rng)?;

    // Step 2: Copula compute koro
    let copula = compute_copula(&samples, returns, past_returns, 100);

    // Step 3: Crisis check koro — paper e band = 10%
    let band = (100.0 * 0.1) as usize; // ±10%
    Ok(copula.is_crisis(band))
}
