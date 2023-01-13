use generic_array::{ArrayLength, GenericArray};

pub mod rng;

struct GenArrayWrapper<T, N: ArrayLength<T>> {
    inner: GenericArray<T, N>,
}

impl<T, N> GenArrayWrapper<T, N>
where
    T: Default + PartialEq,
    N: ArrayLength<T>,
{
    fn new() -> Self {
        let array = GenericArray::<T, N>::default();

        Self { inner: array }
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get_index(&self, val: &T) {
        let _i = 0;
        let index = self.inner.iter().position(|t| t == val).unwrap();
        println!("{}", index);
    }
}

fn main() {
    println!("Hello, world!");
}
