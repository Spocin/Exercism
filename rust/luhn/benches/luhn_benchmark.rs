use luhn::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Bencher};

fn luhn_benchmark(c: &mut Criterion) {
    let mut input = black_box("8273 1232 7352 0569");

    c.bench_function(
        "luhn algorithm",
        |b: &mut Bencher| b.iter(|| is_valid(input))
    );
}

criterion_group!(benches, luhn_benchmark);
criterion_main!(benches);