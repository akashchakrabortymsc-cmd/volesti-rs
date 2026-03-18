use crate::polytope::hpolytope::HPolytope;
use nalgebra::{DMatrix, DVector};

impl HPolytope {
    /// Unit hypercube [-1,1]^n
    /// Protita dimension e 2ta constraint: x[i] <= 1, -x[i] <= 1
    pub fn unit_hypercube(n: usize) -> Self {
        let mut a = DMatrix::zeros(2 * n, n);
        let b = DVector::from_element(2 * n, 1.0_f64);

        for i in 0..n {
            a[(i, i)] = 1.0; //  x[i] <= 1
            a[(n + i, i)] = -1.0; // -x[i] <= 1
        }

        HPolytope::new(a, b)
    }

    /// Standard simplex: x[i] >= 0, sum(x) <= 1
    pub fn simplex(n: usize) -> Self {
        let mut a = DMatrix::zeros(n + 1, n);
        let mut b = DVector::zeros(n + 1);

        for i in 0..n {
            a[(i, i)] = -1.0; // x[i] >= 0
        }

        for j in 0..n {
            a[(n, j)] = 1.0; // sum <= 1
        }
        b[n] = 1.0;

        HPolytope::new(a, b)
    }
}
