use thiserror::Error;

#[derive(Debug, Error)]
pub enum VolestiError {
    // Polytope er modhye kono valid point nei
    #[error("Polytope has no interior point")]
    InfeasiblePolytope,

    // Point er dimension ar Polytope er dimension match korche na
    #[error("Dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },

    // 0 samples nite chai — allowed na
    #[error("Sampler requires at least 1 sample")]
    ZeroSamples,
}