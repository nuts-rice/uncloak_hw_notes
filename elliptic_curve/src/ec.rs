/*
#[derive(Clone)]
struct EllipticCurve {
    order: BigUint,
    p: BigUint,
    a: BigUint,
    b: BigUint,
    g: BigUint,
}
*/

/*
#[derive(Copy, Clone)]
struct Point {
    x: BigUint,
    y: BigUint,
    curve: EllipticCurve,
}
*/

pub use crypto_bigint as bigint;
use num_bigint::BigUint;
use std::fmt::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct EllipticCurve {
    pub p: BigUint,
    pub modulus: BigUint,
    pub a2: usize,
    pub a4: usize,
    pub a6: usize,
    pub is_weirstrass: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub point: Option<FinitePoint>,
    pub curve: EllipticCurve,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FinitePoint {
    pub x: BigUint,
    pub y: BigUint,
}

/*
impl EllipticCurve for Sep256k1 {

    unimplemented!()

}
*/

impl EllipticCurve {
    fn new(p: BigUint, modulus: BigUint, coeffiecents: &[usize]) -> Self {
        EllipticCurve {
            p,
            modulus,
            a2: coeffiecents[0],
            a4: coeffiecents[1],
            a6: coeffiecents[2],
            is_weirstrass: coeffiecents[0] == 0,
        }
    }

    //discriminant is invariant quality of roots that holds true regardless of transformations
    fn discriminant(&self) -> i128 {
        let b2 = 4 * self.a2;
        let (b4, b6, b8) = (2 * self.a4, 4 * self.a6, b2 * self.a6 - self.a4.pow(2));
        let delta_summand = (9 * b2 * b4 * b6) as i128;
        let delta_negand = (b8 * b2.pow(2) + 8 * b4.pow(3) + 27 * b6.pow(2)) as i128;
        delta_summand - delta_negand
    }

    fn is_smooth(&self) -> bool {
        self.discriminant() != 0
    }

    fn contains(&self, _point: &Point) -> bool {
        unimplemented!()
    }

    fn contains_inner(&self, _point: &FinitePoint) -> bool {
        unimplemented!()
    }

    fn eq(&self) {
        unimplemented!()
    }
    fn identity(&self) {
        unimplemented!()
    }
}

impl FinitePoint {
    fn reduce_modulo(self, modulus: &BigUint) -> Self {
        Self {
            x: self.x % modulus,
            y: self.y % modulus,
        }
    }
}

impl Point {
    fn to_bytes(&self) {
        unimplemented!()
    }

    fn is_identity(&self) {
        unimplemented!()
    }

    fn eq(&self, _point: Box<[u8]>) {
        unimplemented!()
    }

    fn add(&self) {
        unimplemented!()
    }

    fn mul(&self) {
        unimplemented!()
    }

    fn invert(&self) {
        unimplemented!()
    }

    fn double(&self) {
        unimplemented!()
    }
}

/*
impl fmt::Debug for EllipticCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_bytes())
    }
}
*/
