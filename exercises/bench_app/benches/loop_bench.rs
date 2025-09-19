use criterion::{criterion_group, criterion_main, Criterion};

fn bench_for_in_vec(c: &mut Criterion) {
    let vec: Vec<i32> = (0..1_000_000).collect();

    c.bench_function("for_in_vec", |b| {
        b.iter(|| {
            let mut sum = 0;
            for x in &vec {
                sum += x;
            }
            sum
        })
    });
}

fn bench_for_index(c: &mut Criterion) {
    let vec: Vec<i32> = (0..1_000_000).collect();

    c.bench_function("for_index", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 0..vec.len() {
                sum += vec[i];
            }
            sum
        })
    });
}

criterion_group!(benches, bench_for_in_vec, bench_for_index);
criterion_main!(benches);