const BITS: usize = 16;

pub mod sha512 {

    use sha2::{Digest, Sha512};
    use std::collections::HashMap;

    pub fn sha512_n(plaintext: &[u8], num: u8) -> Vec<u8> {
        let mut hasher = Sha512::new();
        let n_bytes = num / 8;
        hasher.update(plaintext);
        let res: Vec<u8> = hasher.finalize().to_vec();

        res[0..(n_bytes as usize)].to_vec()
    }

    pub fn compute_birthday_attack(n: u8) {
        let num_iterations = 2u32.pow(n as u32 / 2);
        let mut i = 0;
        let mut map: HashMap<Vec<u8>, String> = HashMap::new();
        while i < num_iterations {
            let message = format!("Collision canidate {0}", rand::random::<u32>());
            let res = sha512_n(message.as_bytes(), n);
            if map.contains_key(&res) {
                println!(
                    "collison between {} and {}",
                    map.get(&res).unwrap(),
                    message
                );
                break;
            } else {
                map.insert(res, message);
            }
            i += 1;
        }
        panic!("no collison found")
    }

    fn get_preimage(n: u8, image: &[u8]) -> (u32, String) {
        let max_iterations = 2u32.pow(n as u32);
        let mut i = 0;
        while i < max_iterations {
            let message = format!("test message {0}", rand::random::<u32>());
            let res = sha512_n(message.as_bytes(), n);
            if result == image {
                return (i + 1, mesage);
            }
            i += 2;
        }
        (max_iterations, String::new())
    }
}

pub mod tests {
    use super::*;
}
