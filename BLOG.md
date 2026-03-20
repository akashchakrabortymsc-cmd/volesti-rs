# Blog — volesti-rs

Technical writing on geometric MCMC sampling, Rust systems
programming, and quantitative finance.

Published as part of the GSoC 2026 project under GeomScale.

---

## Published

---

### [001] Why Rust Needs a Polytope Sampler
*March 21, 2026 — Pre-GSoC*

> **Tags:** `rust` `mcmc` `convex-polytopes` `gsoc2026`

**Summary:**
_[Your intro here]_

**What I cover:**
- What a convex polytope is and why sampling it matters
- The gap in the Rust ecosystem — nothing on crates.io
- Why HFT and quant firms care about this
- What volesti-rs is building

**Key takeaway:**
_[One sentence — what should the reader remember]_

---

### [002] Reading C++ to Write Rust — Porting hpolytope.h
*March 21, 2026 — Pre-GSoC*

> **Tags:** `rust` `cpp` `nalgebra` `hpolytope`

**Summary:**
_[Your intro here]_

**What I cover:**
- How to read volesti's C++ template style without knowing C++
- `is_in()` → `contains()` — the core membership oracle
- Why `normalize()` is mandatory before Billiard Walk
- The `f64` vs `f32` trap that breaks high-dimensional sampling

**Key takeaway:**
_[One sentence]_

---

### [003] Ball Walk from Scratch — MCMC in 150 Lines of Rust
*March 21, 2026 — Pre-GSoC*

> **Tags:** `rust` `ball-walk` `mcmc` `sampling`

**Summary:**
_[Your intro here]_

**What I cover:**
- The three-line mathematical description of Ball Walk
- Uniform ball sampling via Gaussian direction + volume scaling
- Why burn-in exists and how much you need
- The early-exit optimization that gave 53% speedup at 500D

**Key takeaway:**
_[One sentence]_

---

### [004] How to Test an MCMC Sampler — PSRF vs KS
*March 21, 2026 — Pre-GSoC*

> **Tags:** `statistics` `psrf` `gelman-rubin` `mcmc-testing`

**Summary:**
_[Your intro here]_

**What I cover:**
- Why unit tests are not enough for MCMC
- What PSRF (Gelman-Rubin R̂) measures and why R̂ < 1.1
- Why KS tests fail for MCMC (autocorrelation problem)
- How volesti's own test suite works — porting `sampling_test.cpp`

**Key takeaway:**
_[One sentence]_

---

### [005] Portfolio Sampling on the Simplex
*March 21, 2026 — Pre-GSoC*

> **Tags:** `finance` `portfolio` `simplex` `mcmc`

**Summary:**
_[Your intro here]_

**What I cover:**
- Why portfolio constraints form a convex polytope
- The standard simplex as a portfolio feasibility region
- How MCMC sampling replaces brute-force enumeration
- Cross-sectional scoring for portfolio ranking

**Key takeaway:**
_[One sentence]_

---

## Upcoming

---

### [006] Hit-and-Run — Why O(n²) Beats O(n³)
*Planned: GSoC Week 1*

> **Tags:** `hit-and-run` `cdhr` `mixing-time` `rust`

**What I will cover:**
- The chord computation — `line_intersect_coord()` explained
- Coordinate Direction Hit-and-Run vs Random Direction
- Mixing time comparison: Ball Walk O(n³) vs Hit-and-Run O(n²)
- Benchmark results at d=10, 50, 100, 200

---

### [007] Billiard Walk — The Hardest Algorithm to Port
*Planned: GSoC Week 3*

> **Tags:** `billiard-walk` `reflection` `numerical-stability` `rust`

**What I will cover:**
- The reflection formula `v -= 2*(v·n)*n` and why normalization matters
- The EPSILON guard — what happens without it at d=100
- Workspace reuse in Rust — avoiding allocations per step
- Why Billiard Walk mixes in O(n^1.5) in practice

---

### [008] Copulas for Tail Risk — 2008 in One Formula
*Planned: GSoC Week 5*

> **Tags:** `copula` `tail-risk` `cvar` `finance`

**What I will cover:**
- Why Gaussian copulas failed in 2008
- Tail dependence and the crisis indicator
- How polytope sampling gives you copulas that Gaussian models miss
- `compute_copula()` API walkthrough

---