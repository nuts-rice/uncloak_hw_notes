use std::marker::PhantomData;

struct Send<T, S>(PhantomData<(T, S)>);
struct Recv<T, S>(PhantomData<(T, S)>);
struct Offer<Left, Right>(PhantomData<(Left, Right)>);
struct Choose<Left, Right>(PhantomData<(Left, Right)>);
struct Label<S>(PhantomData<S>);
struct Goto<N>(PhantomData<N>);

/*
impl<State> Default for Key<State> {
    fn default() -> Key<State> {
        value : 0,
        cap : 0,
        _state : false,
    }
}


impl<State> Drop for Key<State> {
    fn drop(&mut self) {
        println!("Pointer when zeroing {:p}", self.value.as_ptr());
        self.value.zeroize();
        println!("zeroed, remaining value: {:?}", self.value);
    }
}
*/
