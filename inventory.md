--------------------------------------------------------------------------------
MATHILDE PROPRIETARY AND CONFIDENTIAL
Copyright (c) 2024 MATHILDE. All Rights Reserved.

This document contains trade secrets and confidential information owned
exclusively by MATHILDE, protected under Swiss law (URG, UWG, Art. 162 StGB).

PROHIBITED: Reproduction, copying, distribution, disclosure, or derivative
works without prior written authorization from MATHILDE.

ACCESS REQUIREMENT: Executed NDA with MATHILDE required. Unauthorized access
or possession violates Swiss law. Violations subject to civil remedies,
injunctive relief, damages, and criminal prosecution.

Legal Contact: massimo.nicora@wnlegal.ch
--------------------------------------------------------------------------------

# `math` — Global Inventory (GENERATED; DO NOT EDIT)

Generated: 2026-09-30T09:56:44Z
Protocol: `math/docs/protocols/global_inventory_generation_protocol.md`

This file is generated from per-module inventories under `math/src/*/docs/inventory.md`.
If a file purpose is missing in a module inventory, this file will mark it as `INVENTORY GAP`.

## Modules

- `math::bayes`: `math/src/bayes/docs/inventory.md`
- `math::copula`: `math/src/copula/docs/inventory.md`
- `math::core`: `math/src/core/docs/inventory.md`
- `math::econometrics`: `math/src/econometrics/docs/inventory.md`
- `math::evt`: `math/src/evt/docs/inventory.md`
- `math::finance`: `math/src/finance/docs/inventory.md`
- `math::graph`: `math/src/graph/docs/inventory.md`
- `math::hawkes`: `math/src/hawkes/docs/inventory.md`
- `math::hurst`: `math/src/hurst/docs/inventory.md`
- `math::linalg`: `math/src/linalg/docs/inventory.md`
- `math::markov`: `math/src/markov/docs/inventory.md`
- `math::ml`: `math/src/ml/docs/inventory.md`
- `math::multifractal`: `math/src/multifractal/docs/inventory.md`
- `math::signal`: `math/src/signal/docs/inventory.md`
- `math::state_space`: `math/src/state_space/docs/inventory.md`
- `math::stochastic`: `math/src/stochastic/docs/inventory.md`
- `math::tda`: `math/src/tda/docs/inventory.md`

---

## `math::bayes`

### Artifacts

- Inventory: `math/src/bayes/docs/inventory.md`
- Scope: `math/src/bayes/docs/scope.md`
- Spec: `math/src/bayes/docs/BAYES_SPEC.md`
- Math review: `math/src/bayes/docs/reviews/bayes_module_math_review.md`
- Bench log: `math/src/bayes/docs/bayes_bench_results.md`
- Tests: `math/src/bayes/tests`
- Benches: `math/benches/bayes.rs`

### Source Files

- `math/src/bayes/conjugate/beta_binomial.rs`: Beta-Binomial conjugate posterior updates and credible intervals.
- `math/src/bayes/conjugate/gamma_poisson.rs`: Gamma-Poisson conjugate posterior updates and credible intervals.
- `math/src/bayes/conjugate/mod.rs`: Conjugate-model submodule declarations and re-exports.
- `math/src/bayes/conjugate/normal_inverse_gamma.rs`: Normal-Inverse-Gamma conjugate posterior updates (with Welford stats helpers) and Student-t credible intervals for the mean.
- `math/src/bayes/conjugate/normal_normal.rs`: Normal-Normal conjugate posterior for the mean with known observation variance.
- `math/src/bayes/diagnostics.rs`: Diagnostics helpers for MAP/Laplace fits (standard errors, mean logloss/NLL summaries).
- `math/src/bayes/map_laplace/ar1.rs`: Gaussian AR(1) MAP/Laplace estimation using a Gauss-Newton SPD Hessian approximation.
- `math/src/bayes/map_laplace/cholesky.rs`: Cholesky decomposition/solve/inverse helpers for SPD Hessians.
- `math/src/bayes/map_laplace/logistic.rs`: Logistic regression MAP fit with Laplace (Gaussian) approximation.
- `math/src/bayes/map_laplace/mod.rs`: MAP/Laplace submodule declarations and re-exports.
- `math/src/bayes/map_laplace/poisson.rs`: Poisson regression MAP fit with Laplace (Gaussian) approximation.
- `math/src/bayes/map_laplace/student_t.rs`: Student-t location/scale MAP fit with Laplace (Gaussian) approximation.
- `math/src/bayes/map_laplace/types.rs`: Shared configuration and result types for MAP/Laplace routines.
- `math/src/bayes/mod.rs`: Module root for deterministic Bayesian primitives; declares submodules and re-exports.
- `math/src/bayes/types.rs`: Shared types for Bayesian summaries (credible intervals, posterior summaries, Welford stats, workspace).

---

## `math::copula`

### Artifacts

- Inventory: `math/src/copula/docs/inventory.md`
- Scope: `math/src/copula/docs/scope.md`
- Spec: `math/src/copula/docs/COPULA_SPEC.md`
- Math review: `math/src/copula/docs/reviews/copula_module_math_review.md`
- Bench log: `math/src/copula/docs/copula_bench_results.md`
- Tests: `math/src/copula/tests`
- Benches: `math/benches/copula.rs`

### Source Files

- `math/src/copula/diagnostics.rs`: AIC/BIC/log-likelihood wrappers for fitted bivariate copulas.
- `math/src/copula/fit/frank_inversion.rs`: Frank-family parameter inversion from Kendall’s tau.
- `math/src/copula/fit/loglik.rs`: Bivariate copula log-likelihood evaluation with optional score/caching preparation.
- `math/src/copula/fit/mle.rs`: Deterministic MLE fitting for bivariate copulas (including optional Student-t nu handling).
- `math/src/copula/fit/mod.rs`: Fitting submodule declarations and selective re-exports.
- `math/src/copula/fit/optimizer.rs`: Deterministic 1D optimization helpers (golden-section search and bracket refinement).
- `math/src/copula/fit/tau_fits.rs`: Kendall’s tau based parameter fitting for supported copula families.
- `math/src/copula/measures/mod.rs`: Dependence-measure submodule declarations and re-exports.
- `math/src/copula/measures/pseudo_observations.rs`: Rank-based pseudo-observation (pseudo-uniform) transform for multivariate data.
- `math/src/copula/measures/tail_dependence.rs`: Empirical upper/lower tail dependence estimators over pseudo-observations.
- `math/src/copula/mod.rs`: Module root for copula fitting and dependence measures; declares submodules and re-exports.
- `math/src/copula/types.rs`: Public enums and parameter/fit carriers for bivariate copula families.
- `math/src/copula/workspace.rs`: Workspace buffers (scratch arrays/caches) for repeated copula computations.

---

## `math::core`

### Artifacts

- Inventory: `math/src/core/docs/inventory.md`
- Scope: `math/src/core/docs/scope.md`
- Spec: `math/src/core/docs/CORE_SPEC.md`
- Math review: `math/src/core/docs/reviews/core_module_math_review.md`
- Bench log: `math/src/core/docs/core_bench_results.md`
- Tests: `math/src/core/tests`
- Benches: `math/benches/core_autocorrelation.rs`
- Benches: `math/benches/core_calculus.rs`
- Benches: `math/benches/core_condensed.rs`
- Benches: `math/benches/core_correlation.rs`
- Benches: `math/benches/core_distributions.rs`
- Benches: `math/benches/core_integration.rs`
- Benches: `math/benches/core_run_length.rs`
- Benches: `math/benches/core_statistics.rs`
- Benches: `math/benches/core_variance.rs`

### Source Files

