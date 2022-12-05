const BITS: usize = 16;

mod sha512 {
    use generic_array::GenericArray;
    use sha2::{Digest, Sha512};

    pub fn sha512_n(plaintext: &[u8]) -> Vec<u8> {
        let mut hasher = Sha512::new();
        let mut n_bits = Sha512::digest(&plaintext);
        unimplemented!()
    }
}
