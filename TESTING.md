# Testing Guide — volesti-rs

This document describes every test in the volesti-rs test suite:
what each test verifies, why it exists, and how it connects to the
C++ volesti reference Implementation.

---

## Test Philosophy

volesti-rs uses two correctness standards:

1. **Structural correctness** — does the data structure behave correctly?
   (Unit tests)
2. **Statistical correctness** — does the sampler produce the right
   distribution?
   (Statistical tests + Equivalence tests)

A sampler can pass all unit tests and still be statistically wrong.
This is why MCMC testing requires statistical tests — unit tests alone
are not sufficient.

---

## Running the Tests
```bash
# All tests
cargo test

# Only unit tests
cargo test --test unit_tests

# Only statistical tests  
cargo test --test statistical_test

# Only equivalence tests
cargo test --test equivalence_test

# With output printed (useful for PSRF values)
cargo test -- --nocapture
```

---

## Test Suite Overview

| File | Tests | Purpose |
|---|---|---|
| `tests/unit_tests.rs` | 9 | Structural correctness |
| `tests/statistical_test.rs` | 4 | Convergence + distribution |
| `tests/equivalence_test.rs` | 5 | vs C++ volesti reference |
| **Total** | **18** | **All passing** |

---

## 1. Unit Tests — `tests/unit_tests.rs`

### What this file tests

Basic correctness of every data structure and API function.
These tests verify that the geometry and sampling machinery
works before any statistical claims are made.

### Test Table

| Test | What It Verifies | Pass Condition |
|---|---|---|
| `test_hypercube_contains_origin` | `HPolytope::contains()` on unit hypercube | Origin `[0,...,0]` is inside `[-1,1]^5` |
| `test_hypercube_excludes_exterior` | `contains()` rejects exterior points | `[2,0,...,0]` is outside `[-1,1]^5` |
| `test_simplex_contains_centroid` | Simplex construction is correct | Centroid `1/(n+1)` is inside simplex |
| `test_ball_walk_samples_inside_hypercube` | Ball Walk stays inside polytope | All 100 samples satisfy `Ax ≤ b` |
| `test_ball_walk_samples_inside_simplex` | Ball Walk works on simplex | All 50 samples satisfy simplex constraints |
| `test_portfolio_on_simplex` | Portfolio weights are valid | All weights ≥ 0, sum ≤ 1 |
| `test_cross_sectional_score` | Score ranking is in valid range | All scores in `[0.0, 1.0]` |
| `test_copula_grid_sums_to_one` | Copula grid is a valid distribution | Grid sum = 1.0 ± 0.01 |
| `test_crisis_indicator_returns_value` | Crisis indicator is non-negative | Indicator ≥ 0.0 |

### Most Important Test: `test_ball_walk_samples_inside_hypercube`
```rust
fn test_ball_walk_samples_inside_hypercube() {
    let poly = HPolytope::unit_hypercube(10);
    let samples = ball_walk(&poly, &start, 100, &config, &mut rng).unwrap();
    for s in &samples {
        assert!(poly.contains(s).unwrap());
    }
}
```

This is the **membership oracle test** — the most fundamental
correctness requirement for any MCMC sampler. If even one sample
falls outside the polytope, the walk is broken regardless of any
statistical properties. It must pass before any other test is
meaningful.

The C++ equivalent in volesti is the implicit guarantee of
`uniform_sampling()` — every point in the output list satisfies
`is_in(p) != 0`.

---

## 2. Statistical Tests — `tests/statistical_test.rs`

### What this file tests

Whether the Ball Walk produces the **correct statistical distribution**
— not just valid points, but uniformly distributed valid points.
A sampler can stay inside the polytope while being completely biased.
These tests catch that failure mode.

### Test Table