- `math/src/core/autocorrelation.rs`: Autocovariance/autocorrelation computations with normalization options and workspace support.
- `math/src/core/calculus.rs`: Checked floating-point operations, approximate equality, and safe accumulation/indexing utilities.
- `math/src/core/condensed.rs`: Condensed (upper-triangular) matrix indexing/layout helpers and validators.
- `math/src/core/constants.rs`: Shared numerical constants used across the crate.
- `math/src/core/correlation.rs`: Rank transforms and correlation measures (Pearson, Spearman, Kendall tau-b) with optional workspaces.
- `math/src/core/distributions.rs`: Distribution utilities (erf, standard normal CDF, log-sum-exp, diagonal multivariate Gaussian log-pdf).
- `math/src/core/integration.rs`: Series integration and deterministic log-price path reconstruction from returns.
- `math/src/core/mod.rs`: Module root; declares core submodules.
- `math/src/core/run_length.rs`: Run-length and consecutive-threshold counting utilities for label/threshold regimes.
- `math/src/core/statistics.rs`: Deterministic summary statistics, percentiles/ranks, rolling transforms, and moment calculations.
- `math/src/core/union_find.rs`: Union-find (disjoint set) data structure with checked operations.
- `math/src/core/variance.rs`: Biased/unbiased (weighted/unweighted) variance and standard-deviation estimators.

---

## `math::econometrics`

### Artifacts

- Inventory: `math/src/econometrics/docs/inventory.md`
- Scope: `math/src/econometrics/docs/scope.md`
- Spec: `math/src/econometrics/docs/ECONOMETRICS_SPEC.md`
- Math review: `math/src/econometrics/docs/reviews/econometrics_module_math_review.md`
- Bench log: `math/src/econometrics/docs/econometrics_bench_results.md`
- Tests: `math/src/econometrics/tests`
- Benches: `math/benches/econometrics_adf.rs`
- Benches: `math/benches/econometrics_arch_lm.rs`
- Benches: `math/benches/econometrics_bai_perron.rs`
- Benches: `math/benches/econometrics_bocpd.rs`
- Benches: `math/benches/econometrics_box_pierce.rs`
- Benches: `math/benches/econometrics_chow.rs`
- Benches: `math/benches/econometrics_cusum.rs`
- Benches: `math/benches/econometrics_dcor.rs`
- Benches: `math/benches/econometrics_durbin_watson.rs`
- Benches: `math/benches/econometrics_gph.rs`
- Benches: `math/benches/econometrics_icss.rs`
- Benches: `math/benches/econometrics_kpss.rs`
- Benches: `math/benches/econometrics_ljung_box.rs`
- Benches: `math/benches/econometrics_mosum.rs`
- Benches: `math/benches/econometrics_newey_west.rs`
- Benches: `math/benches/econometrics_normality.rs`
- Benches: `math/benches/econometrics_phillips_perron.rs`
- Benches: `math/benches/econometrics_quandt_andrews.rs`
- Benches: `math/benches/econometrics_quantile_slope.rs`
- Benches: `math/benches/econometrics_regression.rs`
- Benches: `math/benches/econometrics_regression_qr.rs`
- Benches: `math/benches/econometrics_robinson.rs`
- Benches: `math/benches/econometrics_robinson_refined.rs`
- Benches: `math/benches/econometrics_variance_ratio.rs`

### Source Files

- `math/src/econometrics/adf.rs`: Augmented Dickey–Fuller unit-root test statistic with deterministic terms and lag rules.
- `math/src/econometrics/arch.rs`: ARCH LM test statistic for conditional heteroskedasticity in residuals.
- `math/src/econometrics/bai_perron.rs`: Bai–Perron style multiple breakpoints for piecewise-constant mean (dynamic programming solver).
- `math/src/econometrics/bocpd.rs`: Bayesian online changepoint detection (BOCPD) for a Gaussian mean model with pruning options.
- `math/src/econometrics/diagnostics.rs`: Regression residual diagnostics (Durbin–Watson statistic).
- `math/src/econometrics/distance_correlation.rs`: Distance correlation estimator with workspace support for centered distance matrices.
- `math/src/econometrics/fractional.rs`: Fractional integration estimators (GPH and Robinson) with deterministic search/config.
- `math/src/econometrics/hac.rs`: Newey–West (Bartlett) long-run variance estimator for regression residuals.
- `math/src/econometrics/icss.rs`: ICSS variance-change detection (max statistic and breakpoint extraction) with workspace support.
- `math/src/econometrics/kpss.rs`: KPSS stationarity test statistics (level and trend) with workspace support.
- `math/src/econometrics/local_whittle_refined.rs`: Refined local Whittle/Robinson-style estimator variants with workspace support.
- `math/src/econometrics/mod.rs`: Module root; declares econometrics submodules and re-exports public APIs.
- `math/src/econometrics/mosum.rs`: MOSUM breakpoint statistics and grid search over window sizes with workspace support.
- `math/src/econometrics/normality.rs`: Normality test statistics (Jarque–Bera and D’Agostino K²).
- `math/src/econometrics/phillips_perron.rs`: Phillips–Perron unit-root test statistics with HAC correction and lag rules.
- `math/src/econometrics/portmanteau.rs`: Ljung–Box and Box–Pierce portmanteau autocorrelation tests with optional workspaces.
- `math/src/econometrics/quantile_slope.rs`: Deterministic 1D quantile-regression slope/intercept estimation with workspace support.
- `math/src/econometrics/regression.rs`: Deterministic OLS/WLS 1D regression fits, residual computation, and slope standard errors.
- `math/src/econometrics/regression_qr.rs`: QR-based regression for multi-column design matrices with workspace support.
- `math/src/econometrics/stability.rs`: Chow and sup-Chow stability tests for OLS 1D regressions.
- `math/src/econometrics/structural_break.rs`: Structural-break statistics (CUSUM/CUSUMSQ and Quandt–Andrews sup-Wald).
- `math/src/econometrics/variance_ratio.rs`: Overlapping variance-ratio statistic for return series.

---

## `math::evt`

### Artifacts

- Inventory: `math/src/evt/docs/inventory.md`
- Scope: `math/src/evt/docs/scope.md`
- Spec: `math/src/evt/docs/EVT_SPEC.md`
- Math review: `math/src/evt/docs/reviews/evt_module_math_review.md`
- Bench log: `math/src/evt/docs/evt_bench_results.md`
- Tests: `math/src/evt/tests`
- Benches: `math/benches/evt.rs`

### Source Files

- `math/src/evt/crypto.rs`: Crypto-oriented defaults and guardrails for EVT usage.
- `math/src/evt/data.rs`: Exceedance extraction utilities for peaks-over-threshold (POT) workflows.
- `math/src/evt/declustering.rs`: Runs declustering utilities for dependent exceedances.
- `math/src/evt/diagnostics.rs`: GPD CDF and goodness-of-fit diagnostics (e.g., KS/PP summaries).
- `math/src/evt/extremal_index.rs`: Extremal-index proxy estimators (runs/consecutive clustering summaries).
- `math/src/evt/gpd.rs`: Generalized Pareto distribution primitives (NLL/gradient) and deterministic MLE fitting.
- `math/src/evt/mod.rs`: Module root for EVT primitives; declares submodules and re-exports.
- `math/src/evt/optimization.rs`: EVT-specific deterministic bounded 2D BFGS optimizer used by fitting routines.
- `math/src/evt/risk.rs`: VaR/ES calculations from GPD parameters and end-to-end POT helpers.
- `math/src/evt/tail_index/hill.rs`: Hill tail-index estimator utilities (estimator, plot, and k-selection helper).
- `math/src/evt/tail_index/mod.rs`: Tail-index submodule declarations and re-exports.
- `math/src/evt/time_varying.rs`: Time-varying (covariate-linked) POT scale model and associated VaR/ES helpers.
- `math/src/evt/types.rs`: Public parameter/config/result carriers and workspace types for EVT workflows.

---

## `math::finance`

### Artifacts

