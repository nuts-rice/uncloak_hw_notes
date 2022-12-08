use chp_5_6_hw::sha512;
use criterion::{criterion_group, criterion_main, Criterion};
use criterion_plot::curve;

fn sha_n_bench(c: &mut Criterion) {
    let message: Vec<u8> = ("test test").to_vec();
    let sha512 = sha512::sha512_n(&message, 8);
    c.bench_function("compute birthday attack", |f| {
        f.iter(|| sha512::compute_birthday_attack(8));
    });
}
