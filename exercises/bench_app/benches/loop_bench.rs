use criterion::{Criterion, criterion_group, criterion_main};
use std::ops::Deref;

fn bench_for_in_vec(c: &mut Criterion) {
    let vec: Vec<i32> = (0..100_000_000).collect();

    c.bench_function("for_in_vec", |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for x in &vec {
                sum += *x as u64;
            }
            sum
        })
    });
}

fn bench_for_index(c: &mut Criterion) {
    let vec: Vec<i32> = (0..100_000_000).collect();

    c.bench_function("for_index", |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for i in 0..vec.len() {
                sum += vec[i] as u64;
            }
            sum
        })
    });
}

criterion_group!(benches, bench_for_in_vec, bench_for_index);
criterion_main!(benches);
