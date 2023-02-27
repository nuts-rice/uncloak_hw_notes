use core::fmt;
use num_bigint::*;
#[derive(Clone)]
struct EllipticCurve {
    order: BigUint,
    p: BigUint,
    a: BigUint,
    b: BigUint,
    g: BigUint,
}

#[derive(Copy, Clone)]
struct Point {}
impl EllipticCurve {
    fn new() -> Self {
        unimplemented!()
    }

    fn is_on_curve() {
        unimplemented!()
    }

    fn to_bytes(&self) {
        unimplemented!()
    }
}

impl fmt::Debug for EllipticCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_bytes())
    }
}
