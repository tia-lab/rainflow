use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use rainflow::{RainflowCycle, rainflow_cycles};
use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};
use std::time::{Duration, Instant};

struct ProbeAllocator;
static PROBE: AtomicBool = AtomicBool::new(false);
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);

// Benchmark-only instrumentation; pointers/layouts are forwarded unchanged to System.
unsafe impl GlobalAlloc for ProbeAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if PROBE.load(Relaxed) {
            ALLOCATIONS.fetch_add(1, Relaxed);
            BYTES.fetch_add(layout.size(), Relaxed);
        }
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if PROBE.load(Relaxed) {
            ALLOCATIONS.fetch_add(1, Relaxed);
            BYTES.fetch_add(size, Relaxed);
        }
        unsafe { System.realloc(ptr, layout, size) }
    }
}

#[global_allocator]
static ALLOCATOR: ProbeAllocator = ProbeAllocator;

fn inputs(n: usize) -> Vec<(&'static str, Vec<f64>)> {
    let mut residual = vec![0.0; n];
    for i in 1..n {
        let step = (n - i) as f64;
        residual[i] = residual[i - 1] + if i % 2 == 1 { step } else { -step };
    }
    let mut cascade = residual.clone();
    cascade[n - 1] = 2.0 * n as f64;
    vec![
        ("constant", vec![1.0; n]),
        ("monotone", (0..n).map(|i| i as f64).collect()),
        ("alternating", (0..n).map(|i| (i % 2) as f64).collect()),
        (
            "nested",
            (0..n).map(|i| [0.0, 3.0, 1.0, 4.0, -1.0][i % 5]).collect(),
        ),
        ("plateaus", (0..n).map(|i| ((i / 8) % 2) as f64).collect()),
        ("residual", residual),
        ("cascade", cascade),
    ]
}

fn probe(name: &str, values: &[f64]) {
    ALLOCATIONS.store(0, Relaxed);
    BYTES.store(0, Relaxed);
    PROBE.store(true, Relaxed);
    let result = rainflow_cycles(black_box(values));
    PROBE.store(false, Relaxed);
    let result = result.unwrap();
    let calls = ALLOCATIONS.load(Relaxed);
    let bytes = BYTES.load(Relaxed);
    let expected_calls = if name == "constant" { 0 } else { 2 };
    let expected_bytes = if name == "constant" {
        0
    } else {
        values.len() * size_of::<usize>() + (values.len() - 1) * size_of::<RainflowCycle>()
    };
    assert_eq!(calls, expected_calls);
    assert_eq!(bytes, expected_bytes);
    println!(
        "ALLOC {name} n={} calls={calls} requested_bytes={bytes} cycles={}",
        values.len(),
        result.len()
    );
    drop(result);
    if values.len() == 10_000 {
        let mut elapsed = Vec::with_capacity(1_000);
        for _ in 0..1_000 {
            let start = Instant::now();
            drop(black_box(rainflow_cycles(black_box(values)).unwrap()));
            elapsed.push(start.elapsed().as_nanos());
        }
        elapsed.sort_unstable();
        println!(
            "TAIL {name} n=10000 samples=1000 p50_ns={} p95_ns={} p99_ns={} max_ns={}",
            elapsed[499], elapsed[949], elapsed[989], elapsed[999]
        );
    }
}

fn bench_rainflow(c: &mut Criterion) {
    let mut group = c.benchmark_group("signal_rainflow_end_to_end");
    group.sample_size(30);
    group.warm_up_time(Duration::from_millis(200));
    group.measurement_time(Duration::from_millis(500));
    for n in [100, 1_000, 10_000] {
        group.throughput(Throughput::Elements(n as u64));
        for (name, values) in inputs(n) {
            probe(name, &values);
            group.bench_with_input(BenchmarkId::new(name, n), &values, |b, input| {
                b.iter(|| drop(black_box(rainflow_cycles(black_box(input)).unwrap())))
            });
        }
    }
    group.finish();
}

criterion_group!(benches, bench_rainflow);
criterion_main!(benches);
