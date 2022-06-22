use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod fib;

use ita::chapters::example;

fn public_add_two_benchmark(c: &mut Criterion) {
    c.bench_function("example benchmark", |b| {
        b.iter(|| example::public_add_two(black_box(20)))
    });
}

// The first argument defines a variable which can be used in `criterion_main`
criterion_group!(example, public_add_two_benchmark);
criterion_group!(fib, fib::fib_benchmark);

// Variadic macro to benchmark every group
criterion_main!(example, fib);
