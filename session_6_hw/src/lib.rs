use generic_array::{ArrayLength, GenericArray};

struct GenArrayWrapper<T, N: ArrayLength<T>> {
    inner: GenericArray<T, N>,
}

impl<T, N> GenArrayWrapper<T, N>
where
    T: Default,
    N: ArrayLength<T>,
{
    fn new() -> Self {
        let mut array = GenericArray::<T, N>::default();
        let mut this = Self { inner: array };
        this
    }

    fn len(&self) -> &usize {
        &self.len()
    }

    fn get_index(val: &T) -> u8 {
        unimplemented!()
    }
}

fn main() {
    println!("Hello, world!");
}
