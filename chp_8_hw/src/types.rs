use zeroize::Zeroize;

use std::marker;

#[derive(Debug)]
struct Key<State> {
    value: *const u32,
    len: usize,
    cap: usize,
    _state: marker::PhantomData<State>,
}
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
