# volesti-rs 🦀

A high-performance **Rust port** of [volesti](https://github.com/GeomScale/volesti) — a C++ library for volume approximation and MCMC sampling of convex polytopes — with a focus on **quantitative finance applications**.

> **GSoC 2025 Proposal Project** | Organization: [GeomScale](https://geomscale.github.io/)

---

## Why Rust?

GeomScale's current stack covers C++ (core), R (statistics), and Python (data science). **Rust fills a critical gap:**

| | C++ volesti | volesti-rs (this project) |
|---|---|---|
| Memory safety | ❌ Manual | ✅ Guaranteed |
| Python bindings | ❌ Complex | ✅ PyO3 (planned) |
| Package manager | ❌ None | ✅ Cargo |
| HFT / production use | ⚠️ Possible | ✅ Native |
| Finance APIs | ❌ None | ✅ Built-in |

---

---

## Mathematical Foundation

An **H-Polytope** is defined as:

```
P = { x ∈ ℝⁿ | A·x ≤ b }
```

Where:
- `A` is an `(m × n)` constraint matrix
- `b` is an `m`-dimensional bound vector
- `m` = number of constraints, `n` = dimension

**Ball Walk** algorithm:
```
Given current point x ∈ P, step size δ:
1. Sample z uniformly from Ball(0, δ)
2. Propose x' = x + z
3. If x' ∈ P → accept (move to x')
   Else      → reject (stay at x)
```

---

## Quick Start

```toml
# Cargo.toml
[dependencies]
volesti-rs = "0.1.0"
```

```rust
use volesti_rs::geometry::hpolytope::HPolytope;
use volesti_rs::geometry::point::Point;

fn main() {
    // Create a 10-dimensional unit hypercube [-1, 1]^10
    let polytope = HPolytope::unit_hypercube(10);
    
    // Check if origin is inside
    let origin = Point::new(vec![0.0; 10]);
    assert!(polytope.contains(&origin).unwrap());
    
    println!("Origin is inside the hypercube ✅");
}
```

---

## Project Structure

```
volesti-rs/
├── src/
│   ├── lib.rs                  ← Library entry point
│   ├── error.rs                ← Error types
│   ├── geometry/
│   │   ├── point.rs            ← Point struct (n-dimensional)
│   │   ├── hpolytope.rs        ← HPolytope (A*x ≤ b)
│   │   └── shapes.rs           ← Hypercube, Simplex constructors
│   └── samplers/
│       └── ball_walk.rs        ← Ball Walk (in progress)
├── tests/
│   └── unit_tests.rs           ← Correctness tests
├── benches/
│   └── benchmarks.rs           ← Performance benchmarks
└── examples/
    └── portfolio.rs            ← Finance API demo (coming soon)
```

---

## Relationship to C++ volesti

This project directly ports the following C++ components:

| C++ File | Rust Equivalent | Status |
|---|---|---|
| `convex_bodies/hpolytope.h` | `geometry/hpolytope.rs` | ✅ Done |
| `cartesian_geom/point.h` | `geometry/point.rs` | ✅ Done |
| `random_walks/uniform_ball_walk.hpp` | `samplers/ball_walk.rs` | 🔄 In Progress |
| `random_walks/uniform_cdhr_walk.hpp` | `samplers/hit_and_run.rs` | 📋 Planned |
| `random_walks/uniform_billiard_walk.hpp` | `samplers/billiard_walk.rs` | 📋 Planned |
| `volume/copulas.h` | `finance/copula.rs` | 📋 Planned |

---

## Finance Applications

### Portfolio Sampling
```rust
// Sample 10,000 valid portfolios in 50-asset universe
// subject to: weights sum to 1, each weight in [0, 1]
let samples = sample_portfolios(n_assets: 50, n_samples: 10_000);
```

### Crisis Detection
```rust
// Detect financial crises using geometric anomaly detection
// Based on: Bachelard, Chalkis et al. AISTATS 2023
let crises = detect_crisis(returns: &etf_data, window: 60);
// → ["2008-09-15..2009-03-15", "2020-02-20..2020-04-15"]
```

---

## References

- Bachelard, Chalkis, Fisikopoulos, Tsigaridas — [Randomized geometric tools for anomaly detection in stock markets](https://proceedings.mlr.press/v206/bachelard23a.html), AISTATS 2023
- Cales, Chalkis, Emiris, Fisikopoulos — [Practical volume computation and portfolio dependencies](https://drops.dagstuhl.de/opus/volltexte/2018/8732), SoCG 2018
- Chalkis, Fisikopoulos — [volesti: Volume Approximation and Sampling in R](https://journal.r-project.org/archive/2021/RJ-2021-077/index.html), R Journal 2021
- Emiris, Fisikopoulos — [Efficient random-walk methods for polytope volume](https://vissarion.github.io/publications/EF_socg14.pdf), SoCG 2014

---

## Author

**Aakash Chakraborty**
MScFE | GSoC 2026 Applicant — GeomScale

---

## License

Apache 2.0 — same as volesti