- Inventory: `math/src/finance/docs/inventory.md`
- Scope: `math/src/finance/docs/scope.md`
- Spec: `math/src/finance/docs/FINANCE_SPEC.md`
- Math review: `math/src/finance/docs/reviews/finance_module_math_review.md`
- Bench log: `math/src/finance/docs/finance_bench_results.md`
- Tests: `math/src/finance/tests`
- Benches: `math/benches/finance_candles_geometry.rs`
- Benches: `math/benches/finance_candles_returns.rs`
- Benches: `math/benches/finance_flow_volume_clock.rs`
- Benches: `math/benches/finance_inflection_range_expansion.rs`
- Benches: `math/benches/finance_inflection_rolling_extrema.rs`
- Benches: `math/benches/finance_path_geometry_log_path.rs`
- Benches: `math/benches/finance_risk_drawdown.rs`
- Benches: `math/benches/finance_volatility_garman_klass.rs`
- Benches: `math/benches/finance_volatility_parkinson.rs`
- Benches: `math/benches/finance_volatility_realized.rs`
- Benches: `math/benches/finance_volatility_realized_decomposition.rs`
- Benches: `math/benches/finance_volatility_rogers_satchell.rs`
- Benches: `math/benches/finance_volatility_yang_zhang.rs`

### Source Files

- `math/src/finance/candles/geometry.rs`: Candle geometry primitives (range/body/wicks and derived ratios).
- `math/src/finance/candles/mod.rs`: Candle submodule declarations and re-exports.
- `math/src/finance/candles/returns.rs`: Log-return primitives for gaps, intrabar moves, and close-to-close returns.
- `math/src/finance/flow/mod.rs`: Flow submodule declarations and re-exports (scope exception).
- `math/src/finance/flow/volume_clock.rs`: Volume-clock primitives and validated view for bucketed volume imbalance measures.
- `math/src/finance/inflection/mod.rs`: Inflection submodule declarations and re-exports.
- `math/src/finance/inflection/range_expansion.rs`: Range expansion ratio statistics (mean/median) with validated and workspace variants.
- `math/src/finance/inflection/rolling_extrema.rs`: Distances to rolling highs/lows with validated and workspace variants.
- `math/src/finance/mod.rs`: Module root for finance primitives; declares submodules and re-exports.
- `math/src/finance/path_geometry/log_path.rs`: `ValidatedClose` view and log-path geometry metrics from close series.
- `math/src/finance/path_geometry/mod.rs`: Path-geometry submodule declarations and re-exports.
- `math/src/finance/risk/drawdown.rs`: Drawdown statistics from close series (raw and validated).
- `math/src/finance/risk/mod.rs`: Risk submodule declarations and re-exports.
- `math/src/finance/volatility/garman_klass.rs`: Garman–Klass OHLC variance/volatility estimator (raw and validated).
- `math/src/finance/volatility/mod.rs`: Volatility submodule declarations and re-exports.
- `math/src/finance/volatility/parkinson.rs`: Parkinson OHLC range-based variance/volatility estimator (raw and validated).
- `math/src/finance/volatility/realized.rs`: Close-based realized variance/volatility and semivariance primitives (raw and validated).
- `math/src/finance/volatility/realized_decomposition.rs`: Realized volatility decomposition (e.g., RV/BV/JV) from close series.
- `math/src/finance/volatility/rogers_satchell.rs`: Rogers–Satchell OHLC variance/volatility estimators (raw and validated).
- `math/src/finance/volatility/utils.rs`: Shared utilities used by OHLC and realized volatility computations.
- `math/src/finance/volatility/validated.rs`: `ValidatedOhlc` input validator/view for OHLC volatility estimators.
- `math/src/finance/volatility/yang_zhang.rs`: Yang–Zhang OHLC variance/volatility estimator (raw and validated).

---

## `math::graph`

### Artifacts

- Inventory: `math/src/graph/docs/inventory.md`
- Scope: `math/src/graph/docs/scope.md`
- Spec: `math/src/graph/docs/GRAPH_SPEC.md`
- Math review: `math/src/graph/docs/reviews/graph_module_math_review.md`
- Bench log: `math/src/graph/docs/graph_bench_results.md`
- Tests: `math/src/graph/tests`
- Benches: `math/benches/graph.rs`

### Source Files

- `math/src/graph/knn.rs`: k-nearest-neighbor sparse edge construction from condensed distance matrices.
- `math/src/graph/maxst.rs`: Maximum spanning tree (MaxST) constructors (Kruskal/Prim) over condensed weight/similarity matrices.
- `math/src/graph/metrics.rs`: Tree/graph scalar metrics computed from edges (degree stats, diameters, path lengths, quantiles).
- `math/src/graph/mod.rs`: Module root for graph/MST utilities; declares submodules and re-exports public APIs.
- `math/src/graph/mst.rs`: Minimum spanning tree (MST) constructors (Kruskal/Prim) over condensed distance matrices.
- `math/src/graph/output.rs`: Output assembly helpers (summaries, optional edges/linkage steps) for graph constructors.
- `math/src/graph/sparse.rs`: MST construction from explicit sparse edge lists (distance interpretation).
- `math/src/graph/types.rs`: Graph configuration and output types (edges, linkage steps, summaries).
- `math/src/graph/workspace.rs`: Workspace buffers for allocation reuse in graph constructors.

---

## `math::hawkes`

### Artifacts

- Inventory: `math/src/hawkes/docs/inventory.md`
- Scope: `math/src/hawkes/docs/scope.md`
- Spec: `math/src/hawkes/docs/HAWKES_SPEC.md`
- Math review: `math/src/hawkes/docs/reviews/hawkes_module_math_review.md`
- Bench log: `math/src/hawkes/docs/hawkes_bench_results.md`
- Tests: `math/src/hawkes/tests`
- Benches: `math/benches/hawkes.rs`
- Benches: `math/benches/hawkes_alpha.rs`
- Benches: `math/benches/hawkes_erlang2.rs`
- Benches: `math/benches/hawkes_inference.rs`
- Benches: `math/benches/hawkes_long_tail_basis.rs`
- Benches: `math/benches/hawkes_marked.rs`
- Benches: `math/benches/hawkes_multi_exp.rs`
- Benches: `math/benches/hawkes_multivariate.rs`
- Benches: `math/benches/hawkes_piecewise_mu.rs`
- Benches: `math/benches/hawkes_power_law.rs`
- Benches: `math/benches/hawkes_univariate.rs`

### Source Files

- `math/src/hawkes/gof.rs`: Optional goodness-of-fit primitive for univariate fits (config-gated).
- `math/src/hawkes/inference.rs`: Post-fit inference utilities (AIC/BIC, covariance/standard errors via observed information).
- `math/src/hawkes/kernels/alpha.rs`: Rise-then-decay (“alpha”) kernel Hawkes fitting routines.
- `math/src/hawkes/kernels/erlang2.rs`: Erlang-2 kernel Hawkes fitting routines.
- `math/src/hawkes/kernels/long_tail_basis.rs`: Long-tail basis (mixture-of-exponentials) kernel approximation and fit helpers.
- `math/src/hawkes/kernels/mod.rs`: Kernel-family submodule declarations and re-exports.
- `math/src/hawkes/kernels/multi_exp.rs`: Multi-exponential kernel Hawkes fitting routines.
- `math/src/hawkes/kernels/power_law.rs`: Power-law (Omori) kernel fitting using deterministic approximation and grid search.
- `math/src/hawkes/marked.rs`: Marked exponential Hawkes MLE fit (with workspace variant).
- `math/src/hawkes/mod.rs`: Module root for Hawkes process estimation; declares submodules and re-exports.
- `math/src/hawkes/multivariate.rs`: Multivariate (bivariate) exponential Hawkes fit and multivariate log-likelihood/gradient utilities.
- `math/src/hawkes/optimization.rs`: Deterministic bounded optimizer used by Hawkes fitting routines.
- `math/src/hawkes/piecewise_mu/fit.rs`: Piecewise-baseline Hawkes fitting entry points (2/4/8 segments).
- `math/src/hawkes/piecewise_mu/mod.rs`: Piecewise-baseline (`mu(t)`) submodule declarations and re-exports.
- `math/src/hawkes/piecewise_mu/objective.rs`: Objective (negative log-likelihood) evaluation for piecewise-baseline fits.
- `math/src/hawkes/piecewise_mu/types.rs`: Types and workspace definitions for piecewise-baseline fits.
- `math/src/hawkes/piecewise_mu/utils.rs`: Helpers for breakpoint validation, segment bookkeeping, and deterministic utilities.
- `math/src/hawkes/preprocess.rs`: Timestamp/mark preprocessing and deterministic tie aggregation utilities.
- `math/src/hawkes/types.rs`: Public configuration and fit-result types for Hawkes estimators (univariate, marked, multivariate).
- `math/src/hawkes/univariate.rs`: Univariate exponential Hawkes MLE fit (with workspace variant).
- `math/src/hawkes/workspace.rs`: Workspace buffers for allocation reuse across Hawkes fits and diagnostics.

