use xxhash_rust::xxh64::xxh64;

fn bench_comparison(c: &mut Criterion) {
    let attrs = [0.1f64.to_bits(), 0.2f64.to_bits(), /* ... */];

    let mut group = c.benchmark_group("TigerΔ vs Simple Hash");

    group.bench_function("TigerΔ Nonlinear Folding", |b| {
        b.iter(|| tiger_delta_core_logic(black_box(&mock_attrs_f64)))
    });

    group.bench_function("XXHash64 (baseline)", |b| {
        b.iter(|| xxh64(black_box(&attrs), 42))
    });

    group.finish();
}
