use crypto_common::generic_array::GenericArray;
use crypto_common::BlockSizeUser;
use des::cipher::KeyInit;
use des::{cipher::BlockEncrypt, Des};
use rand::prelude::*;

type DesBlock = GenericArray<u8, <Des as BlockSizeUser>::BlockSize>;

fn is_des_complement(key: [u8; 8], plaintxt: [u8; 8]) -> bool {
    let cipher = Des::new(&key.into());
    let mut block: DesBlock = plaintxt.into();
    cipher.encrypt_block(&mut block);
    let _complement_of_encrypted: [u8; 8] = des_complement(block.as_ref());

    true
}

fn des_complement(bytes: &[u8; 8]) -> [u8; 8] {
    bytes.map(|b| !b)
}
#[test]
fn complement_relation_test() {}
