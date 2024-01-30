use collatz::Number;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn sequence(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequence");
    group.throughput(criterion::Throughput::Elements(1));

    for n in [10u32, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, n| {
            b.iter(|| collatz::sequence(Number::new(*n).unwrap()).collect::<Vec<_>>())
        });
    }

    group.finish();
}

criterion_group!(benches, sequence);
criterion_main!(benches);
