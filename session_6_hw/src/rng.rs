use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

fn rng() {
    let mut sneed: <ChaCha20Rng as SeedableRng>::Seed = Default::default();
    thread_rng().fill(&mut sneed);
    let mut rng = ChaCha20Rng::from_seed(sneed);
    println!("{}", rng.gen_range(0..100));
}
