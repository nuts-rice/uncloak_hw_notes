use crate::error;

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
//struct Offer<Left, Right>(PhantomData<(Left, Right)>);
//struct Choose<Left, Right>(PhantomData<(Left, Right)>);
struct Label<S>(PhantomData<S>);
struct Goto<N>(PhantomData<N>);
struct Z;
struct S<N>(PhantomData<N>); //check peano encoding here: https://en.wikipedia.org/wiki/Peano_axioms
struct Close;

type Id = String;

#[tylift]
pub enum ChannelChoice {
    Left,
    Right,
}

pub struct Channel<S>
where
    S: ChannelChoice,
{
    n_left: u64,
    n_right: u64,
    //parameterize state by a type that is not used in types defn
    phantom: PhantomData<S>,
}

pub type LeftChannel = Channel<Left>;
pub type RightChannel = Channel<Right>;

impl<S: ChannelChoice> Channel<S> {
    pub fn new_channel_left() -> LeftChannel {
        Channel {
            n_left: 0,
            n_right: 0,
            phantom: PhantomData,
        }
    }
    pub fn new_channel_right() -> RightChannel {
        Channel {
            n_left: 0,
            n_right: 0,
            phantom: PhantomData,
        }
    }

    pub fn get_left(&self) -> u64 {
        let left = self.n_left;
        let left_ptr: *mut u64 = Box::into_raw(Box::new(left));
        println!("Pointer at creation : {:p}", left_ptr);
        left
    }

    pub fn get_right(&self) -> u64 {
        let right = self.n_right;
        let right_ptr: *mut u64 = Box::into_raw(Box::new(right));
        println!("Pointer at creation : {:p}", right_ptr);
        right
    }
}

impl Channel<Left> {
    pub fn increment_left(&mut self) {
        self.n_left += 1;
    }

    pub fn increment_right(&self) -> error::ChannelError {
        error::ChannelError::ChannelOpError {
            expected: String::from("Right"),
            found: String::from("Left"),
        }
    }

    pub fn switch_right(&mut self) -> Channel<Right> {
        Channel {
            n_left: self.n_left,
            n_right: self.n_right,
            phantom: PhantomData,
        }
    }

    pub fn reset_left(&mut self) -> Channel<Left> {
        self.n_left = 0;
        Channel {
            n_left: self.n_left,
            n_right: self.n_right,
            phantom: PhantomData,
        }
    }
}

impl Channel<Right> {
    pub fn increment_right(&mut self) {
        self.n_right += 1;
    }

    pub fn increment_left(&self) -> error::ChannelError {
        error::ChannelError::ChannelOpError {
            expected: String::from("Left"),
            found: String::from("Right"),
        }
    }

    pub fn switch_left(&mut self) -> Channel<Left> {
        Channel {
            n_left: self.n_left,
            n_right: self.n_right,
            phantom: PhantomData,
        }
    }

    pub fn reset_right(&mut self) -> Channel<Right> {
        self.n_right = 0;
        Channel {
            n_left: self.n_left,
            n_right: self.n_right,
            phantom: PhantomData,
        }
    }
}

//pub struct Chan<Env, Protocol>(Sender<*mut u8>, Receiver<*mut u8>, PhantomData<(Env, Protocol)>);

//struct Ping;
//type PingServer = Label<Offer<Send<Ping, Recv<Ping, Goto<Z>>>, Close>>;

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
