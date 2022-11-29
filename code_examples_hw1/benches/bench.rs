use code_examples_hw1::RSA;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{thread, time};

fn encrypt_bench(c: &mut Criterion) {
    let rsa = RSA::new();
    c.bench_function("encrypt", |f| {
        f.iter(|| rsa.encrypt(b"test test", rand::thread_rng()))
    });
}

criterion_group!(benches, encrypt_bench);
criterion_main!(benches);
