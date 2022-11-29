use rand::{rngs::ThreadRng, CryptoRng, Rng, RngCore};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};

const BITS: usize = 2048;

#[derive(Debug, Clone)]
pub struct RSA {
    pub priv_key: RsaPrivateKey,
    pub pub_key: RsaPublicKey,
}

impl RSA {
    pub fn new() -> Self {
        let priv_key =
            RsaPrivateKey::new(&mut rand::thread_rng(), BITS).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);
        Self { priv_key, pub_key }
    }

    pub fn encrypt<R: CryptoRng + RngCore>(&self, data: &[u8], mut rng: R) -> Vec<u8> {
        self.pub_key
            .encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), data)
            .expect("failed to encrypt")
        // assert_ne!(&data[..], &enc_data[..]);
    }

    pub fn decrypt(&self, enc_data: &[u8]) -> Vec<u8> {
        self.priv_key
            .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), enc_data)
            .expect("failed to decrypt")
    }
}

pub mod vignere {
    #[derive(Clone, Debug)]
    pub struct Vignere<'a> {
        key: &'a [u8],
    }
}
