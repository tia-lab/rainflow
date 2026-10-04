# `rainflow` — benchmark results (append-only)

Append-only log. Do not rewrite historical entries.

---

## 2026-10-04T07:10:18Z — baseline benches (standalone crate port)

Environment:

- OS: `Linux 5.15.0-156-generic x86_64`, Ubuntu 22.04.5 LTS
- CPU: `Intel(R) Xeon(R) W-2295 CPU @ 3.00GHz` (36 threads)
- Toolchain: `rustc 1.90.0 (1159e78c4 2025-09-14)`, `cargo 1.90.0`
- Criterion: 0.8.2 (`Cargo.toml` dev-dependency), sample size 30

Command: `cargo bench` (bench suite `benches/rainflow.rs`, Criterion target `signal_rainflow_end_to_end`).

Raw stdout: `benches/benches.log`

### Key results

Times are Criterion estimates `[low median high]` over seven input classes and `n ∈ {100, 1000, 10000}`:

| Input class  |      n |                          Time (low/med/high) | Med ns/sample |
| ------------ | -----: | -------------------------------------------: | ------------: |
| constant     |    100 |                 46.105 ns / 46.298 ns / 46.458 ns |          0.46 |
| monotone     |    100 |                316.52 ns / 326.90 ns / 345.11 ns |          3.27 |
| alternating  |    100 |                 1.6278 µs / 1.6341 µs / 1.6396 µs |         16.34 |
| nested       |    100 |                 1.2359 µs / 1.2390 µs / 1.2419 µs |         12.39 |
| plateaus     |    100 |                373.34 ns / 374.53 ns / 375.56 ns |          3.75 |
| residual     |    100 |                 1.6909 µs / 1.6971 µs / 1.7036 µs |         16.97 |
| cascade      |    100 |                 1.3336 µs / 1.3368 µs / 1.3395 µs |         13.37 |
| constant     |  1,000 |                377.79 ns / 379.41 ns / 381.05 ns |          0.38 |
| monotone     |  1,000 |                 2.2993 µs / 2.4606 µs / 2.7096 µs |          2.46 |
| alternating  |  1,000 |                14.969 µs / 15.410 µs / 16.355 µs |         15.41 |
| nested       |  1,000 |                11.093 µs / 11.178 µs / 11.312 µs |         11.18 |
| plateaus     |  1,000 |                 2.7925 µs / 2.8019 µs / 2.8114 µs |          2.80 |
| residual     |  1,000 |                16.277 µs / 16.320 µs / 16.367 µs |         16.32 |
| cascade      |  1,000 |                12.784 µs / 13.133 µs / 13.770 µs |         13.13 |
| constant     | 10,000 |                 3.7734 µs / 3.7881 µs / 3.8045 µs |          0.38 |
| monotone     | 10,000 |                21.212 µs / 21.307 µs / 21.446 µs |          2.13 |
| alternating  | 10,000 |                147.55 µs / 148.32 µs / 149.53 µs |         14.83 |
| nested       | 10,000 |                108.79 µs / 109.06 µs / 109.30 µs |         10.91 |
| plateaus     | 10,000 |                26.999 µs / 27.274 µs / 27.811 µs |          2.73 |
| residual     | 10,000 |                161.18 µs / 161.36 µs / 161.52 µs |         16.14 |
| cascade      | 10,000 |                126.64 µs / 127.73 µs / 129.12 µs |         12.77 |

`Med ns/sample` is `(median time) / n`, a rough per-observation cost. It depends on CPU, OS, Rust version, and compiler settings.

### Allocation probes

Asserted in the bench binary (all passed):

- Constant inputs: 0 allocations.
- All nonconstant inputs: exactly 2 allocations (`n * size_of::<usize>() + (n - 1) * size_of::<RainflowCycle>()` requested bytes).

### Tail observations (n = 10,000, 1000 timed calls)

| Input class  | p50_ns | p95_ns  | p99_ns  | max_ns |
| ------------ | -----: | ------: | ------: | -----: |
| constant     |  4,166 |   4,209 |   4,226 | 16,925 |
| monotone     | 21,014 |  25,632 |  37,148 | 52,245 |
| alternating  | 144,360 | 164,008 | 181,630 | 210,579 |
| nested       | 106,717 | 256,208 | 342,432 | 373,357 |
| plateaus     | 26,423 |  39,300 |  46,922 | 62,363 |
| residual     | 160,312 | 320,822 | 513,711 | 547,119 |
| cascade      | 123,526 | 138,161 | 147,707 | 170,305 |

The `nested` and `residual` classes show the widest tail spread; p99 remains within ~3.2x of p50 for all classes.

### Comparison with `math::signal` RF-1

This crate is an unchanged port of `math::signal::rainflow` (RF-1); the bench suite, input classes, and probe assertions are the same as `math/benches/signal_rainflow.rs`. Absolute numbers on this machine are recorded above; see the source module's `signal_bench_results.md` for its own environment.
