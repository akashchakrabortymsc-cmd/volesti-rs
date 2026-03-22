# GSoC 2026 — volesti-rs

**Organization:** GeomScale
**Project:** Native Rust MCMC Samplers for Convex Polytopes with Quantitative Finance API
**Contributor:** Akash Chakraborty
**Mentor:** Apostolos Chalkis (Research Engineer, Quantagonia)
**Project Size:** Large — 350 hours
**Duration:** May 27 — August 25, 2026

---

## Proposal

> Link will be added after submission on March 28, 2026.

---

## The Problem

volesti is the state-of-the-art C++ library for geometric MCMC sampling
of convex polytopes. It has R and Python interfaces.

**There is zero Rust presence.** The entire Rust ecosystem — used heavily
in HFT and fintech — has no access to these algorithms.

This project builds the first pure Rust implementation of three MCMC
samplers for high-dimensional convex polytopes, with a quantitative
finance API layer on top.

---

## Deliverables

### Pre-GSoC PoC — Complete ✅

| Component | Status | Tests |
|---|---|---|
| `HPolytope` struct + `contains()` | ✅ Done | 3 unit tests |
| `unit_hypercube()`, `simplex()` | ✅ Done | 2 unit tests |
| Ball Walk sampler | ✅ Done | PSRF R̂ = 1.03 < 1.1 |
| Portfolio sampling API | ✅ Done | 2 unit tests |
| Copula estimation API | ✅ Done | 2 unit tests |
| Statistical + equivalence tests | ✅ Done | 9 tests |
| Criterion benchmarks | ✅ Done | 15 µs/sample at d=50 |
| GitHub Actions CI | ✅ Done | fmt + clippy + test |

**Total: 18 tests passing. Zero warnings.**

---

### Mid-term Target — July 7–14

| Deliverable | C++ Reference | Week |
|---|---|---|
| Hit-and-Run (CDHR) | `uniform_cdhr_walk.hpp` | 1 |
| Hit-and-Run (RDHR) | `uniform_rdhr_walk.hpp` | 1 |
| Billiard Walk | `uniform_billiard_walk.hpp` | 2–3 |
| Boundary RDHR | `boundary_rdhr_walk.hpp` | 3 |
| Boundary CDHR | `boundary_cdhr_walk.hpp` | 3 |
| `WalkType` enum — walk-agnostic API | Original design | 4 |

**Mid-term passing grade:** Ball Walk + Hit-and-Run complete with
PSRF R̂ < 1.1. Billiard Walk is bonus.

---

### Final Target — August 25

| Deliverable | C++ Reference | Week |
|---|---|---|
| PSRF diagnostic as public API | `univariate_psrf.hpp` | 5 |
| ESS diagnostic as public API | `effective_sample_size.hpp` | 5 |
| `feasible_point()` — Chebyshev center | `feasible_point.hpp` | 6 |
| Finance API upgraded to `WalkType` | Original | 7 |
| Full benchmark report vs C++ volesti | `benchmarks_cb.cpp` | 10 |
| crates.io publication | — | 11 |
| rustdoc API documentation | — | 11 |
| PyO3 Python bindings | Stretch goal | 12 |

---

## Weekly Progress

Tracked in [DEVLOG.md](DEVLOG.md).

---

## Test Strategy

Following volesti's own test methodology from `test/sampling_test.cpp`:

| Layer | Method | Threshold |
|---|---|---|
| Structural | Unit tests — `contains()`, membership | All pass |
| Statistical | PSRF Gelman-Rubin R̂ | R̂ < 1.1 |
| Equivalence | KS test vs uniform reference | D < D_crit (α=0.01) |
| Performance | Criterion benchmarks | Within 2x of C++ |

Full test documentation: [TESTING.md](TESTING.md)

---

## AI Disclosure

Claude (Anthropic) was used to assist with:
- Proposal structure and documentation drafting
- Rust syntax guidance and code review
- C++ volesti codebase analysis and translation

All mathematical content, algorithm implementations, and technical
decisions were independently verified by the contributor against
the C++ volesti source code and the following academic papers:
- Lovász & Vempala (2006) — mixing time proofs
- Polyak & Gryazina (2014) — Billiard Walk algorithm
- Gelman & Rubin (1992) — PSRF convergence diagnostic
- Bachelard, Chalkis et al. (2023) — AISTATS finance application

---

## Contact

- **Contributor:** Akash Chakraborty
  — [GitHub](https://github.com/akashchakrabortymsc-cmd)
  — WorldQuant University MScFE
- **Mentor:** Apostolos Chalkis
  — [GitHub](https://github.com/TolisChal)
  — Research Engineer, Quantagonia
- **Organization:** [GeomScale](https://geomscale.github.io/)