---

## `math::hurst`

### Artifacts

- Inventory: `math/src/hurst/docs/inventory.md`
- Scope: `math/src/hurst/docs/scope.md`
- Spec: `math/src/hurst/docs/HURST_SPEC.md`
- Math review: `math/src/hurst/docs/reviews/hurst_module_math_review.md`
- Bench log: `math/src/hurst/docs/hurst_bench_results.md`
- Tests: `math/src/hurst/tests`
- Benches: `math/benches/hurst_bias_finite_sample.rs`
- Benches: `math/benches/hurst_consensus.rs`
- Benches: `math/benches/hurst_consensus_aggregate.rs`
- Benches: `math/benches/hurst_consensus_fast_lane.rs`
- Benches: `math/benches/hurst_dfa.rs`
- Benches: `math/benches/hurst_dfa_ci.rs`
- Benches: `math/benches/hurst_dfa_fast_ci.rs`
- Benches: `math/benches/hurst_gph.rs`
- Benches: `math/benches/hurst_gph_ci.rs`
- Benches: `math/benches/hurst_gph_fast_ci.rs`
- Benches: `math/benches/hurst_mfdfa_h2.rs`
- Benches: `math/benches/hurst_mfdfa_h2_ci.rs`
- Benches: `math/benches/hurst_mfdfa_h2_fast_ci.rs`
- Benches: `math/benches/hurst_robinson.rs`
- Benches: `math/benches/hurst_robinson_ci.rs`
- Benches: `math/benches/hurst_robinson_fast_ci.rs`
- Benches: `math/benches/hurst_rs.rs`
- Benches: `math/benches/hurst_rs_ci.rs`
- Benches: `math/benches/hurst_rs_fast_ci.rs`
- Benches: `math/benches/hurst_variogram.rs`
- Benches: `math/benches/hurst_variogram_ci.rs`
- Benches: `math/benches/hurst_variogram_fast_ci.rs`
- Benches: `math/benches/hurst_wavelet_variance.rs`
- Benches: `math/benches/hurst_wavelet_variance_ci.rs`
- Benches: `math/benches/hurst_wavelet_variance_fast_ci.rs`

### Source Files

- `math/src/hurst/bias/mod.rs`: Finite-sample bias-correction interface and table selection/mapping.
- `math/src/hurst/bias/tables_n1000.rs`: Finite-sample bias calibration tables for `n = 1000` (increments-only).
- `math/src/hurst/bias/tables_n10000.rs`: Finite-sample bias calibration tables for `n = 10000` (increments-only).
- `math/src/hurst/ci/bootstrap.rs`: Bootstrap confidence intervals for Hurst estimators (explicit seed; workspace variants).
- `math/src/hurst/ci/fast.rs`: Deterministic “fast CI” approximations for log-log and spectral estimators (workspace variants).
- `math/src/hurst/ci/mod.rs`: Confidence-interval submodule declarations and re-exports.
- `math/src/hurst/consensus/aggregation.rs`: Aggregation logic for consensus weighting/disagreement summaries across estimator families.
- `math/src/hurst/consensus/engine.rs`: Consensus engine that runs a configurable set of estimators and produces aggregated measurements.
- `math/src/hurst/consensus/mod.rs`: Consensus submodule declarations and re-exports.
- `math/src/hurst/consensus/types.rs`: Consensus configuration and diagnostic output types.
- `math/src/hurst/estimators/dfa.rs`: DFA-based Hurst estimator (with workspace and optional regression diagnostics).
- `math/src/hurst/estimators/mfdfa_h2.rs`: MF-DFA wrapper returning `H = h(2)` (uses `signal::multifractal`; with workspace and optional diagnostics).
- `math/src/hurst/estimators/mod.rs`: Estimator submodule declarations and re-exports.
- `math/src/hurst/estimators/rs.rs`: Rescaled-range (R/S) Hurst estimator (with workspace and optional diagnostics).
- `math/src/hurst/estimators/spectral.rs`: Spectral Hurst wrappers (GPH and Robinson/local-Whittle) built on econometrics estimators.
- `math/src/hurst/estimators/variogram.rs`: Variogram-based Hurst estimator (with workspace and optional diagnostics).
- `math/src/hurst/estimators/wavelet_variance.rs`: Wavelet-variance (MODWT-family) Hurst estimator (with workspace and optional diagnostics).
- `math/src/hurst/mod.rs`: Module root for Hurst exponent estimation; declares submodules and re-exports.
- `math/src/hurst/types.rs`: Public types for Hurst method selection, configuration, and point-estimate outputs (including bias-correction options).

---

## `math::linalg`

### Artifacts

- Inventory: `math/src/linalg/docs/inventory.md`
- Scope: `math/src/linalg/docs/scope.md`
- Spec: `math/src/linalg/docs/LINALG_SPEC.md`
- Math review: `math/src/linalg/docs/reviews/linalg_module_math_review.md`
- Bench log: `math/src/linalg/docs/linalg_bench_results.md`
- Tests: `math/src/linalg/tests`
- Benches: `math/benches/linalg_cholesky.rs`
- Benches: `math/benches/linalg_least_squares_qr.rs`
- Benches: `math/benches/linalg_least_squares_qr_pivoted.rs`
- Benches: `math/benches/linalg_qr.rs`
- Benches: `math/benches/linalg_solve_spd.rs`

### Source Files

- `math/src/linalg/cholesky.rs`: Cholesky factorization routines for symmetric positive definite (SPD) matrices (lower-triangular).
- `math/src/linalg/mod.rs`: Module root; declares linear-algebra submodules and re-exports public APIs.
- `math/src/linalg/qr.rs`: QR decomposition and QR-based least-squares solvers (including pivoted variants) with workspace support.
- `math/src/linalg/solve.rs`: High-level linear-system solvers built on Cholesky/triangular primitives (including SPD solves).
- `math/src/linalg/triangular.rs`: Internal triangular-solve kernels used by decompositions and solvers.
- `math/src/linalg/validation.rs`: Input and shape validation helpers for linear-algebra routines.
- `math/src/linalg/workspace.rs`: Workspace structs used to reuse allocations across repeated linalg calls.

---

## `math::markov`

### Artifacts

- Inventory: `math/src/markov/docs/inventory.md`
- Scope: `math/src/markov/docs/scope.md`
- Spec: `math/src/markov/docs/MARKOV_SPEC.md`
- Math review: `math/src/markov/docs/reviews/markov_module_math_review.md`
- Bench log: `math/src/markov/docs/markov_bench_results.md`
- Tests: `math/src/markov/tests`
- Benches: `math/benches/markov_chain_entropy_rate.rs`
- Benches: `math/benches/markov_chain_fit.rs`
- Benches: `math/benches/markov_chain_mfpt.rs`
- Benches: `math/benches/markov_chain_spectral_gap.rs`
- Benches: `math/benches/markov_chain_stationary.rs`
- Benches: `math/benches/markov_hmm_baum_welch_gaussian_diag.rs`
- Benches: `math/benches/markov_hmm_baum_welch_gaussian_diag_mixture.rs`
- Benches: `math/benches/markov_hmm_baum_welch_gaussian_diag_robust.rs`
- Benches: `math/benches/markov_hmm_baum_welch_gaussian_diag_robust_fused.rs`
- Benches: `math/benches/markov_hmm_filtered_last.rs`
- Benches: `math/benches/markov_hmm_forward.rs`
- Benches: `math/benches/markov_hmm_forward_backward.rs`
- Benches: `math/benches/markov_hmm_forward_backward_fused.rs`
- Benches: `math/benches/markov_hmm_forward_student_t_diag.rs`
- Benches: `math/benches/markov_hmm_model_selection.rs`
- Benches: `math/benches/markov_hmm_posterior_entropy.rs`
- Benches: `math/benches/markov_hmm_transition_metrics.rs`
- Benches: `math/benches/markov_hmm_viterbi.rs`
- Benches: `math/benches/markov_hmm_xi_counts.rs`
- Benches: `math/benches/markov_hsmm_filter.rs`

