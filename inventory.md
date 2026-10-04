
# `rainflow` — Global Inventory (GENERATED; DO NOT EDIT)

Generated: 2026-10-04T14:44:27Z
Protocol: `.dev/protocols/global_inventory_generation_protocol.md`

This file is generated from the crate inventory at `docs/inventory.md`.
If a file purpose is missing in a module inventory, this file will mark it as `INVENTORY GAP`.

## Modules

- `rainflow`: `docs/inventory.md`

---

## `rainflow`

### Artifacts

- Inventory: `docs/inventory.md`
- Bench log: `docs/rainflow_bench_results.md`
- Tests: `src/tests`
- Benches: `benches/rainflow.rs`

### Source Files

- `bin/generate_global_inventory.rs`: Deterministic generator that rewrites `inventory.md` from this inventory; run via `cargo make inventory`.
- `src/config.rs`: Batch safety bound `RAINFLOW_MAX_INPUT_LEN` (10,000 samples).
- `src/core.rs`: Rainflow cycle extraction: `RainflowCycle` type, input validation, private counting helpers, and the public `rainflow_cycles` batch function.
- `src/errors.rs`: `MathError` enum for deterministic failure reporting at the API boundary.
- `src/lib.rs`: Crate entrypoint; declares modules and re-exports `rainflow_cycles`, `RainflowCycle`, `RAINFLOW_MAX_INPUT_LEN`, `MathError`, `MathResult`.
- `src/types.rs`: `MathResult<T>` result alias.

---
