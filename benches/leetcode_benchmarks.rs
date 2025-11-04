use criterion::{black_box, criterion_group, criterion_main, Criterion};
use MyRustLeetCode::two_sum::two_sum::two_sum;

fn benchmark_two_sum(c: &mut Criterion) {
    let nums = vec![2, 7, 11, 15, 3, 6, 9, 12, 1, 8, 4, 10];
    let target = 9;

    c.bench_function("two_sum", |b| {
        b.iter(|| two_sum(black_box(nums.clone()), black_box(target)))
    });
}

criterion_group!(benches, benchmark_two_sum);
criterion_main!(benches);