### Source Files

- `math/src/markov/chain/fit.rs`: MLE/Laplace-smoothed estimation of transition matrices from observed state sequences.
- `math/src/markov/chain/hitting_times.rs`: mean-first-passage times (MFPT) via the Kemeny–Snell fundamental matrix (small `k`).
- `math/src/markov/chain/metrics.rs`: deterministic scalar metrics for transition matrices.
- `math/src/markov/chain/mod.rs`: Markov chain submodule exports.
- `math/src/markov/chain/spectral_gap.rs`: spectral gap proxy `1 - |λ₂|` via bounded Schur decomposition.
- `math/src/markov/chain/stationary.rs`: stationary distribution via time-bounded power iteration.
- `math/src/markov/chain/workspace.rs`: workspace buffers for stationary distribution.
- `math/src/markov/hmm/emissions/gaussian_diag.rs`: diagonal Gaussian emission model (log-space log-pdf).
- `math/src/markov/hmm/emissions/gaussian_diag_mixture.rs`: Gaussian diagonal mixture emission model (fixed component count `m`).
- `math/src/markov/hmm/emissions/mod.rs`: emissions module exports.
- `math/src/markov/hmm/emissions/student_t_diag.rs`: Student-t diagonal emission model (heavy-tail, robust likelihood; fixed `ν`).
- `math/src/markov/hmm/filtered.rs`: filtered (online) posterior last-step computation from a scaled forward recursion.
- `math/src/markov/hmm/inference.rs`: forward (loglik-only), forward-backward (gamma), and Viterbi in log-space.
- `math/src/markov/hmm/metrics.rs`: posterior-uncertainty measurements from `gamma` (entropy and effective states).
- `math/src/markov/hmm/mod.rs`: HMM submodule exports.
- `math/src/markov/hmm/model_selection.rs`: AIC/BIC scalars with explicit parameter counting for supported model families.
- `math/src/markov/hmm/training/gaussian_diag.rs`: Baum–Welch EM for diagonal Gaussian emissions.
- `math/src/markov/hmm/training/gaussian_diag_mixture/e_step.rs`: mixture E-step (forward-backward) producing `gamma` and expected transition counts.
- `math/src/markov/hmm/training/gaussian_diag_mixture/fit.rs`: Baum–Welch EM loop updating `(pi, A)` and mixture-emission parameters from sufficient statistics.
- `math/src/markov/hmm/training/gaussian_diag_mixture/init.rs`: deterministic initialization and init+fit wrapper for mixture training.
- `math/src/markov/hmm/training/gaussian_diag_mixture/mod.rs`: mixture-training submodule exports (init, fit, workspace).
- `math/src/markov/hmm/training/gaussian_diag_mixture/validate.rs`: mixture-training config validation (rejects unsupported options like robust preprocessing).
- `math/src/markov/hmm/training/gaussian_diag_mixture/workspace.rs`: workspace buffers for mixture training (gamma, xi counts, emission accumulators, scratch).
- `math/src/markov/hmm/training/init.rs`: deterministic initialization policies for training.
- `math/src/markov/hmm/training/mod.rs`: training submodule exports (Baum–Welch configs/results and model-specific fit entry points).
- `math/src/markov/hmm/training/robust.rs`: deterministic robust preprocessing (winsorization).
- `math/src/markov/hmm/training/types.rs`: training config/result types (`BaumWelchConfig`, `BaumWelchResult`) and validation.
- `math/src/markov/hmm/training/workspace.rs`: training workspace buffers (`BaumWelchWorkspace`).
- `math/src/markov/hmm/transition_metrics.rs`: HMM transition-matrix measurements derived from `A` (stationary distribution, entropies, durations, stickiness).
- `math/src/markov/hmm/types.rs`: `Hmm` type and emission model trait.
- `math/src/markov/hmm/workspace.rs`: HMM workspace buffers (logA/logpi, alpha, beta, delta, psi, scratch).
- `math/src/markov/hmm/xi.rs`: expected transition counts (xi sufficient statistics) and a fused gamma+xi path (no `T*k*k` allocation).
- `math/src/markov/hsmm/duration.rs`: duration PMF validation + hazard/survival/expected-remaining precompute (`DurationModel`).
- `math/src/markov/hsmm/filtering.rs`: HSMM filtering (scaled probability-space) over expanded `(state, age)` recursion.
- `math/src/markov/hsmm/mod.rs`: HSMM submodule exports.
- `math/src/markov/hsmm/types.rs`: `Hsmm` type (HMM + explicit bounded duration model).
- `math/src/markov/hsmm/workspace.rs`: HSMM workspace buffers.
- `math/src/markov/mod.rs`: module entrypoint (exports).
- `math/src/markov/transition.rs`: validation for probability vectors and row-stochastic matrices.

---

## `math::ml`

### Artifacts

- Inventory: `math/src/ml/docs/inventory.md`
- Scope: `math/src/ml/docs/scope.md`
- Spec: `math/src/ml/docs/ML_SPEC.md`
- Math review: `math/src/ml/docs/reviews/ml_module_math_review.md`
- Bench log: `math/src/ml/docs/ml_bench_results.md`
- Tests: `math/src/ml/tests`
- Benches: `math/benches/ml_change_detection.rs`
- Benches: `math/benches/ml_clustering.rs`
- Benches: `math/benches/ml_clustering_metrics.rs`
- Benches: `math/benches/ml_density.rs`
- Benches: `math/benches/ml_mixture.rs`
- Benches: `math/benches/ml_pca.rs`
- Benches: `math/benches/ml_preprocess.rs`

### Source Files

- `math/src/ml/change_detection/mod.rs`: Change-detection submodule declarations and re-exports.
- `math/src/ml/change_detection/page_hinkley.rs`: Page–Hinkley change detection implementation.
- `math/src/ml/change_detection/types.rs`: Change-detection config/output types (Page–Hinkley, mean/variance shift wrappers).
- `math/src/ml/change_detection/wrappers.rs`: Wrappers around structural-break statistics for change detection (no p-values).
- `math/src/ml/clustering/kmeans.rs`: Deterministic k-means clustering (with initialization policies, warm-start, and workspace support).
- `math/src/ml/clustering/metrics.rs`: Clustering evaluation metrics (silhouette, Davies–Bouldin, Calinski–Harabasz) with workspace support.
- `math/src/ml/clustering/mod.rs`: Clustering submodule declarations and re-exports.
- `math/src/ml/density/mahalanobis_diag.rs`: Diagonal Mahalanobis outlier score computation with workspace support.
- `math/src/ml/density/mod.rs`: Density/outlier submodule declarations and re-exports.
- `math/src/ml/density/types.rs`: Density model configuration/workspace types.
- `math/src/ml/mixture/mod.rs`: Mixture-model submodule declarations and re-exports.
- `math/src/ml/mixture/normal_1d_k2.rs`: Bounded EM fit for 1D Gaussian mixture with `K=2` (workspace variant).
- `math/src/ml/mixture/types.rs`: Gaussian mixture (1D K=2) configuration/model/workspace types.
- `math/src/ml/mod.rs`: Module root for deterministic ML primitives; declares submodules and re-exports.
- `math/src/ml/pca/fit.rs`: PCA fit and fit-transform routines (including workspace variants).
- `math/src/ml/pca/jacobi.rs`: Jacobi eigen decomposition for symmetric matrices used by PCA fitting.
- `math/src/ml/pca/mod.rs`: PCA submodule declarations and re-exports.
- `math/src/ml/pca/stats.rs`: Statistics helpers for PCA (means/covariances and related accumulators).
- `math/src/ml/pca/transform.rs`: PCA transform utilities (projecting data onto fitted components).
- `math/src/ml/pca/types.rs`: PCA configuration/model/workspace types.
- `math/src/ml/pca/validation.rs`: PCA input validation helpers (shape/finite checks and invariants).
- `math/src/ml/preprocess/mod.rs`: Preprocessing submodule declarations and re-exports.
- `math/src/ml/preprocess/scaling.rs`: Standardization and robust scaling fit/transform routines (with workspaces).
- `math/src/ml/preprocess/utils.rs`: Shared preprocessing utilities (matrix validation, percentile indexing) used across preprocess routines.
- `math/src/ml/preprocess/winsorize.rs`: Winsorization fit/transform routines (with workspaces).

