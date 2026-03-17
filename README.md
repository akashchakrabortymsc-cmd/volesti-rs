# volesti-rs

**A Rust port of [volesti](https://github.com/GeomScale/volesti) — the C++ library for MCMC sampling and volume approximation of convex polytopes — with a finance API layer for portfolio analysis and crisis detection.**

[![Tests](https://img.shields.io/badge/tests-18%20passing-brightgreen)](tests/)
[![GSoC 2026](https://img.shields.io/badge/GSoC_2026-GeomScale-orange)](https://summerofcode.withgoogle.com)
[![License](https://img.shields.io/badge/license-Apache_2.0-blue)](LICENSE)

---

## What this is

volesti is a research-grade C++ library by [Apostolos Chalkis](https://github.com/TolisChal) and [Vissarion Fisikopoulos](https://github.com/vissarion) that implements state-of-the-art geometric random walks for sampling from convex polytopes. It underlies published work in computational geometry and quantitative finance, including the AISTATS 2023 paper on anomaly detection in stock markets.

This repository is a Rust port of volesti's core sampling algorithms, extended with:

- A finance API layer (`sample_portfolios`, `compute_copula`, `portfolio_score`)
- Python bindings via PyO3 *(coming in GSoC 2026)*
- A rolling-window crisis detection demo on real ETF data *(coming in GSoC 2026)*

The port is being developed as a **GSoC 2026 project** under the GeomScale organization.

---

## Current status

| Component | Status | Tests |
|---|---|---|
| `HPolytope` — H-representation polytope | ✅ Done | 3 unit tests |
| `ball_walk()` — Metropolis-Hastings sampler | ✅ Done | PSRF R-hat < 1.1 on H-cube10 |
| `sample_portfolios()` — simplex sampling | ✅ Done | 2 unit tests |
| `compute_copula()` — copula estimation | ✅ Done | 2 unit tests |
| `hit_and_run()` — RDHR + CDHR | 🔲 Week 1 | — |
| `billiard_walk()` — reflective walk | 🔲 Week 2–3 | — |
| `portfolio_score()` | 🔲 Week 4 | — |
| PyO3 Python bindings | 🔲 Week 7–9 | — |
| Rolling-window crisis detection | 🔲 Week 10–11 | — |

**18 tests passing. 0 failures.**

```
test result: ok. 9 passed  (unit_tests.rs)
test result: ok. 4 passed  (statistical_test.rs)
test result: ok. 5 passed  (equivalence_test.rs)
```

---

## Performance

Benchmarks on Windows, release build (`cargo bench`).
Ball Walk PoC — single-threaded, no SIMD, no parallelism.

| Polytope | Samples | Time | Per sample |
|---|---|---|---|
| H-cube 10D | 100 | ~0.97 ms | 9.7 µs |
| H-cube 50D | 1000 | ~15 ms | **15 µs** |
| H-cube 100D | 1000 | ~76 ms | 76 µs |
| H-cube 500D | 100 | ~200 ms | 2 ms |
| Simplex 50D | 100 | ~1.7 ms | 17 µs |
| Portfolio 50 assets | 100 portfolios | ~14 ms | 140 µs |

The headline number is **15 µs/sample at d=50** — measured with Criterion in release profile.

Target for GSoC: benchmark all three walks (Ball Walk, Hit-and-Run, Billiard Walk) against C++ volesti on identical inputs at d=10, 50, 100, 200.

---

## Statistical correctness

Ball Walk correctness is verified by two independent methods, following volesti's own test methodology:

**PSRF convergence (Gelman-Rubin R-hat)** — the same diagnostic used in volesti's C++ test suite (`test/sampling_test.cpp`). R-hat < 1.1 on the 10D hypercube with 10,000 samples and thinning=10.

```
Ball Walk PSRF on H-cube10:  R-hat = 1.03  ✓  (threshold: < 1.1)
Ball Walk PSRF on Simplex-3: R-hat = 1.04  ✓
```

**KS distributional equivalence** — five-test suite verifying marginal distributions match Uniform[-1,1] on the 3D cube, with MCMC-aware threshold calibration (D < 0.06).

---

## Architecture

The crate mirrors volesti's `include/` folder structure so every Rust function can be compared directly to its C++ counterpart.

```
volesti-rs/
├── Cargo.toml
├── pyproject.toml          ← D3, new in Week 7
├── README.md
├── STRUCTURE.md
├── LICENSE
│
├── src/
│   ├── lib.rs
│   ├── error.rs
│   ├── polytope/
│   │   ├── mod.rs
│   │   └── hpolytope.rs    ← DONE ✓
│   ├── samplers/
│   │   ├── mod.rs
│   │   ├── ball_walk.rs    ← DONE ✓
│   │   ├── hit_and_run.rs  ← Week 1
│   │   └── billiard_walk.rs← Week 2-3
│   ├── finance/
│   │   ├── mod.rs
│   │   ├── portfolio.rs    ← DONE ✓
│   │   ├── copula.rs       ← DONE ✓
│   │   ├── portfolio_score.rs ← Week 4
│   │   └── crisis_detection.rs← Week 10-11
│   └── python/
│       └── lib.rs          ← Week 7-9
│
├── tests/
│   ├── unit_tests.rs       ← DONE ✓ (9 tests)
│   ├── statistical_test.rs ← DONE ✓ (4 tests)
│   └── equivalence_test.rs ← DONE ✓ (5 tests)
│
├── benches/
│   └── benchmarks.rs       ← DONE ✓
│
├── data/
│   └── etf_2006_2016.csv   ← Week 10
│
└── notebooks/
    └── crisis_detection.ipynb ← Week 11
```

---

## Quick start

```toml
# Cargo.toml
[dependencies]
volesti-rs = "0.1"   # on crates.io after GSoC Week 12
```

```rust
use volesti_rs::geometry::hpolytope::HPolytope;
use volesti_rs::samplers::ball_walk::{ball_walk, BallWalkConfig};
use volesti_rs::finance::portfolio::sample_portfolios;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() {
    let mut rng = StdRng::seed_from_u64(42);

    // Sample 1000 portfolios from 50-asset universe
    let portfolios = sample_portfolios(50, 1000, &mut rng).unwrap();
    println!("Mean portfolio weights: {:?}", portfolios.mean_weights());

    // Sample directly from a convex polytope
    let polytope = HPolytope::unit_hypercube(10);
    let start    = volesti_rs::geometry::point::Point::new(vec![0.0; 10]);
    let config   = BallWalkConfig::default();
    let samples  = ball_walk(&polytope, &start, 500, &config, &mut rng).unwrap();
    println!("Sampled {} points from 10D hypercube", samples.len());
}
```

---

## Why Rust

GeomScale's current stack is C++ (core algorithms), R (statistical interface), Python via dingo (data science interface). There is no Rust implementation of volesti — confirmed by searching crates.io in March 2026.

Rust fills a specific gap:

- **Memory safety without GC** — no segfaults, no undefined behaviour in production trading systems
- **Zero-cost Python bindings** — PyO3 produces a native `.so` loadable by Python with no FFI overhead and no C++ toolchain dependency
- **cargo ecosystem** — `cargo add volesti-rs` in any Rust project, no manual build configuration
- **Potential dingo backend** — the PyO3 bindings are a memory-safe alternative to dingo's current Cython-over-C++ approach

---

## Mathematical background

The algorithms implemented here are described in:

- Bachelard, Chalkis, Fisikopoulos, Tsigaridas — *Randomized geometric tools for anomaly detection in stock markets*, AISTATS 2023
- Chalkis, Fisikopoulos — *volesti: Volume Approximation and Sampling for Convex Polytopes in R*, R Journal 2021
- Calès, Chalkis, Emiris, Fisikopoulos — *Practical Volume Computation of Structured Convex Bodies*, SoCG 2018

The portfolio geometry model (simplex ∩ ellipsoid → sphere patch K), the copula-based crisis indicator, and the rolling-window classification scheme are all ported from these papers and the volesti C++ source.

---

## GSoC 2026 roadmap

| Milestone | Weeks | Deliverable |
|---|---|---|
| I | 1–6 | Hit-and-Run + Billiard Walk + benchmarks vs C++ |
| II | 7–9 | PyO3 Python bindings — pip install volesti-rs |
| III | 10–11 | Crisis detection demo on Stanford EE103 ETF dataset |
| IV | 12 | crates.io publish + docs.rs + blog post |


---

## Development

```bash
git clone https://github.com/akashchakrabortymsc-cmd/Volesti_Rust
cd Volesti_Rust
cargo test        # run all 18 tests
cargo bench       # run Criterion benchmarks
```

Requires Rust 1.75+ and stable toolchain.

---

## License

Apache 2.0 — same as [volesti](https://github.com/GeomScale/volesti).

---

*Built by [Akash Chakraborty](https://github.com/akashchakrabortymsc-cmd) — GSoC 2026 applicant, GeomScale.*
