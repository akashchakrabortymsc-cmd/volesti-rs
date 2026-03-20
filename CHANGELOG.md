# Changelog

All notable changes to volesti-rs are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning follows [Semantic Versioning](https://semver.org/).

---

## [Unreleased] — GSoC 2026 in progress

### Planned
- Hit-and-Run sampler (CDHR + RDHR)
- Billiard Walk sampler
- Boundary RDHR + Boundary CDHR
- `WalkType` enum — walk-agnostic API
- PSRF + ESS as public diagnostic API
- `feasible_point()` — Chebyshev center
- crates.io publication
- PyO3 Python bindings (stretch)

---

## [0.1.0] — 2026-03-21 — Pre-GSoC PoC

### Added

**Geometry**
- `HPolytope` struct — H-representation `Ax ≤ b`
  - `contains()` — membership oracle with early exit (53% speedup at 500D)
  - `normalize()` — row normalization for Billiard Walk reflection
  - `line_intersect_coord()` — chord computation for Hit-and-Run
  - `line_positive_intersect()` — first wall hit for Billiard Walk
- `unit_hypercube(n)` — standard test polytope
- `simplex(n)` — standard simplex for portfolio constraints
- `Point` struct with operator overloading (`+`, `-`, `*`, dot product)
- `VolestiError` enum — typed error handling via `thiserror`

**Samplers**
- `ball_walk()` — Ball Walk MCMC sampler
  - Auto-tuned delta: `4r / sqrt(n)` where r = inradius
  - Configurable burn-in (default 100), thinning (default 1)
  - Uniform ball sampling via Gaussian direction + volume scaling
  - PSRF R̂ = 1.03 on H-cube10 — matches C++ volesti threshold

**Finance API**
- `sample_portfolios()` — samples valid portfolio weight vectors
  from the standard simplex via Ball Walk
  - Weights non-negative, sum ≤ 1
  - `mean_weights()` — average portfolio across samples
  - `cross_sectional_score()` — rank-based return scoring
- `compute_copula()` — empirical copula density estimation
  - `crisis_indicator()` — tail dependence market stress metric
  - `detect_crisis()` — threshold-based crisis detection

**Testing — 18 tests, 0 failures**
- `tests/unit_tests.rs` — 9 tests: geometry, Ball Walk, portfolio,
  copula
- `tests/statistical_test.rs` — 4 tests: PSRF convergence on
  H-cube10 and Simplex-3, membership, sample count
- `tests/equivalence_test.rs` — 5 tests: KS distributional
  equivalence vs C++ volesti reference

**Infrastructure**
- Criterion benchmarks — Ball Walk at d=10, 50, 100, 500;
  portfolio sampling at n=10, 30, 50
- GitHub Actions CI — `cargo fmt` + `cargo clippy -D warnings`
  + `cargo test` on every push
- `rust-toolchain.toml` — pinned stable toolchain

### Performance
- 15 µs/sample at d=50 (H-cube, 1000 samples, release build)
- 200 ms / 100 samples at d=500 (after early-exit optimization)

### Dependencies
- `nalgebra = "0.33.0"` — pinned, pure Rust linear algebra
- `rand = "0.8.5"` — seeded RNG for reproducible tests
- `thiserror = "1.0"` — typed error handling
- `criterion = "0.5"` (dev) — benchmarking

### C++ References Verified Against
- `include/convex_bodies/hpolytope.h`
- `include/random_walks/uniform_ball_walk.hpp`
- `include/volume/copulas.h`
- `test/sampling_test.cpp`