---

## `math::multifractal`

### Artifacts

- Inventory: `math/src/multifractal/docs/inventory.md`
- Scope: `math/src/multifractal/docs/scope.md`
- Spec: `math/src/multifractal/docs/MULTIFRACTAL_SPEC.md`
- Math review: `math/src/multifractal/docs/reviews/multifractal_module_math_review.md`
- Bench log: `math/src/multifractal/docs/multifractal_bench_results.md`
- Tests: `math/src/multifractal/tests`
- Benches: `math/benches/multifractal_dcca_rho.rs`
- Benches: `math/benches/multifractal_fit_range.rs`
- Benches: `math/benches/multifractal_global_fit_range.rs`
- Benches: `math/benches/multifractal_legendre.rs`
- Benches: `math/benches/multifractal_mfdcca_hxy.rs`
- Benches: `math/benches/multifractal_mfdfa_hq.rs`
- Benches: `math/benches/multifractal_mfdma_hq.rs`
- Benches: `math/benches/multifractal_robust_fit.rs`
- Benches: `math/benches/multifractal_rolling_mfdfa_spectrum.rs`
- Benches: `math/benches/multifractal_wlm_cumulants.rs`
- Benches: `math/benches/multifractal_wlm_zeta.rs`
- Benches: `math/benches/multifractal_wtmm_tau.rs`

### Source Files

- `math/src/multifractal/dcca_rho.rs`: scale-dependent DCCA correlation coefficient `ρ_DCCA(s)` endpoint.
- `math/src/multifractal/fit.rs`: shared deterministic linear regression utilities (OLS + opt-in Huber IRLS).
- `math/src/multifractal/fit_range.rs`: deterministic fit-range selector for a single log–log regression.
- `math/src/multifractal/global_fit_range.rs`: deterministic global fit-range selector across all `q` (one shared window).
- `math/src/multifractal/mfdcca.rs`: MF-DCCA post-processing endpoint (fits `H_xy(q)` from MF-DCCA fluctuation functions).
- `math/src/multifractal/mfdfa.rs`: MF-DFA post-processing endpoint (fits `H(q)` from MF-DFA fluctuation functions).
- `math/src/multifractal/mfdma.rs`: MF-DMA post-processing endpoint (fits `H(q)` from MF-DMA fluctuation functions).
- `math/src/multifractal/mod.rs`: module entrypoint (exports).
- `math/src/multifractal/regularized_legendre.rs`: opt-in deterministic regularization / concavity enforcement before Legendre transform.
- `math/src/multifractal/rolling.rs`: bounded rolling MF-DFA spectrum descriptors (time-bounded by `max_windows`).
- `math/src/multifractal/spectrum.rs`: Legendre transform `tau(q) -> (alpha(q), f(alpha))` and spectrum descriptors.
- `math/src/multifractal/types.rs`: public types for configs, fits, diagnostics, selectors.
- `math/src/multifractal/wavelet_leader_cumulants.rs`: wavelet leader log-cumulants endpoint (`c1,c2,c3` fits).
- `math/src/multifractal/wavelet_leaders.rs`: wavelet leaders multifractal endpoint (fits `ζ(q)`).
- `math/src/multifractal/wtmm.rs`: WTMM post-processing endpoint (fits `tau(q)` from WTMM partition functions).

---

## `math::signal`

### Artifacts

- Inventory: `math/src/signal/docs/inventory.md`
- Scope: `math/src/signal/docs/scope.md`
- Spec: `math/src/signal/docs/SIGNAL_SPEC.md`
- Math review: `math/src/signal/docs/reviews/signal_module_math_review.md`
- Bench log: `math/src/signal/docs/signal_bench_results.md`
- Tests: `math/src/signal/tests`
- Benches: `math/benches/signal_biquad.rs`
- Benches: `math/benches/signal_cross_spectrum.rs`
- Benches: `math/benches/signal_dcca.rs`
- Benches: `math/benches/signal_detrending.rs`
- Benches: `math/benches/signal_dfa.rs`
- Benches: `math/benches/signal_entropy.rs`
- Benches: `math/benches/signal_goertzel.rs`
- Benches: `math/benches/signal_hilbert.rs`
- Benches: `math/benches/signal_instantaneous_frequency.rs`
- Benches: `math/benches/signal_kalman.rs`
- Benches: `math/benches/signal_lomb_scargle.rs`
- Benches: `math/benches/signal_modwt_detail_energy_levels.rs`
- Benches: `math/benches/signal_modwt_families.rs`
- Benches: `math/benches/signal_mse.rs`
- Benches: `math/benches/signal_multifractal.rs`
- Benches: `math/benches/signal_multitaper.rs`
- Benches: `math/benches/signal_phase_coherence.rs`
- Benches: `math/benches/signal_rainflow.rs`
- Benches: `math/benches/signal_rqa.rs`
- Benches: `math/benches/signal_savgol.rs`
- Benches: `math/benches/signal_shape.rs`
- Benches: `math/benches/signal_spectral.rs`
- Benches: `math/benches/signal_ssa.rs`
- Benches: `math/benches/signal_stft_windowed.rs`
- Benches: `math/benches/signal_wavelet_coherence.rs`
- Benches: `math/benches/signal_wavelet_denoise.rs`
- Benches: `math/benches/signal_wavelets.rs`
- Benches: `math/benches/signal_welch.rs`
- Benches: `math/benches/signal_zero_crossing.rs`

### Source Files

