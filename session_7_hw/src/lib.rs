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
    //returns 0 on prime _number
    ////otherwise return witness
    pub fn miller_rabin(_number: u64, bases: &[u64]) -> u64 {
        if _number <= 4 {
            match _number {
                2 => return 0,
                _ => return _number,
            }
        }
        if bases.contains(&_number) {
            return 0;
        }
        //n -1 = 2^k * q where q is odd
        let k: u64 = (_number - 1).trailing_zeros() as u64;
        let q = (_number - 1) >> k;
        for base in bases {
            if !is_prime_base(_number, *base, k, q) {
                return *base;
            }
        }
        0
    }

    fn is_prime_base(_number: u64, base: u64, two_power: u64, odd_power: u64) -> bool {
        let mut x: u128 = modulo_power(base, odd_power, _number) as u128;
        let b_number: u128 = _number as u128;
        if x == 1 || x == (b_number - 1) {
            return true;
        }
        for _ in 1..two_power {
            x = (x * x) % b_number;
            if x == (b_number - 1) {
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

#[cfg(test)]
mod test {

    use crate::eea::extended_euclid;
    use crate::miller_rabin::miller_rabin;
    #[test]
    fn eea_test() {
        let executed = extended_euclid(153, 87);
        assert_eq!((3, 4, -7), executed);
    }

    #[test]
    fn miller_rabin_test() {
        let primes = vec![3, 5, 7, 11, 13];
        let mut product = 1;
        let mut prime_flag = 0;
        for p in &primes {
            product *= p;
            prime_flag |= miller_rabin(*p as u64, &primes);
        }
        assert_eq!(prime_flag, 0)
    }
}

fn main() {
    println!("Hello, world!");
}
