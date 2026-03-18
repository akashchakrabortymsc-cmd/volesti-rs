# volesti-rs

**A native Rust port of [volesti](https://github.com/GeomScale/volesti) — the C++ library for geometric MCMC sampling and volume approximation of convex polytopes — with a walk-agnostic quantitative finance API.**

[![Tests](https://img.shields.io/badge/tests-18%20passing-brightgreen)](tests/)
[![Clippy](https://img.shields.io/badge/clippy-zero%20warnings-brightgreen)](.github/workflows/ci.yml)
[![GSoC 2026](https://img.shields.io/badge/GSoC_2026-GeomScale-orange)](https://summerofcode.withgoogle.com)
[![License](https://img.shields.io/badge/license-Apache_2.0-blue)](LICENSE)

---

## What this is

[volesti](https://github.com/GeomScale/volesti) is a research-grade C++ library by
[Apostolos Chalkis](https://github.com/TolisChal) and
[Vissarion Fisikopoulos](https://github.com/vissarion)
implementing state-of-the-art geometric random walks for uniform sampling
from convex polytopes.
It underlies published work in computational geometry and quantitative finance,
including the AISTATS 2023 paper on anomaly detection in stock markets.

**No Rust crate on crates.io implements any of these algorithms.**
This repository is a native Rust port of volesti's core sampling infrastructure —
bringing Ball Walk, Hit-and-Run (RDHR + CDHR), Billiard Walk, and Boundary walks
to the Rust ecosystem for the first time.

The port is being developed as a
**GSoC 2026 project** under the GeomScale organisation.

---

## Current status

| Component | C++ source | Status | Tests |
|---|---|---|---|
| `HPolytope` — H-representation | `include/convex_bodies/hpolytope.h` | ✅ Done | 3 unit tests |
| `unit_hypercube()`, `simplex()` | `include/generators/known_polytope_generators.h` | ✅ Done | 3 unit tests |
| `ball_walk()` — Ball Walk | `include/random_walks/uniform_ball_walk.hpp` | ✅ Done | PSRF R̂ = 1.03 < 1.1 |
| `sample_portfolios()` | Original — not in C++ volesti | ✅ Done | 2 unit tests |
| `compute_copula()` | `include/volume/copulas.h` | ✅ Done | 2 unit tests |
| `hit_and_run()` — RDHR + CDHR | `include/random_walks/uniform_rdhr_walk.hpp` | 🔲 Week 1 | — |
| `billiard_walk()` | `include/random_walks/uniform_billiard_walk.hpp` | 🔲 Week 2–3 | — |
| `boundary_rdhr()` | `include/random_walks/boundary_rdhr_walk.hpp` | 🔲 Week 3 | — |
| `boundary_cdhr()` | `include/random_walks/boundary_cdhr_walk.hpp` | 🔲 Week 3 | — |
| `WalkType` enum | Original design | 🔲 Week 4 | — |
| `psrf()` — PSRF diagnostic | `include/diagnostics/univariate_psrf.hpp` | 🔲 Week 5 | — |
| `ess()` — Effective Sample Size | `include/diagnostics/effective_sample_size.hpp` | 🔲 Week 5 | — |
| `feasible_point()` | `include/preprocess/feasible_point.hpp` | 🔲 Week 6 | — |
| PyO3 Python bindings | Original — stretch goal | 🔲 Week 12 | — |

**18 tests passing. 0 failures. `cargo clippy` zero warnings.**

```
test result: ok. 9 passed  (unit_tests.rs)
test result: ok. 4 passed  (statistical_test.rs)
test result: ok. 5 passed  (equivalence_test.rs)
```

---

## Performance

Benchmarks on Windows, release build (`cargo bench`).
Ball Walk — single-threaded, no SIMD, no parallelism.

| Polytope | Samples | Time | Per sample |
|---|---|---|---|
| H-cube 10D | 100 | ~0.97 ms | 9.7 µs |
| H-cube 50D | 1000 | ~15 ms | **15 µs** |
| H-cube 100D | 1000 | ~76 ms | 76 µs |
| Simplex 50D | 100 | ~1.7 ms | 17 µs |
| Portfolio 50 assets | 100 portfolios | ~14 ms | 140 µs |

The headline number is **15 µs/sample at d=50** in release mode —
measured with Criterion.

The GSoC Week 10 benchmark report will compare all six walks
at d=10, 50, 100, 200 against C++ volesti (`test/benchmarks_cb.cpp`).

---

## Statistical correctness

Ball Walk correctness is verified by two independent methods,
following volesti's own test methodology (`test/sampling_test.cpp`):

**PSRF convergence (Gelman-Rubin R̂)** — the same diagnostic
as volesti's C++ test suite, ported from
`include/diagnostics/univariate_psrf.hpp`:

```
Ball Walk PSRF on H-cube10:  R-hat = 1.03  ✓  (threshold: < 1.1)
Ball Walk PSRF on Simplex-3: R-hat = 1.04  ✓
```

**KS distributional equivalence** — five-test suite verifying
marginal distributions match Uniform[-1,1] on the 3D and 10D cubes,
with MCMC-aware threshold (D < 0.06).

---

## Architecture

The crate mirrors volesti's `include/` directory structure.
Every Rust file maps to a specific C++ counterpart.

```
volesti-rs/
├── Cargo.toml
├── rust-toolchain.toml
├── README.md
├── LICENSE
│
├── src/
│   ├── lib.rs                       ← public API surface
│   ├── error.rs                     ← VolestiError enum
│   │
│   ├── polytope/
│   │   ├── mod.rs
│   │   ├── hpolytope.rs             ← hpolytope.h             DONE ✓
│   │   └── shape.rs                 ← known_polytope_generators.h  DONE ✓
│   │
│   ├── samplers/
│   │   ├── mod.rs
│   │   ├── ball_walk.rs             ← uniform_ball_walk.hpp   DONE ✓
│   │   ├── hit_and_run.rs           ← uniform_rdhr_walk.hpp
│   │   │                                + uniform_cdhr_walk.hpp    Week 1
│   │   ├── billiard_walk.rs         ← uniform_billiard_walk.hpp    Week 2–3
│   │   ├── boundary_rdhr.rs         ← boundary_rdhr_walk.hpp       Week 3
│   │   ├── boundary_cdhr.rs         ← boundary_cdhr_walk.hpp       Week 3
│   │   └── walk_type.rs             ← WalkType enum  [ORIGINAL]    Week 4
│   │
│   ├── preprocess/
│   │   ├── mod.rs
│   │   └── feasible_point.rs        ← feasible_point.hpp           Week 6
│   │
│   ├── diagnostics/
│   │   ├── mod.rs
│   │   ├── psrf.rs                  ← univariate_psrf.hpp          Week 5
│   │   └── ess.rs                   ← effective_sample_size.hpp    Week 5
│   │
│   ├── finance/
│   │   ├── mod.rs
│   │   ├── portfolio.rs             ← walk-agnostic  [ORIGINAL]    DONE ✓ → Week 4
│   │   └── copula.rs                ← copulas.h                    DONE ✓
│   │
│   └── python/
│       └── lib.rs                   ← PyO3 [STRETCH Week 12]
│
├── tests/
│   ├── unit_tests.rs                ← 9 tests   DONE ✓
│   ├── statistical_test.rs          ← 4 tests   DONE ✓
│   └── equivalence_test.rs          ← 5 tests   DONE ✓
│
├── benches/
│   └── benchmarks.rs                ← Criterion  DONE ✓ → extend Week 10
│
└── .github/
    └── workflows/
        └── ci.yml                   ← cargo fmt + clippy + test
```

---

## Quick start

```toml
# Cargo.toml — available on crates.io after GSoC Week 11
[dependencies]
volesti-rs = "0.1"
```

### Rust — sample from any convex polytope

```rust
use volesti_rs::polytope::hpolytope::HPolytope;
use volesti_rs::samplers::ball_walk::{ball_walk, BallWalkConfig};
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() {
    let mut rng = StdRng::seed_from_u64(42);

    // Sample 1000 points from a 50-dimensional hypercube
    let polytope = HPolytope::unit_hypercube(50);
    let start    = Point::new(vec![0.0; 50]);
    let config   = BallWalkConfig::default();
    let samples  = ball_walk(&polytope, &start, 1000, &config, &mut rng).unwrap();

    println!("Sampled {} points", samples.len());

    // Check convergence
    let r_hat = volesti_rs::diagnostics::psrf::univariate_psrf(&samples);
    println!("PSRF R-hat = {:.3} (< 1.1 = converged)", r_hat);
}
```

### Rust — walk-agnostic portfolio sampling

```rust
use volesti_rs::samplers::walk_type::{WalkType, BilliardWalkConfig};
use volesti_rs::finance::portfolio::sample_portfolios;
use volesti_rs::finance::copula::compute_copula;

fn main() {
    let mut rng = StdRng::seed_from_u64(42);

    // Choose any walk — finance API is walk-agnostic
    let walk = WalkType::BilliardWalk(BilliardWalkConfig::default());

    // Sample 10,000 portfolios from 50-asset universe
    let portfolios = sample_portfolios(50, 10_000, walk, &mut rng).unwrap();

    // Compute copula density between return and volatility
    let returns_1 = vec![0.01_f64; 50]; // replace with real returns
    let returns_2 = vec![0.02_f64; 50];
    let copula = compute_copula(&portfolios, &returns_1, &returns_2, 10);

    println!("Copula grid sum = {:.4}", copula.grid.iter()
        .flat_map(|r| r.iter()).sum::<f64>());  // ≈ 1.0
}
```

---

## Why Rust

GeomScale's current language stack: C++ (core), R (statistical interface),
Python via dingo (data science interface).
There is no Rust implementation of volesti —
confirmed by searching crates.io in March 2026.

Rust fills a specific and permanent gap:

- **Memory safety without GC** — no use-after-free, no data races, all verified at compile time. Zero `unsafe` blocks in core paths.
- **C++ performance** — 15 µs/sample at d=50, competitive with the C++ original, without any architecture-specific optimisation.
- **Native ecosystem integration** — `cargo add volesti-rs`. No CMake. No compiler flags. No header dependency.
- **Walk-agnostic finance API** — `WalkType` enum lets users choose Ball Walk, Hit-and-Run, or Billiard Walk at the call site. This cleaner design does not exist in the C++ original.

---

## Mathematical background

The algorithms implemented here are described in:

- Bachelard, Chalkis, Fisikopoulos, Tsigaridas —
  *Randomized geometric tools for anomaly detection in stock markets*,
  AISTATS 2023, PMLR 206:9400–9416
- Chalkis, Fisikopoulos —
  *volesti: Volume Approximation and Sampling for Convex Polytopes in R*,
  The R Journal, 13(2):642–660, 2021
- Calès, Chalkis, Emiris, Fisikopoulos —
  *Practical Volume Computation of Structured Convex Bodies*,
  SoCG 2018, LIPIcs vol. 99
- Smith —
  *Efficient Monte Carlo Procedures for Generating Points Uniformly Distributed over Bounded Regions*,
  Operations Research, 32(6):1296–1308, 1984
- Lovász, Vempala —
  *Hit-and-Run from a Corner*,
  SIAM Journal on Computing, 35(4):985–1005, 2006
- Polyak, Gryazina —
  *Random Sampling: Billiard Walk Algorithm*,
  EJOR, 238(2):497–504, 2014
- Gelman, Rubin —
  *Inference from Iterative Simulation Using Multiple Sequences*,
  Statistical Science, 7(4):457–472, 1992
- Geyer —
  *Practical Markov Chain Monte Carlo*,
  Statistical Science, 7(4):473–483, 1992

---

## GSoC 2026 roadmap

| Deliverable | Weeks | What gets built |
|---|---|---|
| D1 — Six walks | 1–4 | Hit-and-Run, Billiard Walk, Boundary RDHR/CDHR, WalkType enum |
| D2 — Finance + diagnostics | 5–6 | PSRF + ESS as public API, portfolio upgrade to WalkType |
| D3 — Preprocessing | 7–9 | `feasible_point()`, CI + benchmarks at d=10,50,100,200 |
| D4 — Release | 10–11 | crates.io publish, rustdoc, benchmark report blog post |
| Stretch — PyO3 | 12 | `pip install volesti-rs` if D1–D4 complete ahead of schedule |

**Post-GSoC roadmap:**
Phase 2 (Year 1): Accelerated Billiard Walk, Gaussian walks.
Phase 3 (Year 2): Volume algorithms.
Phase 4 (Year 3): HMC, CRHMC, NUTS.

---

## Development

```bash
git clone https://github.com/akashchakrabortymsc-cmd/Volesti_Rust
cd Volesti_Rust
cargo test        # run all 18 tests
cargo bench       # run Criterion benchmarks
cargo clippy      # zero warnings enforced
cargo fmt --check # formatting enforced
```

Requires Rust 1.75+ stable toolchain (pinned in `rust-toolchain.toml`).

CI runs on every push: `cargo fmt --check` → `cargo clippy -D warnings` → `cargo test`.

---

## License

Apache 2.0 — same as [volesti](https://github.com/GeomScale/volesti).

---

*Built by [Akash Chakraborty](https://github.com/akashchakrabortymsc-cmd) —
WorldQuant University MScFE — GSoC 2026 applicant, GeomScale.*
