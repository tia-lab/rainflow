# `rainflow` — inventory

This inventory lists what exists in this crate: public API, docs, benches, and tests.

The crate is a standalone port of the rainflow cycle counter from `math::signal` (RF-1, SIGNAL_SPEC RF-1.1); algorithm, contracts, and tests are unchanged from the source module.

---

## 1) Module tree (source)

Flat single-module crate; everything is re-exported from `src/lib.rs`:

- `core`: `RainflowCycle` type and the `rainflow_cycles` batch function (validation, private counting helpers)
- `config`: `RAINFLOW_MAX_INPUT_LEN` batch safety bound
- `errors`: `MathError` enum
- `types`: `MathResult<T>` alias

---

## 2) Public API

All public APIs return `MathResult<T>`.

### 2.1 `rainflow`

File: `src/core.rs`

- `rainflow_cycles(values: &[f64]) -> MathResult<Vec<RainflowCycle>>`: ordered open-record cycle ranges, means, half/full weights, and original extrema indices; no binning or hidden preprocessing.
- `RainflowCycle`: fields `range`, `mean`, `count`, `start_index`, `end_index`; endpoint indices are not availability timestamps.
- `RAINFLOW_MAX_INPUT_LEN = 10_000`: named batch safety bound; minimum length two.

Boundary conventions: interior plateau extrema use their last index; the first/last samples are endpoints; residual excursions are half cycles; a constant record yields no cycles; equal ranges are not merged.

Failures: `InsufficientDataAlgo`, `InvalidData`, `NumericalError`, recoverable reservation `CalculationError`; no partial successful output.

---

## 3) Determinism and allocation discipline

- Deterministic: no RNG, no call history, input never mutated, never sorted.
- O(n) time and space; two bounded reservations for nonconstant inputs, none for constant inputs; no per-sample allocation.

---

## 4) Benchmarks and logs

- Bench: `benches/rainflow.rs`, Criterion target `rainflow`; seven input classes over the standard sizes `n ∈ {100, 1000, 10000}`, with allocation probes and maximum-size tail observations.
- Append-only benchmark log: `docs/rainflow_bench_results.md`.
- Raw bench stdout: `benches/benches.log`.

---

## 5) Tests

- `src/tests/test_rainflow.rs`: 26 contract tests (correctness, edge cases, numerical stability, failure contracts, panic-safety, determinism, exhaustive small-input invariants, pinned independent reference fixture).
- Registered by `src/tests/mod.rs`.

---

## 6) Source Files (1-line purposes; required for global inventory)

- `src/lib.rs`: Crate entrypoint; declares modules and re-exports `rainflow_cycles`, `RainflowCycle`, `RAINFLOW_MAX_INPUT_LEN`, `MathError`, `MathResult`.
- `src/config.rs`: Batch safety bound `RAINFLOW_MAX_INPUT_LEN` (10,000 samples).
- `src/core.rs`: Rainflow cycle extraction: `RainflowCycle` type, input validation, private counting helpers, and the public `rainflow_cycles` batch function.
- `src/errors.rs`: `MathError` enum for deterministic failure reporting at the API boundary.
- `src/types.rs`: `MathResult<T>` result alias.
- `bin/generate_global_inventory.rs`: Deterministic generator that rewrites `inventory.md` from this inventory; run via `cargo make inventory`.
