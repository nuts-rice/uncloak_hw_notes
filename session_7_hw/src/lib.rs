mod eea {
    fn advance_euclid(a: &mut i32, old_a: &mut i32, quoatiant: i32) {
        let temp = *a;
        *a = *old_a - quoatiant * temp;
        *old_a = temp;
    }

    pub fn extended_euclid(a: i32, b: i32) -> (i32, i32, i32) {
        let (mut old_r, mut rem) = (a, b);
        let (mut old_s, mut coeff_s) = (1, 0);
        let (mut old_t, mut coeff_t) = (0, 1);
        while rem != 0 {
            let quoatiant = old_r / rem;
            advance_euclid(&mut rem, &mut old_r, quoatiant);
            advance_euclid(&mut coeff_s, &mut old_s, quoatiant);
            advance_euclid(&mut coeff_t, &mut old_t, quoatiant);
        }
        (old_r, old_s, old_t)
    }
}

mod miller_rabin {
    pub fn miller_rabin(_number: u64, _babses: &[u64]) -> u64 {
        unimplemented!()
    }

    fn is_prime_base(number: u64, base: u64, two_power: u64, odd_power: u64) -> bool {
        let mut x: u128 = modulo_power(base, odd_power, number) as u128;
        let bnumber: u128 = number as u128;
        if x == 1 || x == (bnumber - 1) {
            return true;
        }
        for _ in 1..two_power {
            x = (x * x) % bnumber;
            if x == (bnumber - 1) {
                return true;
            }
        }
        false
    }

    fn modulo_power(mut base: u64, mut power: u64, modulo: u64) -> u64 {
        base %= modulo;
        if base == 0 {
            return 0;
        }
        let mut res: u128 = 1;
        let mut bbase: u128 = base as u128;
        while power > 0 {
            //power is
            if (power % 2) == 1 {
                res = (res * bbase) % (modulo as u128);
            }
            bbase = (bbase * bbase) % (modulo as u128);
            power /= 2;
        }
        res as u64
    }
}

fn main() {
    println!("Hello, world!");
}
