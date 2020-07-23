use std::fmt::Debug;

use rand::Rng;
use rand_chacha::ChaCha20Rng;

use num::basic::integers::PrimitiveInteger;

/// Generates random primitive integers uniformly.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ThriftyRandomState {
    x: u32,
    bits_left: u64,
}

pub trait ThriftyRandom {
    type State: Clone + Debug;

    fn new_state() -> Self::State;

    fn get_random(rng: &mut ChaCha20Rng, state: &mut Self::State) -> Self;
}

macro_rules! impl_trivial_thrifty_random {
    ($t: ident) => {
        impl ThriftyRandom for $t {
            type State = ();

            #[inline]
            fn new_state() -> () {
                ()
            }

            #[inline]
            fn get_random(rng: &mut ChaCha20Rng, _state: &mut ()) -> $t {
                rng.gen()
            }
        }
    };
}
impl_trivial_thrifty_random!(u32);
impl_trivial_thrifty_random!(u64);
impl_trivial_thrifty_random!(u128);
impl_trivial_thrifty_random!(usize);
impl_trivial_thrifty_random!(i32);
impl_trivial_thrifty_random!(i64);
impl_trivial_thrifty_random!(i128);
impl_trivial_thrifty_random!(isize);

fn _get_random<T: PrimitiveInteger>(rng: &mut ChaCha20Rng, state: &mut ThriftyRandomState) -> T {
    if state.bits_left == 0 {
        state.x = rng.gen();
        state.bits_left = 32 - T::WIDTH;
    } else {
        state.x >>= T::WIDTH;
        state.bits_left -= T::WIDTH;
    }
    T::wrapping_from(state.x)
}

macro_rules! impl_thrifty_thrifty_random {
    ($t: ident) => {
        impl ThriftyRandom for $t {
            type State = ThriftyRandomState;

            #[inline]
            fn new_state() -> ThriftyRandomState {
                ThriftyRandomState { x: 0, bits_left: 0 }
            }

            #[inline]
            fn get_random(rng: &mut ChaCha20Rng, state: &mut ThriftyRandomState) -> $t {
                _get_random(rng, state)
            }
        }
    };
}
impl_thrifty_thrifty_random!(u8);
impl_thrifty_thrifty_random!(u16);
impl_thrifty_thrifty_random!(i8);
impl_thrifty_thrifty_random!(i16);

#[derive(Clone, Debug)]
pub struct RandomPrimitiveIntegers<T: ThriftyRandom> {
    pub(crate) rng: ChaCha20Rng,
    pub(crate) state: T::State,
}

impl<T: ThriftyRandom> Iterator for RandomPrimitiveIntegers<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        Some(T::get_random(&mut self.rng, &mut self.state))
    }
}
