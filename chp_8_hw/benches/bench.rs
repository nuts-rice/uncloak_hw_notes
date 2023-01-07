use chp_8_hw::secret::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn left_channel(c: &mut Criterion) {
    //if we don't allow the compiler to optimize evaluating SECRET by using black_box(), then we can see how
    //computationally hard it is to evaluate for SECRET and possibly how long it could be
    c.bench_function("left channel", |b| {
        b.iter(|| branch_secret(black_box(SECRET)))
    });
}

pub fn right_channel(c: &mut Criterion) {
    c.bench_function("right channel", |b| b.iter(|| branch_secret(black_box(0))));
}

criterion_group!(benches, left_channel, right_channel);
criterion_main!(benches);
