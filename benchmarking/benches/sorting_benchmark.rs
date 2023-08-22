use benchmarking::sorting::bubble_sort;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bubble_sort_benchmark(c: &mut Criterion) {
    let mut arr = black_box([6, 2, 4, 1, 9, -2, 5]);

    c.bench_function("sorting algorithm", |b| b.iter(|| bubble_sort(&mut arr)));
}

criterion_group!(sorting_benchmark, bubble_sort_benchmark);
criterion_main!(sorting_benchmark);