- `math/src/signal/dcca.rs`: Detrended cross-correlation analysis (DCCA) primitives with workspace support.
- `math/src/signal/detrending.rs`: Deterministic detrending utilities (mean/linear/polynomial) with QR-based workspaces.
- `math/src/signal/dfa.rs`: DFA helper primitives (integration, segment fluctuation RMS, window-size generation).
- `math/src/signal/entropy.rs`: Deterministic entropy measures (e.g., permutation entropy, sample entropy) with workspaces.
- `math/src/signal/filtering/biquad.rs`: Biquad IIR filters (DF2T) with stable coefficient factories and validation.
- `math/src/signal/filtering/kalman/diagnostics.rs`: Innovation/likelihood diagnostics types for Kalman filters.
- `math/src/signal/filtering/kalman/local_level.rs`: 1D local-level Kalman filter/smoother primitives (with optional diagnostics).
- `math/src/signal/filtering/kalman/local_linear_trend.rs`: 1D local linear-trend Kalman filter primitives (with optional diagnostics).
- `math/src/signal/filtering/kalman/mod.rs`: Kalman filtering submodule declarations and re-exports.
- `math/src/signal/filtering/kalman/utils.rs`: Shared Kalman recursion helpers and numerically-stable primitives.
- `math/src/signal/filtering/mod.rs`: Filtering submodule declarations and re-exports.
- `math/src/signal/filtering/savgol.rs`: Savitzky–Golay filtering (with cached coefficient workspace).
- `math/src/signal/mod.rs`: Module root for signal-processing primitives; declares submodules and re-exports.
- `math/src/signal/mse.rs`: Multiscale entropy (MSE) computation with workspace support.
- `math/src/signal/multifractal/mfdfa.rs`: MF-DFA implementation producing multifractal fluctuation functions (workspace-supported).
- `math/src/signal/multifractal/mfdma.rs`: MF-DMA implementation for multifractal analysis (workspace-supported).
- `math/src/signal/multifractal/mod.rs`: Multifractal submodule declarations and re-exports.
- `math/src/signal/multifractal/wtmm.rs`: WTMM (wavelet transform modulus maxima) partition-function primitives (workspace-supported).
- `math/src/signal/rainflow.rs`: Bounded deterministic open-record rainflow cycle extraction with original extrema indices and explicit finite-arithmetic/failure contracts.
- `math/src/signal/rqa.rs`: Recurrence quantification analysis (RQA) metrics with workspace support.
- `math/src/signal/shape.rs`: Time-series shape descriptors (Hjorth parameters and spectral-shape scalars).
- `math/src/signal/spectral/autocorrelation_fft.rs`: FFT-based autocorrelation computation with workspace support.
- `math/src/signal/spectral/coherence.rs`: Magnitude-squared coherence estimation between two signals (workspace-supported).
- `math/src/signal/spectral/cross_spectrum.rs`: Cross-spectrum and cross-phase/group-delay primitives (workspace-supported).
- `math/src/signal/spectral/dpss.rs`: DPSS (Slepian) taper generation utilities for multitaper spectral estimation.
- `math/src/signal/spectral/fft.rs`: FFT utilities used by spectral estimators.
- `math/src/signal/spectral/goertzel.rs`: Goertzel single-/multi-bin power estimation with workspace support.
- `math/src/signal/spectral/hilbert.rs`: Hilbert transform / analytic signal primitives (phase unwrap and instantaneous frequency helpers).
- `math/src/signal/spectral/lomb_scargle.rs`: Lomb–Scargle periodogram for irregularly sampled time series (workspace-supported).
- `math/src/signal/spectral/mod.rs`: Spectral submodule declarations and re-exports.
- `math/src/signal/spectral/multitaper.rs`: Multitaper power spectral density estimation (DPSS) with workspace support.
- `math/src/signal/spectral/periodogram.rs`: Periodogram computation with optional detrending and workspace support.
- `math/src/signal/spectral/phase_coherence.rs`: Phase-locking value and phase coherence primitives (workspace-supported).
- `math/src/signal/spectral/precomputed_dpss.rs`: Precomputed DPSS tapers/constants used for deterministic multitaper setups.
- `math/src/signal/spectral/time_varying.rs`: Time-frequency periodograms (STFT-style) and flat output variants with workspace support.
- `math/src/signal/spectral/welch.rs`: Welch power spectral density estimation with workspace support.
- `math/src/signal/spectral/windows.rs`: Window coefficient generation and window application primitives.
- `math/src/signal/ssa.rs`: Singular spectrum analysis (SSA) decomposition utilities with workspace support.
- `math/src/signal/types.rs`: Shared enums/types for detrending methods, wavelet families, and spectral window functions.
- `math/src/signal/wavelet_coherence.rs`: Wavelet coherence computation between two signals (workspace-supported).
- `math/src/signal/wavelets.rs`: MODWT-based wavelet detail coefficients, wavelet variance, and wavelet denoising primitives.
- `math/src/signal/zero_crossing.rs`: Zero-crossing detection and related counting utilities.

---

## `math::state_space`

### Artifacts

- Inventory: `math/src/state_space/docs/inventory.md`
- Scope: `math/src/state_space/docs/scope.md`
- Spec: `math/src/state_space/docs/STATE_SPACE_SPEC.md`
- Math review: `math/src/state_space/docs/reviews/state_space_module_math_review.md`
- Bench log: `math/src/state_space/docs/state_space_bench_results.md`
- Tests: `math/src/state_space/tests`
- Benches: `math/benches/state_space_local_level.rs`
- Benches: `math/benches/state_space_local_level_contamination.rs`
- Benches: `math/benches/state_space_local_level_path.rs`
- Benches: `math/benches/state_space_local_level_student_t.rs`
- Benches: `math/benches/state_space_local_level_switching_r_m2.rs`
- Benches: `math/benches/state_space_local_level_switching_r_m2_path.rs`
- Benches: `math/benches/state_space_local_linear_trend.rs`
- Benches: `math/benches/state_space_local_linear_trend_contamination.rs`
- Benches: `math/benches/state_space_local_linear_trend_path.rs`
- Benches: `math/benches/state_space_local_linear_trend_student_t.rs`
- Benches: `math/benches/state_space_local_linear_trend_switching_r_m2.rs`
- Benches: `math/benches/state_space_local_linear_trend_switching_r_m2_path.rs`

### Source Files

- `math/src/state_space/local_level/contamination.rs`: Local-level robust filter with contamination-mixture observation model (last-only; diagnostics variants).
- `math/src/state_space/local_level/dt.rs`: Local-level Gaussian filter supporting irregular sampling via per-step `dt`.
- `math/src/state_space/local_level/masked.rs`: Local-level Gaussian filter supporting masked/missing observations (path and last-only variants).
- `math/src/state_space/local_level/mod.rs`: Local-level (1D) state-space filters: submodule declarations and re-exports.
- `math/src/state_space/local_level/student_t.rs`: Local-level robust filter with Student-t observation model (last-only; diagnostics variants).
- `math/src/state_space/local_level/switching_r_m2.rs`: Local-level switching observation-noise filter (IMM with fixed `M=2`) including path and diagnostics variants.
- `math/src/state_space/local_linear_trend/contamination.rs`: Local linear-trend robust filter with contamination-mixture observation model (last-only; diagnostics variants).
- `math/src/state_space/local_linear_trend/dt.rs`: Local linear-trend Gaussian filter supporting irregular sampling via per-step `dt`.
- `math/src/state_space/local_linear_trend/masked.rs`: Local linear-trend Gaussian filter supporting masked/missing observations.
- `math/src/state_space/local_linear_trend/masked_diagnostics.rs`: Local linear-trend masked filter variants that return innovation diagnostics and total log-likelihood.
- `math/src/state_space/local_linear_trend/mod.rs`: Local linear-trend (2D) state-space filters: submodule declarations and re-exports.
- `math/src/state_space/local_linear_trend/student_t.rs`: Local linear-trend robust filter with Student-t observation model (last-only; diagnostics variants).
- `math/src/state_space/local_linear_trend/switching_r_m2.rs`: Local linear-trend switching observation-noise filter (IMM with fixed `M=2`) including path and diagnostics variants.
- `math/src/state_space/mixture.rs`: Shared scalar Gaussian log-likelihood and `logaddexp` primitives used by robust/switching models.
- `math/src/state_space/mod.rs`: Module root for state-space (Kalman-family) filters; declares submodules and re-exports.

---

## `math::stochastic`

### Artifacts

