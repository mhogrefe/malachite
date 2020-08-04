use num::basic::integers::PrimitiveInteger;
use rand::Rng;
use rand_chacha::ChaCha20Rng;
use std::fmt::Debug;

/// Uniformly generates random primitive integers.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ThriftyRandomState {
    x: u32,
    bits_left: u64,
}

pub trait HasRandomPrimitiveIntegers {
    type State: Clone + Debug;

    fn new_state() -> Self::State;

    fn get_random(rng: &mut ChaCha20Rng, state: &mut Self::State) -> Self;
}

macro_rules! impl_trivial_random_primitive_integers {
    ($t: ident) => {
        impl HasRandomPrimitiveIntegers for $t {
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
impl_trivial_random_primitive_integers!(u32);
impl_trivial_random_primitive_integers!(u64);
impl_trivial_random_primitive_integers!(u128);
impl_trivial_random_primitive_integers!(usize);
impl_trivial_random_primitive_integers!(i32);
impl_trivial_random_primitive_integers!(i64);
impl_trivial_random_primitive_integers!(i128);
impl_trivial_random_primitive_integers!(isize);

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

macro_rules! impl_thrifty_random_primitive_integers {
    ($t: ident) => {
        impl HasRandomPrimitiveIntegers for $t {
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
impl_thrifty_random_primitive_integers!(u8);
impl_thrifty_random_primitive_integers!(u16);
impl_thrifty_random_primitive_integers!(i8);
impl_thrifty_random_primitive_integers!(i16);

/// Uniformly generates random primitive integers.
///
/// This `struct` is created by the `random_primitive_integers` method. See its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct RandomPrimitiveIntegers<T: HasRandomPrimitiveIntegers> {
    pub(crate) rng: ChaCha20Rng,
    pub(crate) state: T::State,
}

impl<T: HasRandomPrimitiveIntegers> Iterator for RandomPrimitiveIntegers<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        Some(T::get_random(&mut self.rng, &mut self.state))
    }
}
