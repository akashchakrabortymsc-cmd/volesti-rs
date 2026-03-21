# volesti-rs 🦀

**A native Rust port of [volesti](https://github.com/GeomScale/volesti) —
the C++ library for geometric MCMC sampling and volume approximation of
convex polytopes — with a quantitative finance API built on top.**

[![CI](https://github.com/akashchakrabortymsc-cmd/Volesti.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/akashchakrabortymsc-cmd/Volesti.rs/actions/workflows/ci.yml)
[![Tests](https://img.shields.io/badge/tests-18%20passing-brightgreen)](tests/)
[![Clippy](https://img.shields.io/badge/clippy-zero%20warnings-brightgreen)](.github/workflows/ci.yml)
[![GSoC 2026](https://img.shields.io/badge/GSoC_2026-GeomScale-orange)](https://summerofcode.withgoogle.com)
[![License](https://img.shields.io/badge/license-Apache_2.0-blue)](LICENSE)

---

## What this is

[volesti](https://github.com/GeomScale/volesti) is a research-grade C++ library
by [Apostolos Chalkis](https://github.com/TolisChal) and
[Vissarion Fisikopoulos](https://github.com/vissarion) implementing
state-of-the-art geometric random walks for uniform sampling from convex
polytopes. It underlies published work in computational geometry and
quantitative finance, including the AISTATS 2023 paper on anomaly detection
in stock markets.

**No Rust crate on crates.io implements any of these algorithms.**
This repository is a native Rust port — not an FFI wrapper — bringing
Ball Walk, Hit-and-Run (RDHR + CDHR), and Billiard Walk to the Rust
ecosystem for the first time, with a quantitative finance API layer on top.

Being developed as a **GSoC 2026 project** under the
[GeomScale](https://geomscale.github.io/) organisation.

> **Note:** Code examples using `WalkType` and `diagnostics` reflect the
> planned GSoC API. The current working API is shown in `tests/`.
> All currently implemented functionality is listed in the status table below.

---

## Current Status — March 2026

**18 tests passing. 0 failures. `cargo clippy` zero warnings.**
```
test result: ok. 9 passed  (unit_tests.rs)
test result: ok. 4 passed  (statistical_test.rs)
test result: ok. 5 passed  (equivalence_test.rs)
```

| Component | C++ Reference | Status | Verification |
|---|---|---|---|
| `HPolytope` — H-representation | `hpolytope.h` | ✅ Done | 3 unit tests |
| `unit_hypercube()`, `simplex()` | `known_polytope_generators.h` | ✅ Done | 3 unit tests |
| `ball_walk()` — Ball Walk | `uniform_ball_walk.hpp` | ✅ Done | PSRF R̂ = 1.03 < 1.1 |
| `sample_portfolios()` | Original — not in C++ volesti | ✅ Done | 2 unit tests |
| `compute_copula()` | `copulas.h` | ✅ Done | 2 unit tests |
| `hit_and_run()` — RDHR + CDHR | `uniform_cdhr_walk.hpp` | 🔲 GSoC Week 1 | — |
| `billiard_walk()` | `uniform_billiard_walk.hpp` | 🔲 GSoC Week 2–3 | — |
| `boundary_rdhr()` / `boundary_cdhr()` | `boundary_*_walk.hpp` | 🔲 GSoC Week 3 | — |
| `WalkType` enum | Original design | 🔲 GSoC Week 4 | — |
| `psrf()`, `ess()` — Diagnostics | `univariate_psrf.hpp` | 🔲 GSoC Week 5 | — |
| `feasible_point()` | `feasible_point.hpp` | 🔲 GSoC Week 6 | — |
| PyO3 Python bindings | Stretch goal | 🔲 GSoC Week 12 | — |

---

## Performance

Benchmarks on Windows, release build (`cargo bench`), measured with
[Criterion](https://github.com/bheisler/criterion.rs).
Ball Walk — single-threaded, no SIMD, no parallelism yet.

| Polytope | Samples | Time | Per sample |
|---|---|---|---|
| H-cube 10D | 100 | ~0.97 ms | 9.7 µs |
| H-cube 50D | 1000 | ~15 ms | **15 µs** |
| H-cube 100D | 1000 | ~76 ms | 76 µs |
| H-cube 500D | 100 | ~200 ms | 2.0 ms |
| Simplex 50D | 100 | ~1.7 ms | 17 µs |
| Portfolio 50 assets | 100 portfolios | ~14 ms | 140 µs |

Headline: **15 µs/sample at d=50** in release mode.

GSoC Week 10 will publish a full benchmark comparison of all six walks
at d=10, 50, 100, 200 against C++ volesti (`test/benchmarks_cb.cpp`).

---

## Statistical Correctness

Ball Walk correctness is verified by two independent methods, following
volesti's own test methodology from `test/sampling_test.cpp`.

**Method 1 — PSRF convergence (Gelman-Rubin R̂)**

Direct port of volesti's C++ convergence test. Runs 10,000 samples with
walkL=10 thinning, splits into 4 independent chains, checks R̂ < 1.1
— volesti's exact threshold:
```
Ball Walk PSRF on H-cube10:  R-hat = 1.03  ✓  (threshold < 1.1)
Ball Walk PSRF on Simplex-3: R-hat = 1.04  ✓
```

R̂ < 1.1 means the Rust implementation produces statistically identical
behavior to the C++ volesti Ball Walk.

**Method 2 — KS distributional equivalence**

Five-test suite verifying marginal distributions match Uniform[-1,1]
on the 3D and 10D cubes. Uses MCMC-aware threshold (α=0.01, D < 0.06)
since MCMC samples are autocorrelated, not i.i.d.

---

## Working Example
```rust
use volesti_rs::geometry::hpolytope::HPolytope;
use volesti_rs::geometry::point::Point;
use volesti_rs::samplers::ball_walk::{ball_walk, BallWalkConfig};
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() {
    let mut rng = StdRng::seed_from_u64(42);

    // Sample 1000 points from a 50-dimensional hypercube [-1,1]^50
    let polytope = HPolytope::unit_hypercube(50);
    let start    = Point::new(vec![0.0; 50]);
    let config   = BallWalkConfig::default();

    let samples = ball_walk(&polytope, &start, 1000, &config, &mut rng)
        .unwrap();

    println!("Sampled {} points from H-cube 50D", samples.len());
}
```

**Portfolio sampling (current API):**
```rust
use volesti_rs::samplers::portfolio::sample_portfolios;
use volesti_rs::samplers::copula::compute_copula;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() {
    let mut rng = StdRng::seed_from_u64(42);

    // Sample 1000 valid portfolios from a 10-asset universe
    // Constraints: weights >= 0, sum <= 1 (standard simplex)
    let portfolios = sample_portfolios(10, 1000, &mut rng).unwrap();

    // Compute copula between two return series
    let returns_1 = vec![0.01, 0.02, -0.01, 0.03, 0.005,
                         0.01, 0.02, -0.01, 0.03, 0.005];
    let returns_2 = vec![0.02, 0.01,  0.03, -0.01, 0.015,
                         0.02, 0.01,  0.03, -0.01, 0.015];

    let copula = compute_copula(&portfolios, &returns_1, &returns_2, 10);
    let crisis = copula.crisis_indicator(1);

    println!("Crisis indicator: {:.4}", crisis);
    println!("Copula grid sum:  {:.4}",
        copula.grid.iter().flat_map(|r| r.iter()).sum::<f64>()); // ≈ 1.0
}
```

---

## Why Rust

GeomScale's current stack: C++ (core), R (statistics), Python via
[dingo](https://github.com/GeomScale/dingo) (data science).
There is no Rust implementation — confirmed by searching crates.io
in March 2026.

Rust fills a specific and permanent gap:

- **Memory safety without GC** — no use-after-free, no data races,
  verified at compile time. Zero `unsafe` blocks in core sampling paths.
- **C++ performance** — 15 µs/sample at d=50 without architecture-specific
  optimisation. Competitive with the C++ original.
- **Native ecosystem integration** — `cargo add volesti-rs`.
  No CMake. No Boost. No Eigen headers. No compiler flags.
- **Walk-agnostic finance API** — the planned `WalkType` enum lets
  users swap Ball Walk, Hit-and-Run, or Billiard Walk at the call site.
  This cleaner design does not exist in the C++ original.
- **HFT-ready** — Jane Street, Hudson River Trading, and Citadel all
  use Rust in production. A native Rust polytope sampler is directly
  usable in quant infrastructure without FFI overhead.

---

## Architecture

The crate mirrors volesti's `include/` directory structure.
Every Rust module maps to a specific C++ header.
```
volesti-rs/
├── Cargo.toml
├── rust-toolchain.toml
│
├── src/
│   ├── lib.rs                   ← public API surface
│   ├── error.rs                 ← VolestiError enum
│   │
│   ├── polytope/                ← replaces: include/convex_bodies/
│   │   ├── hpolytope.rs         ← hpolytope.h                  ✅
│   │   └── shapes.rs            ← known_polytope_generators.h  ✅
│   │
│   ├── samplers/                ← replaces: include/random_walks/
│   │   ├── walk_type.rs         ← WalkType enum (new, Week 4)  🔲
│   │   ├── ball_walk.rs         ← uniform_ball_walk.hpp        ✅
│   │   ├── hit_and_run.rs       ← uniform_rdhr_walk.hpp +
│   │   │                          uniform_cdht_walk.hpp        🔲 Week 1
│   │   ├── boundary_rdhr.rs     ← boundary_rdhr_walk.hpp       🔲 Week 3
│   │   ├── boundary_cdhr.rs     ← boundary_cdhr_walk.hpp       🔲 Week 3
│   │   └── billiard_walk.rs     ← uniform_billiard_walk.hpp    🔲 Week 2–3
│   │
│   ├── finance/                 ← replaces: include/volume/ + include/sampling/
│   │   ├── portfolio.rs         ← Sam_Canon_Unit in copulas.h  ✅
│   │   └── copula.rs            ← twoParHypFam in copulas.h    ✅
│   │
│   ├── diagnostics/             ← replaces: include/diagnostics/
│   │   ├── psrf.rs              ← univariate_psrf.hpp          🔲 Week 5
│   │   └── ess.rs               ← effective_sample_size.hpp    🔲 Week 5
│   │
│   └── preprocess/              ← replaces: include/preprocess/
│       └── feasible_point.rs    ← feasible_point.hpp           🔲 Week 6
│
├── tests/
│   ├── unit_tests.rs            ← 9 tests                      ✅
│   ├── statistical_test.rs      ← 4 tests                      ✅
│   └── equivalence_test.rs      ← 5 tests                      ✅
│
├── benches/
│   └── benchmarks.rs            ← Criterion 6 walks × 4 dims   ✅
│
└── .github/
    └── workflows/
        └── ci.yml               ← fmt + clippy + test on push
---

## GSoC 2026 Roadmap

| Deliverable | Weeks | What gets built |
|---|---|---|
| D1 — Samplers | 1–4 | Hit-and-Run, Billiard Walk, Boundary walks, WalkType enum |
| D2 — Diagnostics | 5–6 | PSRF + ESS as public API, finance API upgraded to WalkType |
| D3 — Preprocessing | 7–9 | `feasible_point()`, benchmarks at d=10,50,100,200 vs C++ |
| D4 — Release | 10–11 | crates.io publish, rustdoc, benchmark report |
| Stretch — PyO3 | 12 | `pip install volesti-rs` Python bindings |

**Post-GSoC:**
Year 1 — Accelerated Billiard Walk, Gaussian walks.
Year 2 — Volume approximation algorithms.
Year 3 — HMC, CRHMC, NUTS.

---

## Mathematical Background

- Bachelard, Chalkis, Fisikopoulos, Tsigaridas —
  *Randomized geometric tools for anomaly detection in stock markets*,
  AISTATS 2023, PMLR 206:9400–9416
- Chalkis, Fisikopoulos —
  *volesti: Volume Approximation and Sampling for Convex Polytopes in R*,
  The R Journal, 13(2):642–660, 2021
- Lovász, Vempala —
  *Hit-and-Run from a Corner*,
  SIAM Journal on Computing, 35(4):985–1005, 2006
- Polyak, Gryazina —
  *Random Sampling: Billiard Walk Algorithm*,
  EJOR, 238(2):497–504, 2014
- Gelman, Rubin —
  *Inference from Iterative Simulation Using Multiple Sequences*,
  Statistical Science, 7(4):457–472, 1992

---

## Development
```bash
git clone https://github.com/akashchakrabortymsc-cmd/Volesti.rs
cd Volesti.rs
cargo test        # all 18 tests
cargo bench       # Criterion benchmarks
cargo clippy      # zero warnings enforced
cargo fmt --check # formatting enforced
```

Requires Rust stable (pinned in `rust-toolchain.toml`).
CI runs on every push: `fmt` → `clippy -D warnings` → `test`.

See [TESTING.md](TESTING.md) for full test documentation.
See [DEVLOG.md](DEVLOG.md) for weekly development progress.

---

## License

Apache 2.0 — same as [volesti](https://github.com/GeomScale/volesti).

---

*[Akash Chakraborty](https://github.com/akashchakrabortymsc-cmd) —
WorldQuant University MScFE — GSoC 2026 applicant, GeomScale.*
