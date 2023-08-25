use benchmarking::sorting::bubble_sort;
use criterion::{black_box, BenchmarkId, Criterion};
use rand::Rng;
use std::boxed::Box;

fn bubble_sort_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bubble Sort Benchmark");
    let sizes = [10, 100, 1_000, 10_000, 100_000];
    for array_size in sizes {
        let mut array = generate_random_array(array_size);
        group.bench_with_input(
            BenchmarkId::new("bubble_sort algorithm", array_size),
            &array_size,
            |b, &_size| b.iter(|| bubble_sort(black_box(&mut array))),
        );
    }
    group.finish();
}

fn generate_random_array(length: usize) -> Box<[i32]> {
    let mut rng = rand::thread_rng();
    let mut array: Box<[i32]> = vec![0; length].into_boxed_slice();
    for value in array.iter_mut() {
        *value = rng.gen();
    }
    array
}

fn main() {
    let mut criterion = Criterion::default();
    criterion = criterion.sample_size(10);
    bubble_sort_benchmark(&mut criterion);
}
