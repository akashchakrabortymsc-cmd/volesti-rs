use crate::error::VolestiError;
use crate::geometry::point::Point;
use nalgebra::{DMatrix, DVector};

/// H-Polytope: A*x <= b
#[derive(Debug, Clone)]
pub struct HPolytope {
    pub a: DMatrix<f64>, // constraint matrix (m x n)
    pub b: DVector<f64>, // constraint vector (m)
}

impl HPolytope {
    /// Creating  a new hpolytope with given A and b
    pub fn new(a: DMatrix<f64>, b: DVector<f64>) -> Self {
        assert_eq!(
            a.nrows(),
            b.len(),
            "A has {} rows but b has {} entries",
            a.nrows(),
            b.len()
        );
        HPolytope { a, b }
    }

    /// Dimension (n) — koto dimension e ache
    pub fn dim(&self) -> usize {
        self.a.ncols()
    }

    /// Constraints
    pub fn num_constraints(&self) -> usize {
        self.a.nrows()
    }

    /// Point IN polytope ? A*x <= b?
    pub fn contains(&self, point: &Point) -> Result<bool, VolestiError> {
        if point.dim() != self.dim() {
            return Err(VolestiError::DimensionMismatch {
                expected: self.dim(),
                got: point.dim(),
            });
        }

        // A*x compute
        let ax = &self.a * &point.coords;

        // Protita constraint check
        for i in 0..ax.len() {
            if ax[i] > self.b[i] + 1e-10 {
                return Ok(false);
            }
        }

        Ok(true) // andar
    }

    /// Protita row normalize koro: A[i] / ||A[i]||, b[i] / ||A[i]||
    pub fn normalize(&mut self) {
        for i in 0..self.a.nrows() {
            let row_norm = self.a.row(i).norm();
            if row_norm > 1e-10 {
                let mut row = self.a.row_mut(i);
                row /= row_norm;
                self.b[i] /= row_norm;
            }
        }
    }
    /// Simple version: b vector er minimum value
    /// (proper implementation: Chebyshev center via LP — later)
    pub fn inner_ball_radius(&self) -> f64 {
        self.b
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min)
            .max(0.1) // minimum 0.1 jate delta 0 na hoy
    }
}
