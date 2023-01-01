use std::marker::PhantomData;

use tylift::tylift;

//SessionType::==
//recieve message type *t*
//send message type *t*
//choose sub protocol{L} | {R}
//offer sub protocol{L} | {R}
//Label point
//go to label
//End protocol

struct Send<T, S>(PhantomData<(T, S)>);
struct Recv<T, S>(PhantomData<(T, S)>);
struct Offer<Left, Right>(PhantomData<(Left, Right)>);
struct Choose<Left, Right>(PhantomData<(Left, Right)>);
struct Label<S>(PhantomData<S>);
struct Goto<N>(PhantomData<N>);
struct Z;
struct S<N>(PhantomData<N>); //check peano encoding here: https://en.wikipedia.org/wiki/Peano_axioms
struct Close;

#[tylift]
pub enum Choose {
    Left,
    Right,
}

#[tylift]
pub enum Offer {
    Left,
    Right,
}

//pub struct Chan<Env, Protocol>(Sender<*mut u8>, Receiver<*mut u8>, PhantomData<(Env, Protocol)>);

struct Ping;
type PingServer = Label<Offer<Send<Ping, Recv<Ping, Goto<Z>>>, Close>>;

fn example_ping_server() {
    //    let (c, _) : (Chan<(), PingServer>,
    //    Chan<(), Dual<PingServer>) = Chan::new();
    unimplemented!()
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
