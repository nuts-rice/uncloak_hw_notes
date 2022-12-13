use chp_5_6_hw::sha512::{self, compute_birthday_attack};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use criterion_plot::curve;

fn sha_n_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("birthday group");
    group.sample_size(30);
    for length in [8_u16, 16, 24, 32, 48] {
        group.bench_with_input(BenchmarkId::from_parameter(length), &length, |b, length| {
            b.iter(|| compute_birthday_attack((*length as u16).try_into().unwrap()))
        });
    }
    group.finish();
}

criterion_group!(benches, sha_n_bench);
criterion_main!(benches);
