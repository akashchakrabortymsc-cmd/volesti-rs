# Development Log — volesti-rs

Weekly engineering journal for the volesti-rs GSoC 2026 project.
Each entry documents what was completed, what's in progress,
and what's blocked.

---

## Pre-GSoC Period — March 2026

### Week of March 21, 2026

#### Completed ✅
- `HPolytope` struct — `contains()`, `normalize()`,
  `line_intersect_coord()`, `line_positive_intersect()`
- `unit_hypercube(n)` and `simplex(n)` constructors
- Ball Walk sampler — auto-tuned delta, burn-in, thinning
- `sample_portfolios()` — valid weight vectors from simplex
- `compute_copula()` — tail dependency estimation
- `crisis_indicator()` — market stress detection
- 18 tests passing across 3 test files
- PSRF R̂ = 1.03 on H-cube10 — matches C++ volesti threshold
- Criterion benchmarks — 15 µs/sample at d=50
- `cargo clippy` zero warnings
- GitHub Actions CI — runs on every push
- README, TESTING.md, GSOC.md, CHANGELOG.md

#### Key Technical Decisions
- Pure Rust rewrite (not FFI) — maintainable, crates.io publishable
- `nalgebra` for linear algebra — fully Rust-native, no C dependency
- `f64` everywhere — f32 causes polytope escape in high dimensions
- `StdRng` (seeded) — reproducible tests and deterministic benchmarks
- PSRF over KS for convergence — MCMC samples are autocorrelated,
  not i.i.d. KS assumes i.i.d. PSRF handles autocorrelation correctly
- Early exit in `contains()` — 53% speedup at 500D (422ms → 200ms)

#### Performance Improvement This Week
| Optimization | Before | After | Gain |
|---|---|---|---|
| Early exit in `contains()` | 422 ms | 200 ms | 53% at 500D |
| Early exit in `contains()` | 39.8 ms | 15 ms | 62% at 50D/1000 samples |

#### Blocked
- Nothing currently

---

### Week of March 14, 2026

#### Completed ✅
- Repository created
- `Cargo.toml` with pinned dependencies
- Folder structure set up
- First contact with mentor Apostolos Chalkis on LinkedIn
- Mentor confirmed interest: *"Yes, we would definitely be
  interested in such a proposal"*
- Read `hpolytope.h`, `uniform_ball_walk.hpp` C++ source

---

## GSoC Period — May 27 to August 25, 2026

*Entries will be added weekly during GSoC.*

---

### Week 1 — May 27 to June 2

**Target:** Hit-and-Run (CDHR + RDHR) complete

- [ ] Read `uniform_cdhr_walk.hpp` and `uniform_rdhr_walk.hpp`
- [ ] Implement `line_intersect_coord()` — chord computation
- [ ] Implement `hit_and_run()` function
- [ ] PSRF test passing — R̂ < 1.1 on H-cube10
- [ ] Compare mixing time vs Ball Walk at d=10, 50, 100

**Expected result:** Hit-and-Run mixes in O(n²) vs Ball Walk O(n³).
Should see significant speedup at d=100.

---

### Week 2–3 — June 3 to June 16

**Target:** Billiard Walk complete

- [ ] Read `uniform_billiard_walk.hpp`
- [ ] Implement `line_positive_intersect()` with workspace reuse
- [ ] Implement reflection formula: `v -= 2*(v·n)*n`
- [ ] Add EPSILON guard for degenerate reflections (blind spot #4)
- [ ] PSRF test passing — R̂ < 1.1 on H-cube10

**Known risk:** Numerical instability near polytope edges at high
dimensions. EPSILON = 1e-10 guard required — documented in
project master sheet.

---

### Week 3 — June 17 to June 23

**Target:** Boundary walks + WalkType enum

- [ ] Boundary RDHR
- [ ] Boundary CDHR
- [ ] `WalkType` enum — walk-agnostic API design

---

### Week 4 — June 24 to June 30

**Target:** Walk-agnostic finance API

- [ ] Upgrade `sample_portfolios()` to accept `WalkType`
- [ ] Integration tests across all walk types

---

### Week 5–6 — July 1 to July 13

**Target:** Diagnostics as public API + mid-term evaluation

- [ ] `psrf()` — public API wrapping internal PSRF logic
- [ ] `ess()` — Effective Sample Size
- [ ] Mid-term submission: Ball Walk + Hit-and-Run passing

**Mid-term passing grade:** Two samplers complete with PSRF < 1.1.

---

### Week 7–9 — July 14 to August 3

**Target:** Billiard Walk finance integration + preprocessing

- [ ] `feasible_point()` — Chebyshev center via LP
- [ ] Billiard Walk integrated into finance API
- [ ] Full benchmark suite at d=10, 50, 100, 200

---

### Week 10–11 — August 4 to August 17

**Target:** Release preparation

- [ ] crates.io publication
- [ ] rustdoc documentation for all public API
- [ ] Benchmark report blog post vs C++ volesti
- [ ] README updated with final numbers

---

### Week 12 — August 18 to August 25

**Target:** PyO3 stretch goal + final submission

- [ ] PyO3 bindings (if D1–D4 complete)
- [ ] Final evaluation submission
- [ ] `pip install volesti-rs` demo

---

## Notes

### C++ Files Read
| File | Date | Key Insight |
|---|---|---|
| `hpolytope.h` | Mar 14 | `is_in()` → our `contains()`. Normalize before Billiard Walk. |
| `uniform_ball_walk.hpp` | Mar 21 | Ball sampling via Gaussian + volume scaling |
| `test/sampling_test.cpp` | Mar 21 | They use PSRF, not KS. Threshold R̂ < 1.1. walkL=10. |