# MATHILDE `math` (Rust crate)

MATHILDE PROPRIETARY AND CONFIDENTIAL  
Legal contact: massimo.nicora@wnlegal.ch

## Purpose

This repository contains the `math` Rust crate: deterministic, time-bounded numerical/statistical primitives used by MATHILDE.

The crate is intentionally algorithmic-only: it contains no I/O, no networking, and no runtime services.

## Where to start

- Global inventory (generated): `inventory.md`
- Module inventories: `src/*/docs/inventory.md`
- Protocols (doc processes): `docs/protocols/`

Note on paths: `inventory.md` and some docs use canonical monorepo-style paths like `v2/crates/math/...`. In this repository checkout, those paths correspond to this directory (for example, `v2/crates/math/src/bayes/mod.rs` maps to `src/bayes/mod.rs`).

## Repository layout

- `src/lib.rs`: crate entrypoint; exports modules and `MathError` / `MathResult`
- `src/<module>/`: implementation by domain (see module index below)
- `src/<module>/docs/`: module scope/spec/inventory and reviews
- `src/<module>/tests/`: module tests (when present)
- `benches/`: Criterion benches (bench names are listed in `inventory.md`)
- `src/bin/`: deterministic generators (inventory + precomputed assets)
- `docs/protocols/`: documentation protocols (inventory generation, module creation/audit)

## Module index (high level)

Each module has a dedicated inventory and spec under `src/<module>/docs/`.

- `bayes`: conjugate updates and MAP/Laplace fits (logistic/poisson/student-t/AR(1))
- `copula`: bivariate copula fitting (MLE / Kendall’s tau) and dependence measures
- `core`: shared primitives (validation, statistics, correlation, condensed indexing, union-find)
- `econometrics`: deterministic tests/estimators (unit-root, breakpoints, regression, HAC, diagnostics)
- `evt`: peaks-over-threshold (GPD fit, declustering, tail index, VaR/ES utilities)
- `finance`: candle/return features, volatility estimators, drawdown, volume-clock utilities
- `graph`: MST/MaxST and kNN graph construction and graph/tree scalar metrics
- `hawkes`: Hawkes process estimation (univariate/marked/multivariate; multiple kernel families)
- `hurst`: Hurst exponent estimators, confidence intervals, bias correction, and consensus aggregation
- `linalg`: small linear algebra (Cholesky, QR, least-squares, SPD solves) with validation/workspaces
- `markov`: Markov chains, HMM (forward/backward/Viterbi + EM training), and HSMM filtering
- `ml`: deterministic ML primitives (k-means, clustering metrics, PCA, preprocessing, change detection)
- `multifractal`: MF-DFA/MF-DMA/MF-DCCA post-processing, spectrum/Legendre utilities, WTMM/WLM endpoints
- `signal`: time-series signal processing (detrending, entropy, filtering, spectral estimators, wavelets, SSA)
- `state_space`: 1D/2D state-space filters (local level / local linear trend; robust variants; masked/dt support)
- `stochastic`: deterministic sampling and resampling utilities (bootstrap, QMC/RQMC, innovations, processes)
- `tda`: Vietoris–Rips persistence (H0/H1) and multiscale summaries with workspace reuse

## Common commands

From repo root:

- Build: `cargo build`
- Run tests: `cargo test`
- Run benches: `cargo bench`

## Generators

- Global inventory generation (overwrites `inventory.md`):
  - `cargo run -p math --bin generate_global_inventory`
  - strict mode: `cargo run -p math --bin generate_global_inventory -- --strict`
- Asset generators (write precomputed tables/constants used by runtime code):
  - `src/bin/gen_dpss_asset.rs`
  - `src/bin/gen_hurst_bias_tables.rs`
  - `src/bin/gen_savgol_coeffs.rs`