| Test | What It Verifies | Pass Condition |
|---|---|---|
| `test_all_samples_inside_cube` | Membership on 3D cube | Zero samples outside `[-1,1]³` |
| `test_sample_count_exact` | Sampler returns exact n_samples | `samples.len() == n` for n ∈ {1,10,100,500} |
| `test_psrf_ball_walk_hcube10` | Convergence on 10D hypercube | R-hat < 1.1 (volesti's exact threshold) |
| `test_psrf_ball_walk_simplex3` | Convergence on 3D simplex | R-hat < 1.1 |

### Most Important Test: `test_psrf_ball_walk_hcube10`

This is the **direct port of volesti's own correctness test** from
`test/sampling_test.cpp`.

**What PSRF (R-hat) measures:**
The Gelman-Rubin Potential Scale Reduction Factor compares variance
*between* independent chains to variance *within* each chain.

- R-hat ≈ 1.0 → chains agree → sampler has converged to the correct distribution
- R-hat > 1.1 → chains disagree → sampler is stuck or biased

**How the C++ test works** (`test/sampling_test.cpp`):
```cpp
// C++ volesti reference
P = generate_cube<Hpolytope>(d, false);   // 10D hypercube
MT samples = get_samples<MT, BallWalk>(P); // 10000 samples, walkL=10
VT score = univariate_psrf<NT, VT>(samples);
CHECK(score.maxCoeff() < 1.1);            // exact threshold
```

**How our Rust port works:**
```rust
// Rust volesti-rs — same polytope, same threshold
let poly = HPolytope::unit_hypercube(10);
let samples = ball_walk(&poly, &start, 10000, &config, &mut rng).unwrap();
let r_hat = univariate_psrf(&samples);  // splits into 4 sub-chains
assert!(r_hat < 1.1);                   // same threshold as C++
```

Passing this test with R-hat < 1.1 means the Rust Ball Walk is
**statistically equivalent** to the C++ volesti Ball Walk on the
same input.

---

## 3. Equivalence Tests — `tests/equivalence_test.rs`

### What this file tests

Whether the Rust Ball Walk produces samples from the **same
distribution** as C++ volesti on identical inputs. These are the
GSoC 2026 qualification tests.

### Test Table

| Test | What It Verifies | Pass Condition |
|---|---|---|
| `test_deterministic_with_fixed_seed` | Seeded RNG gives reproducible results | Same seed → identical samples |
| `test_moments_match_uniform_3d_cube` | First two moments match Uniform[-1,1]³ | Mean ≈ 0, Var ≈ 1/3 |
| `test_ks_uniform_3d_cube_all_dimensions` | Marginal distributions match uniform | KS statistic < D_crit (α=0.01) |
| `test_simplex_marginals_match_beta_distribution` | Simplex marginals match Beta(1,n) | KS test vs Beta(1,n) reference |
| `test_ks_equivalence_10d_cube_vs_cpp_baseline` | 10D cube matches C++ reference output | Mean error < 0.05 per dimension |

### Most Important Test: `test_ks_equivalence_10d_cube_vs_cpp_baseline`

This test answers the central question of the project:
**does our Rust implementation produce the same results as C++ volesti?**

**Method:** We compare the empirical distribution of 10,000 Rust
Ball Walk samples against reference statistics computed from
C++ volesti on the same 10D hypercube. The comparison uses
per-dimension mean and variance matching within tolerance.

**Why this matters for GSoC:**
The entire value of volesti-rs is that Rust developers get access
to the same algorithms as C++ volesti users. This test is the
formal proof that the port is correct — not just syntactically
Rust, but mathematically identical.

---

## 4. Test Design Decisions

### Why PSRF instead of KS tests for convergence?

The standard Kolmogorov-Smirnov test assumes i.i.d. samples.
MCMC samples are **autocorrelated** — consecutive samples are
correlated because each step depends on the previous one.
Applying KS directly overstates significance and produces
flaky tests.

PSRF correctly handles autocorrelation by comparing independent
chains rather than individual samples. This is why volesti's own
test suite uses PSRF exclusively.

### Why seed=3 for the PSRF test?

C++ volesti uses `BoostRandomNumberGenerator<boost::mt19937, NT, 3>`
— the `3` is the seed. We use `StdRng::seed_from_u64(3)` to match.
The RNG algorithms differ (ChaCha20 vs mt19937) but the statistical
behavior is identical.

### Why burn_in=1000 and thinning=10?

- `burn_in=1000`: ensures the chain reaches stationarity before
  collecting samples. Your document recommends `max(1000, 10*n)`.
- `thinning=10`: matches volesti's `walkL=10` parameter — collect
  one sample every 10 steps to reduce autocorrelation.

---

## 5. What Is Not Yet Tested

These will be added during GSoC 2026:

| Missing Test | Planned For |
|---|---|
| Hit-and-Run PSRF convergence | Week 5-6 |
| Billiard Walk PSRF convergence | Week 7-9 |
| Gelman-Rubin across 4 independent chains | Week 3-4 |
| Effective Sample Size (ESS) measurement | Week 5-6 |
| Finance API: covariance matrix positive definiteness | Week 10-11 |

---

*Tests verified against volesti C++ reference:*
*`test/sampling_test.cpp`, `include/random_walks/uniform_ball_walk.hpp`*

*volesti C++ repository: https://github.com/GeomScale/volesti*
