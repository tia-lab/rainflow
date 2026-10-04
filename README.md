# rainflow

Minimal rainflow cycle counting for `f64` time series.

## Scope

This crate currently implements one batch function:

- `rainflow_cycles`: extracts full and half cycles from a completed time series.

The function returns a `Vec<RainflowCycle>`. Each cycle contains:

- `range`: distance between its low and high values.
- `mean`: midpoint of those values.
- `count`: `1.0` for a full cycle or `0.5` for a half cycle.
- `start_index`, `end_index`: positions of its extrema in the original input, not when the cycle became observable.

The first and last samples are endpoints. Interior plateaus use their last index. Remaining excursions are counted as half cycles; constant input returns an empty vector. Equal ranges are not merged.

The API allocates its output and runs in O(n) time and space. No streaming API, binning or fatigue-damage calculation is provided.

The function returns `MathResult<T> = Result<T, MathError>`.

## Performance

The repository includes Criterion benchmarks over seven input shapes at lengths 100, 1,000 and 10,000 (see `benches/rainflow.rs`). Timings include cycle extraction, allocation and dropping the output.

Example results (from `docs/rainflow_bench_results.md`, Intel Xeon W-2295, Ubuntu 22.04.5, rustc 1.90.0):

| Input       |      n | Time (low / estimate / high)       | ns/sample |
| ----------- | -----: | --------------------------------: | --------: |
| monotone    |    100 | 316.52 ns / 326.90 ns / 345.11 ns   |      3.27 |
| alternating |    100 | 1.6278 µs / 1.6341 µs / 1.6396 µs   |     16.34 |
| monotone    |  1,000 | 2.2993 µs / 2.4606 µs / 2.7096 µs   |      2.46 |
| alternating |  1,000 | 14.969 µs / 15.410 µs / 16.355 µs   |     15.41 |
| monotone    | 10,000 | 21.212 µs / 21.307 µs / 21.446 µs   |      2.13 |
| alternating | 10,000 | 147.55 µs / 148.32 µs / 149.53 µs   |     14.83 |

`ns/sample` is the central Criterion time estimate divided by `n`, not a latency percentile. Results depend on CPU, OS, Rust version and compiler settings.

Full details (machine, date, allocation probes and tail observations): `docs/rainflow_bench_results.md`.

## Usage

The crate is currently available from GitHub. Add to `Cargo.toml`:

```toml
[dependencies]
rainflow = { git = "https://github.com/tia-lab/rainflow.git" }
```

### Count cycles

```rust
use rainflow::{rainflow_cycles, MathResult};

fn main() -> MathResult<()> {
    let values = [0.0, 3.0, 1.0, 4.0];
    let cycles = rainflow_cycles(&values)?;

    // The inner swing produces a full cycle: range 2, mean 2.
    assert_eq!((cycles[0].range, cycles[0].mean, cycles[0].count), (2.0, 2.0, 1.0));
    // The remaining rise produces a half cycle: range 4, mean 2.
    assert_eq!((cycles[1].range, cycles[1].mean, cycles[1].count), (4.0, 2.0, 0.5));
    Ok(())
}
```

## Error handling and input checks

The function validates:

- Input length must be between 2 and `RAINFLOW_MAX_INPUT_LEN` (10,000).
- All values must be finite (`NaN`/`±inf` rejected).
- Ranges and means must be representable as finite `f64` values.

Recoverable buffer-reservation failures return an error. No partial result is returned on failure.

Errors are reported via `MathError` (see `src/errors.rs`).

## Tests

Run unit tests:

```bash
cargo test
```

Test sources: `src/tests/test_rainflow.rs`.

## Benchmarks

Benchmarks use Criterion (`criterion = "0.8.2"`).

Run benchmarks:

```bash
cargo bench
```

Bench source: `benches/rainflow.rs`.

The append-only benchmark report is checked in at `docs/rainflow_bench_results.md`; raw output is in `benches/benches.log`.

## Inventory

The generated file index is `inventory.md`. Update file purposes in `docs/inventory.md`, then regenerate:

```bash
cargo make inventory
```
