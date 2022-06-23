use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
mod fib;

use ita::chapters::example;

fn public_add_two_benchmark(c: &mut Criterion) {
    c.bench_function("example benchmark", |b| {
        b.iter(|| example::public_add_two(black_box(20)))
    });
}

fn head<T: Copy>(xs: &Vec<T>) -> Option<T> {
    xs.get(0).map(|x| *x)
}

fn size_complexity_example(c: &mut Criterion) {
    static MULT: usize = 2;
    let mut group = c.benchmark_group("size_complexity_example");
    for size in [MULT, 10 * MULT, 100 * MULT, 1000 * MULT] {
        let mut vec = Vec::with_capacity(size);
        for _ in 0..size {
            vec.push(black_box(1))
        }
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| head(&vec));
        });
    }
}

// The first argument defines a variable which can be used in `criterion_main`
criterion_group!(example, public_add_two_benchmark);
criterion_group!(fib, fib::fib_benchmark);
criterion_group!(size_complexity, size_complexity_example);

// Variadic macro to benchmark every group
// criterion_main!(example, fib, size_complexity);
criterion_main!(size_complexity);
