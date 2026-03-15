# Volesti-rs 🦀

A high-performance **Rust port** of [volesti](https://github.com/GeomScale/volesti) — a C++ library for volume approximation and MCMC sampling of convex polytopes — with a focus on **quantitative finance applications**.

> **GSoC 2026 Proposal Project** | Organization: [GeomScale](https://geomscale.github.io/)

---

## Why Rust?

GeomScale's current stack covers C++ (core), R (statistics), and Python (data science). **Rust fills a critical gap:**


## Author

**Akash Chakraborty**
MScFE | GSoC 2026 Applicant — GeomScale

---
## Performance

Benchmarks on Windows, release build (`cargo bench`):

| Polytope | Samples | Time |
|---|---|---|
| H-cube 10D | 100 | ~0.97 ms |
| H-cube 50D | 1000 | ~15 ms |
| H-cube 100D | 1000 | ~76 ms |
| H-cube 500D | 100 | ~200 ms |
| Portfolio 50 assets | 100 portfolios | ~14 ms |

Target: within 2x of C++ volesti (Hit-and-Run and Billiard Walk coming in GSoC 2026).



## License

Apache 2.0 — same as volesti