- Inventory: `math/src/stochastic/docs/inventory.md`
- Scope: `math/src/stochastic/docs/scope.md`
- Spec: `math/src/stochastic/docs/STOCHASTIC_SPEC.md`
- Math review: `math/src/stochastic/docs/reviews/stochastic_module_math_review.md`
- Bench log: `math/src/stochastic/docs/stochastic_bench_results.md`
- Tests: `math/src/stochastic/tests`
- Benches: `math/benches/stochastic_bootstrap.rs`
- Benches: `math/benches/stochastic_bootstrap_ci.rs`
- Benches: `math/benches/stochastic_bootstrap_fused.rs`
- Benches: `math/benches/stochastic_bootstrap_paired.rs`
- Benches: `math/benches/stochastic_bootstrap_pwsd.rs`
- Benches: `math/benches/stochastic_bootstrap_resample.rs`
- Benches: `math/benches/stochastic_bootstrap_studentized.rs`
- Benches: `math/benches/stochastic_bootstrap_validate.rs`
- Benches: `math/benches/stochastic_bootstrap_validate_parallel.rs`
- Benches: `math/benches/stochastic_fgn.rs`
- Benches: `math/benches/stochastic_fgn_observed.rs`
- Benches: `math/benches/stochastic_innovations.rs`
- Benches: `math/benches/stochastic_innovations_batch.rs`
- Benches: `math/benches/stochastic_innovations_extended.rs`
- Benches: `math/benches/stochastic_lhs.rs`
- Benches: `math/benches/stochastic_monte_carlo.rs`
- Benches: `math/benches/stochastic_normal.rs`
- Benches: `math/benches/stochastic_qmc_halton.rs`
- Benches: `math/benches/stochastic_qmc_sobol.rs`
- Benches: `math/benches/stochastic_rqmc.rs`
- Benches: `math/benches/stochastic_variance_reduction.rs`
- Benches: `math/benches/stochastic_weighted_bootstrap.rs`
- Benches: `math/benches/stochastic_weights.rs`

### Source Files

- `math/src/stochastic/bootstrap/api.rs`: main bootstrap validation APIs (estimator-based and indices-based variants).
- `math/src/stochastic/bootstrap/bootstrap.rs`: bootstrap module wiring/re-exports for internal bootstrap implementation units.
- `math/src/stochastic/bootstrap/ci.rs`: bootstrap confidence intervals (percentile, BCa, studentized with deterministic fallback policies).
- `math/src/stochastic/bootstrap/fused.rs`: specialized fused bootstrap summaries (mean and unbiased variance; percentile CI only).
- `math/src/stochastic/bootstrap/mod.rs`: bootstrap module exports (types, workspace, and APIs).
- `math/src/stochastic/bootstrap/paired.rs`: paired bootstrap validation (single resampling plan applied to paired series).
- `math/src/stochastic/bootstrap/parallel.rs`: deterministic parallel bootstrap validation APIs (i.i.d. only).
- `math/src/stochastic/bootstrap/pwsd.rs`: Politis–White (2004) deterministic block-length selection heuristic.
- `math/src/stochastic/bootstrap/resample.rs`: i.i.d. and block-bootstrap resampling implementations (values and index plans).
- `math/src/stochastic/bootstrap/types.rs`: bootstrap configuration and result types (methods, CI policies).
- `math/src/stochastic/bootstrap/utils.rs`: bootstrap utilities (unbiased index sampling, stationary block lengths, deterministic seeding).
- `math/src/stochastic/bootstrap/workspace.rs`: bootstrap workspace buffers for allocation reuse and bounded scratch.
- `math/src/stochastic/gamma.rs`: gamma sampling primitives (Marsaglia–Tsang + shape-boost).
- `math/src/stochastic/innovations/alpha_stable.rs`: alpha-stable sampling (CMS-style) with strict parameter validation and special cases.
- `math/src/stochastic/innovations/approx.rs`: opt-in approximation policy definitions for sampling.
- `math/src/stochastic/innovations/distribution.rs`: innovation distribution dispatcher (sampling/validation/metadata).
- `math/src/stochastic/innovations/mod.rs`: innovation distribution exports and shared policy wiring.
- `math/src/stochastic/innovations/nig.rs`: Normal-Inverse-Gaussian sampling (inverse Gaussian mixture) and moments.
- `math/src/stochastic/innovations/skewed_student_t.rs`: two-piece deterministic skew transform applied to Student-t draws.
- `math/src/stochastic/innovations/student_t.rs`: Student-t sampling (normal/gamma mixture) and moments where defined.
- `math/src/stochastic/innovations/variance_gamma.rs`: Variance-Gamma sampling (gamma-normal mixture) and moments.
- `math/src/stochastic/lhs.rs`: deterministic Latin hypercube sampling point generation with workspace reuse.
- `math/src/stochastic/mod.rs`: module entrypoint and public re-exports for `math::stochastic`.
- `math/src/stochastic/monte_carlo.rs`: deterministic Monte Carlo wrappers for scalar expectations (IID/QMC backends, optional normal mappings).
- `math/src/stochastic/normal.rs`: normal sampling helpers (Box–Muller over `rand::Rng`).
- `math/src/stochastic/processes/fgn.rs`: fractional Gaussian noise generation (Davies–Harte circulant embedding; FFT-based).
- `math/src/stochastic/processes/fgn_observed.rs`: observed series generator combining fGn increments, drift, optional noise, and optional integration.
- `math/src/stochastic/processes/mod.rs`: time-series simulation primitives exports.
- `math/src/stochastic/processes/observation_noise.rs`: additive Gaussian observation-noise model (seeded or RNG-driven).
- `math/src/stochastic/qmc/halton.rs`: Halton low-discrepancy sequence with optional deterministic digit scrambling.
- `math/src/stochastic/qmc/mod.rs`: QMC sequence exports (Halton and Sobol).
- `math/src/stochastic/qmc/sobol.rs`: Sobol low-discrepancy sequence wrapper with deterministic scrambling and open-unit mapping.
- `math/src/stochastic/rng.rs`: deterministic RNG seeding/stream partitioning and `(0,1)` uniform + Box–Muller normal primitives.
- `math/src/stochastic/rqmc.rs`: randomized QMC estimator (replicate mean + stderr across deterministic scrambles).
- `math/src/stochastic/variance_reduction.rs`: variance reduction primitives (antithetic normals, stratified uniforms).
- `math/src/stochastic/weighted_bootstrap.rs`: Dirichlet-weighted (Bayesian) bootstrap validation and fast paths for weighted mean.
- `math/src/stochastic/weights.rs`: log-weight utilities (logsumexp, normalization) and ESS computations.

---

## `math::tda`

### Artifacts

- Inventory: `math/src/tda/docs/inventory.md`
- Scope: `math/src/tda/docs/scope.md`
- Spec: `math/src/tda/docs/TDA_SPEC.md`
- Math review: `math/src/tda/docs/reviews/tda_module_math_review.md`
- Bench log: `math/src/tda/docs/tda_bench_results.md`
- Tests: `math/src/tda/tests`
- Benches: `math/benches/tda.rs`

### Source Files

- `math/src/tda/approx/mod.rs`: opt-in approximate persistence on sparse clique complexes (deterministic kNN/radius graphs).
- `math/src/tda/approx/types.rs`: configuration types for explicit sparse approximation modes (kNN/radius) and budget caps.
- `math/src/tda/approx/workspace.rs`: workspace buffers for sparse graph construction and triangle enumeration.
- `math/src/tda/distance.rs`: condensed distance validation and indexing helpers (delegating to `math::core::condensed`).
- `math/src/tda/mod.rs`: module entrypoint and public re-exports for VR persistence, multiscale, and opt-in approximations.
- `math/src/tda/multiscale.rs`: baseline multiscale evaluation over an epsilon grid plus aggregated Betti-curve features.
- `math/src/tda/multiscale_incremental.rs`: incremental multiscale evaluation using a single pre-sorted global edge list and per-epsilon prefixes.
- `math/src/tda/persistence.rs`: exact VR persistence computation for H0/H1 (Z2) from condensed distances or pre-built edges/triangles.
- `math/src/tda/summaries.rs`: scalar summaries derived from persistence pairs (finite lifetimes + essential counts).
- `math/src/tda/types.rs`: configuration, persistence pair types, and summary output containers.
- `math/src/tda/union_find.rs`: re-export of `math::core::union_find::UnionFind` for DSU operations.
- `math/src/tda/vr.rs`: Vietoris–Rips filtration primitives (edge and triangle enumeration under `epsilon_max` with caps).
- `math/src/tda/workspace.rs`: workspace buffers for allocation reuse across persistence and multiscale calls.

---
