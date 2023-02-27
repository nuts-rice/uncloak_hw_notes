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

pub trait EllipticCurve: 'static + Clone + Eq + Ord + Send + Sync {
    fn is_on_curve() {
        unimplemented!()
    }

    fn to_bytes(&self) {
        unimplemented!()
    }

    //fn has_point(&self, point: Point) {
    //    unimplemented!()
    //

    fn eq(&self) {
        unimplemented!()
    }
}

pub trait Point: 'static + Copy + Clone + Eq + Ord + Send + Sync {
    fn to_bytes(&self) {
        unimplemented!()
    }

    fn eq(&self) {
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
}

/*
impl fmt::Debug for EllipticCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_bytes())
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.to_bytes())
    }
}
*/
