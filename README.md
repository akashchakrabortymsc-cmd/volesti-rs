
KS test H-cube 10D max dim: D = 0.051  ✓  (threshold < 0.06)
```
 
---
 
## Quick Start
 
Add to your `Cargo.toml`:
 
```toml
[dependencies]
volesti-rs = "0.1.0"
```
 
**Sampling from a convex polytope:**
 
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
 
**Portfolio sampling and copula estimation:**
 
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
 
    println!("Crisis indicator κ: {:.4}", crisis);
    println!("Copula grid sum:    {:.4}",
        copula.grid.iter().flat_map(|r| r.iter()).sum::<f64>()); // ≈ 1.0
}
```
 
---
 
## Why Rust
 
GeomScale's current stack is C++ (core), R (statistics), and Python via
[dingo](https://github.com/GeomScale/dingo). There is no Rust implementation
— confirmed by searching crates.io in March 2026.
 
Rust fills a specific gap for quantitative finance workloads:
 
**Memory safety without GC.** No use-after-free, no data races, verified
at compile time. Zero `unsafe` blocks in core sampling paths.
 
**C++ performance.** 15 µs/sample at d=50 without architecture-specific
optimisation. Competitive with the C++ original.
 
**Native ecosystem integration.** `cargo add volesti-rs`. No CMake,
no Boost, no Eigen headers, no compiler flags.
 
**HFT-ready.** Jane Street, Hudson River Trading, and Citadel all use
Rust in production. A native Rust polytope sampler is directly usable
in quant infrastructure without FFI overhead.
 
---
 
## Architecture
 
The crate mirrors volesti's `include/` directory structure exactly.
Every Rust module maps to a specific C++ header.
 
```
volesti-rs/
├── Cargo.toml
├── rust-toolchain.toml          ← pinned stable toolchain
│
├── src/
│   ├── lib.rs                   ← public API surface
│   ├── error.rs                 ← VolestiError enum
│   │
│   ├── geometry/                ← mirrors: include/convex_bodies/
│   │   ├── hpolytope.rs         ← hpolytope.h                  ✅
│   │   └── shapes.rs            ← known_polytope_generators.h  ✅
│   │
│   ├── samplers/                ← mirrors: include/random_walks/
│   │   ├── ball_walk.rs         ← uniform_ball_walk.hpp        ✅
│   │   ├── hit_and_run.rs       ← uniform_cdhr_walk.hpp        🔲
│   │   ├── billiard_walk.rs     ← uniform_billiard_walk.hpp    🔲
│   │   ├── portfolio.rs         ← original finance API         ✅
│   │   └── copula.rs            ← copulas.h                    ✅
│   │
│   ├── diagnostics/             ← mirrors: include/diagnostics/
│   │   ├── psrf.rs              ← univariate_psrf.hpp          🔲
│   │   └── ess.rs               ← effective_sample_size.hpp    🔲
│   │
│   └── preprocess/              ← mirrors: include/preprocess/
│       └── feasible_point.rs    ← feasible_point.hpp           🔲
│
├── tests/
│   ├── unit_tests.rs            ← 9 tests   ✅
│   ├── statistical_test.rs      ← 4 tests   ✅
│   └── equivalence_test.rs      ← 5 tests   ✅
│
├── benches/
│   └── benchmarks.rs            ← Criterion benchmarks ✅
│
└── .github/
    └── workflows/
        └── ci.yml               ← fmt + clippy + test on push
```
 
---
 
## Roadmap
 
This is an independent, ongoing port of the full volesti library into Rust.
The work proceeds in layers, each release adding a new set of algorithms
from the C++ original.
 
| Version | Scope |
|---|---|
| v0.1.0 | Ball Walk + HPolytope + portfolio sampling + copula (current) |
| v0.2.0 | Hit-and-Run (RDHR + CDHR) + WalkType enum |
| v0.3.0 | Billiard Walk + Boundary walks + PSRF/ESS diagnostics |
| v0.4.0 | `feasible_point()` + full benchmark report vs C++ volesti |
| v0.5.0 | Gaussian walks (Gaussian Hit-and-Run, Gaussian Ball Walk) |
| v1.0.0 | Volume approximation algorithms |
| Future  | HMC, CRHMC, NUTS + PyO3 Python bindings |
 
---
 
## Mathematical Background
 
The algorithms in this crate are based on the following papers,
all authored or co-authored by the original volesti team:
 
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
See [DEVLOG.md](DEVLOG.md) for development progress.
 
---
 
## Contributing
 
Contributions are welcome. If you are porting a specific algorithm from
the C++ volesti source, please note the corresponding header file in your
PR description so the mapping can be verified. All PRs must pass
`cargo clippy -D warnings` and `cargo fmt --check` before review.
 
---
 
## License
 
MIT — see [LICENSE](LICENSE).
 
The original volesti C++ library is licensed under
[Apache 2.0](https://github.com/GeomScale/volesti/blob/develop/LICENSE).
This Rust port is an independent work and is released under MIT.
Use of this crate in research should include citation of the original
volesti papers listed above.
 
---
 
*Developed by [Akash Chakraborty](https://github.com/akashchakrabortymsc-cmd)
(WorldQuant University MScFE) as an independent contribution to the
Rust ecosystem and the GeomScale research community