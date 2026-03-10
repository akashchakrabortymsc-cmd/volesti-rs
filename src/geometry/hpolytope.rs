use nalgebra::{DMatrix, DVector};
use crate::geometry::point::Point;
use crate::error::VolestiError;

/// H-Polytope: A*x <= b satisfy kore emon sob point er set
#[derive(Debug, Clone)]
pub struct HPolytope {
    pub a: DMatrix<f64>,  // constraint matrix (m x n)
    pub b: DVector<f64>,  // constraint vector (m)
}

impl HPolytope {
    /// Notun polytope banao
    pub fn new(a: DMatrix<f64>, b: DVector<f64>) -> Self {
        assert_eq!(
            a.nrows(), b.len(),
            "A has {} rows but b has {} entries",
            a.nrows(), b.len()
        );
        HPolytope { a, b }
    }

    /// Dimension (n) — koto dimension e ache
    pub fn dim(&self) -> usize {
        self.a.ncols()
    }

    /// Constraints er sonkha (m)
    pub fn num_constraints(&self) -> usize {
        self.a.nrows()
    }

    /// Point ta polytope er modhye ache? A*x <= b?
    pub fn contains(&self, point: &Point) -> Result<bool, VolestiError> {
        if point.dim() != self.dim() {
            return Err(VolestiError::DimensionMismatch {
                expected: self.dim(),
                got: point.dim(),
            });
        }

        // A*x compute koro
        let ax = &self.a * &point.coords;

        // Protita constraint check koro
        for i in 0..ax.len() {
            if ax[i] > self.b[i] + 1e-10 {
                return Ok(false); // baaire
            }
        }

        Ok(true) // andar
    }

    /// Protita row normalize koro — Billiard Walk er age lagbe
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
}