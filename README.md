# rainflow

Rainflow cycle counting for `f64` time series.

Standalone port of the rainflow cycle counter from the MATHILDE `math` crate (`math::signal`, spec entry RF-1). Algorithm, numerical contracts, and tests are unchanged from the source module; the pinned independent reference for the counting semantics is `iamlikeme/rainflow` commit `5fc6822` (ASTM E1049-85 cycle counting).

## Scope

This crate currently implements one public batch function:

- `rainflow_cycles(values: &[f64]) -> MathResult<Vec<RainflowCycle>>`

It returns the ordered open-record cycles of a finalized scalar record: range, mean, half/full weight, and the original extrema indices of each cycle. There is no binning, no damage model, and no hidden preprocessing.

Types and constants:

- `RainflowCycle { range, mean, count, start_index, end_index }` — `count` is exactly `0.5` (half) or `1.0` (full); endpoint indices identify extrema, not when a cycle became observable.
- `RAINFLOW_MAX_INPUT_LEN = 10_000` — batch safety bound; minimum input length is 2.

Boundary conventions:

- Interior plateau extrema use their last index; the first and last samples are always endpoints.
- Residual excursions are counted as half cycles.
- A constant record yields no cycles.
- Equal ranges are not merged.

All functions return `MathResult<T> = Result<T, MathError>`.

## Performance

Criterion benchmarks cover seven input classes (constant, monotone, alternating, nested, plateaus, residual, cascade) over `n ∈ {100, 1000, 10000}`, with asserted allocation probes and maximum-size tail observations (see `benches/rainflow.rs`).

Example results (from `docs/rainflow_bench_results.md`, Intel Xeon W-2295, Ubuntu 22.04.5, rustc 1.90.0):

| Input class  |      n |                          Time (low/med/high) | Med ns/sample |
| ------------ | -----: | -------------------------------------------: | ------------: |
| monotone     |    100 |                316.52 ns / 326.90 ns / 345.11 ns |          3.27 |
| alternating  |    100 |                 1.6278 µs / 1.6341 µs / 1.6396 µs |         16.34 |
| monotone     |  1,000 |                 2.2993 µs / 2.4606 µs / 2.7096 µs |          2.46 |
| alternating  |  1,000 |                14.969 µs / 15.410 µs / 16.355 µs |         15.41 |
| monotone     | 10,000 |                21.212 µs / 21.307 µs / 21.446 µs |          2.13 |
| alternating  | 10,000 |                147.55 µs / 148.32 µs / 149.53 µs |         14.83 |

`Med ns/sample` is computed as `(median time) / n` and is a rough per-observation cost. It depends on CPU, OS, Rust version, and compiler settings.

Full details (all classes, allocation probes, tail observations): `docs/rainflow_bench_results.md`.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
rainflow = "0.1"
```

```rust
use rainflow::{rainflow_cycles, MathResult};

fn main() -> MathResult<()> {
    let y = [-2.0, 1.0, -3.0, 5.0, -1.0, 3.0, -4.0, 4.0, -2.0];

    for cycle in rainflow_cycles(&y)? {
        // range, mean, count (0.5 half / 1.0 full), start_index, end_index
        println!(
            "range={} mean={} count={} [{}..{}]",
            cycle.range, cycle.mean, cycle.count, cycle.start_index, cycle.end_index
        );
    }
    Ok(())
}
```

## Error handling and input checks

`rainflow_cycles` validates:

- `values` must contain at least 2 samples (rejected as `InsufficientDataAlgo`).
- Length must not exceed `RAINFLOW_MAX_INPUT_LEN` (rejected as `InvalidData`).
- All values must be finite (`NaN`/`±inf` rejected as `InvalidData`).
- Any range or mean that is not representable is rejected as `NumericalError` (e.g. `[-f64::MAX, f64::MAX]`).
- Allocation failure is converted to a recoverable `CalculationError`; there is no partial successful output.

Errors are reported via `MathError` (see `src/errors.rs`).

## Tests

Run unit tests:

```bash
cargo test
```

Test sources: `src/tests/test_rainflow.rs` (26 tests: correctness, edge cases, numerical stability, failure contracts, panic safety, determinism, exhaustive small-input invariants, and a pinned independent reference fixture).

## Benchmarks

Benchmarks use Criterion (`criterion = "0.8.2"`).

Run benchmarks:

```bash
cargo bench
```

Bench source: `benches/rainflow.rs`. The benchmark log (append-only) is `docs/rainflow_bench_results.md`; raw output is kept in `benches/benches.log`.

## Inventory

The generated crate inventory is `inventory.md`; regenerate with:

```bash
cargo make inventory
```

Input for generation: `docs/inventory.md` (per-file purposes). Generator: `bin/generate_global_inventory.rs`.
