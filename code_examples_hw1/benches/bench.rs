use code_examples_hw1::RSA;
use criterion::{criterion_group, criterion_main, Criterion};

fn encrypt_bench(c: &mut Criterion) {
    let rsa = RSA::new();
    c.bench_function("encrypt", |f| {
        f.iter(|| rsa.encrypt(b"test test", rand::thread_rng()))
    });
}
fn decrypt_bench(c: &mut Criterion) {
    let rsa = RSA::new();
    let ciphertxt: Vec<u8> = rsa.encrypt(b"test test", rand::thread_rng());
    c.bench_function("decrypt", |f| f.iter(|| rsa.decrypt(&ciphertxt)));
}

criterion_group!(benches, encrypt_bench, decrypt_bench);
criterion_main!(benches);
