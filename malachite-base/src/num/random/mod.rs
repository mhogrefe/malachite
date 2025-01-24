// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::bools::random::{random_bools, RandomBools};
use crate::iterators::{
    nonzero_values, with_special_value, with_special_values, NonzeroValues, WithSpecialValue,
    WithSpecialValues,
};
use crate::num::arithmetic::traits::{Parity, PowerOf2, ShrRound};
use crate::num::basic::floats::PrimitiveFloat;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::num::float::NiceFloat;
use crate::num::iterators::{iterator_to_bit_chunks, IteratorToBitChunks};
use crate::num::logic::traits::{BitAccess, SignificantBits};
use crate::num::random::geometric::{
    geometric_random_signed_inclusive_range, geometric_random_unsigned_inclusive_range,
    geometric_random_unsigneds, GeometricRandomNaturalValues, GeometricRandomSignedRange,
};
use crate::random::{Seed, EXAMPLE_SEED};
use crate::rounding_modes::RoundingMode::*;
use crate::vecs::{random_values_from_vec, RandomValuesFromVec};
use itertools::Itertools;
use rand::Rng;
use rand_chacha::ChaCha20Rng;
use std::collections::HashMap;
use std::convert::identity;
use std::fmt::Debug;
use std::marker::PhantomData;

// Uniformly generates random primitive integers.
#[doc(hidden)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ThriftyRandomState {
    x: u32,
    bits_left: u64,
}

#[doc(hidden)]
pub trait HasRandomPrimitiveInts {
    type State: Clone + Debug;

    fn new_state() -> Self::State;

    fn get_random(rng: &mut ChaCha20Rng, state: &mut Self::State) -> Self;
}

macro_rules! impl_trivial_random_primitive_ints {
    ($t: ident) => {
        impl HasRandomPrimitiveInts for $t {
            type State = ();

            #[inline]
            fn new_state() -> () {}

            #[inline]
            fn get_random(rng: &mut ChaCha20Rng, _state: &mut ()) -> $t {
                rng.gen()
            }
        }
    };
}
impl_trivial_random_primitive_ints!(u32);
impl_trivial_random_primitive_ints!(u64);
impl_trivial_random_primitive_ints!(u128);
impl_trivial_random_primitive_ints!(usize);
impl_trivial_random_primitive_ints!(i32);
impl_trivial_random_primitive_ints!(i64);
impl_trivial_random_primitive_ints!(i128);
impl_trivial_random_primitive_ints!(isize);

fn get_random<T: PrimitiveInt>(rng: &mut ChaCha20Rng, state: &mut ThriftyRandomState) -> T {
    if state.bits_left == 0 {
        state.x = rng.gen();
        state.bits_left = u32::WIDTH - T::WIDTH;
    } else {
        state.x >>= T::WIDTH;
        state.bits_left -= T::WIDTH;
    }
    T::wrapping_from(state.x)
}

macro_rules! impl_thrifty_random_primitive_ints {
    ($t: ident) => {
        impl HasRandomPrimitiveInts for $t {
            type State = ThriftyRandomState;

            #[inline]
            fn new_state() -> ThriftyRandomState {
                ThriftyRandomState { x: 0, bits_left: 0 }
            }

            #[inline]
            fn get_random(rng: &mut ChaCha20Rng, state: &mut ThriftyRandomState) -> $t {
                get_random(rng, state)
            }
        }
    };
}
impl_thrifty_random_primitive_ints!(u8);
impl_thrifty_random_primitive_ints!(u16);
impl_thrifty_random_primitive_ints!(i8);
impl_thrifty_random_primitive_ints!(i16);

/// Uniformly generates random primitive integers.
///
/// This `struct` is created by [`random_primitive_ints`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomPrimitiveInts<T: HasRandomPrimitiveInts> {
    pub(crate) rng: ChaCha20Rng,
    pub(crate) state: T::State,
}

impl<T: HasRandomPrimitiveInts> Iterator for RandomPrimitiveInts<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        Some(T::get_random(&mut self.rng, &mut self.state))
    }
}

/// Uniformly generates random unsigned integers less than a positive limit.
///
/// This `enum` is created by [`random_unsigneds_less_than`]; see its documentation for more.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum RandomUnsignedsLessThan<T: PrimitiveUnsigned> {
    One,
    AtLeastTwo(RandomUnsignedBitChunks<T>, T),
}

impl<T: PrimitiveUnsigned> Iterator for RandomUnsignedsLessThan<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        match *self {
            RandomUnsignedsLessThan::One => Some(T::ZERO),
            RandomUnsignedsLessThan::AtLeastTwo(ref mut xs, limit) => loop {
                let x = xs.next();
                if x.unwrap() < limit {
                    return x;
                }
            },
        }
    }
}

/// Uniformly generates random unsigned integers in the half-open interval $[a, b)$.
///
/// This `struct` is created by [`random_unsigned_range`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomUnsignedRange<T: PrimitiveUnsigned> {
    pub(crate) xs: RandomUnsignedsLessThan<T>,
    pub(crate) a: T,
}

impl<T: PrimitiveUnsigned> Iterator for RandomUnsignedRange<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.xs.next().map(|x| x + self.a)
    }
}

/// Uniformly generates random unsigned integers in the closed interval $[a, b]$.
///
/// This `struct` is created by [`random_unsigned_inclusive_range`]; see its documentation for more.
#[derive(Clone, Debug)]
pub enum RandomUnsignedInclusiveRange<T: PrimitiveUnsigned> {
    NotAll(RandomUnsignedsLessThan<T>, T),
    All(RandomPrimitiveInts<T>),
}

impl<T: PrimitiveUnsigned> Iterator for RandomUnsignedInclusiveRange<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        match self {
            RandomUnsignedInclusiveRange::NotAll(xs, a) => xs.next().map(|x| x + *a),
            RandomUnsignedInclusiveRange::All(xs) => xs.next(),
        }
    }
}

#[doc(hidden)]
pub trait HasRandomSignedRange: Sized {
    type UnsignedValue: PrimitiveUnsigned;

    fn new_unsigned_range(seed: Seed, a: Self, b: Self)
        -> RandomUnsignedRange<Self::UnsignedValue>;

    fn new_unsigned_inclusive_range(
        seed: Seed,
        a: Self,
        b: Self,
    ) -> RandomUnsignedInclusiveRange<Self::UnsignedValue>;

    fn from_unsigned_value(x: Self::UnsignedValue) -> Self;
}

macro_rules! impl_has_random_signed_range {
    ($u: ident, $s: ident) => {
        impl HasRandomSignedRange for $s {
            type UnsignedValue = $u;

            fn new_unsigned_range(seed: Seed, mut a: $s, mut b: $s) -> RandomUnsignedRange<$u> {
                a.flip_bit($u::WIDTH - 1);
                b.flip_bit($u::WIDTH - 1);
                random_unsigned_range(seed, $u::wrapping_from(a), $u::wrapping_from(b))
            }

            fn new_unsigned_inclusive_range(
                seed: Seed,
                mut a: $s,
                mut b: $s,
            ) -> RandomUnsignedInclusiveRange<$u> {
                a.flip_bit($u::WIDTH - 1);
                b.flip_bit($u::WIDTH - 1);
                random_unsigned_inclusive_range(seed, $u::wrapping_from(a), $u::wrapping_from(b))
            }

            fn from_unsigned_value(mut u: $u) -> $s {
                u.flip_bit($u::WIDTH - 1);
                $s::wrapping_from(u)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_has_random_signed_range);

/// Uniformly generates random signed integers in the half-open interval $[a, b)$.
///
/// This `struct` is created by [`random_signed_range`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomSignedRange<T: HasRandomSignedRange> {
    pub(crate) xs: RandomUnsignedRange<T::UnsignedValue>,
}

impl<T: HasRandomSignedRange> Iterator for RandomSignedRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.xs.next().map(T::from_unsigned_value)
    }
}

/// Uniformly generates random signed integers in the closed interval $[a, b]$.
///
/// This `struct` is created by [`random_signed_inclusive_range`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomSignedInclusiveRange<T: HasRandomSignedRange> {
    pub(crate) xs: RandomUnsignedInclusiveRange<T::UnsignedValue>,
}

impl<T: HasRandomSignedRange> Iterator for RandomSignedInclusiveRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.xs.next().map(T::from_unsigned_value)
    }
}

/// Uniformly generates unsigned integers with up to some number of bits.
///
/// This `struct` is created by [`random_unsigned_bit_chunks`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomUnsignedBitChunks<T: PrimitiveUnsigned> {
    xs: IteratorToBitChunks<RandomPrimitiveInts<T>, T, T>,
}

impl<T: PrimitiveUnsigned> Iterator for RandomUnsignedBitChunks<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.xs.next_with_wrapping(identity).map(Option::unwrap)
    }
}

#[doc(hidden)]
pub trait RandomSignedChunkable: Sized {
    type AbsoluteChunks: Clone + Debug;

    fn new_absolute_chunks(seed: Seed, chunk_size: u64) -> Self::AbsoluteChunks;

    fn next_chunk(xs: &mut Self::AbsoluteChunks) -> Option<Self>;
}

macro_rules! impl_random_signed_chunkable {
    ($u: ident, $s: ident) => {
        impl RandomSignedChunkable for $s {
            type AbsoluteChunks = RandomUnsignedBitChunks<$u>;

            fn new_absolute_chunks(seed: Seed, chunk_size: u64) -> RandomUnsignedBitChunks<$u> {
                random_unsigned_bit_chunks(seed, chunk_size)
            }

            fn next_chunk(xs: &mut Self::AbsoluteChunks) -> Option<$s> {
                xs.next().map(WrappingFrom::wrapping_from)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_random_signed_chunkable);

/// Uniformly generates signed integers with up to some number of bits.
///
/// This `struct` is created by [`random_signed_bit_chunks`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomSignedBitChunks<T: RandomSignedChunkable> {
    pub(crate) xs: T::AbsoluteChunks,
}

impl<T: RandomSignedChunkable> Iterator for RandomSignedBitChunks<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        T::next_chunk(&mut self.xs)
    }
}

/// Modifies the output values of an iterator by setting their highest bit.
#[derive(Clone, Debug)]
pub struct RandomHighestBitSetValues<I: Iterator>
where
    I::Item: PrimitiveInt,
{
    pub(crate) xs: I,
    pub(crate) mask: I::Item,
}

impl<I: Iterator> Iterator for RandomHighestBitSetValues<I>
where
    I::Item: PrimitiveInt,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        self.xs.next().map(|x| x | self.mask)
    }
}

/// Uniformly generates random primitive integers.
///
/// $P(x) = 2^{-W}$, where $W$ is the width of the type.
///
/// The output length is infinite.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(random_primitive_ints::<u8>(EXAMPLE_SEED), 10),
///     "[113, 239, 69, 108, 228, 210, 168, 161, 87, 32, ...]"
/// )
/// ```
#[inline]
pub fn random_primitive_ints<T: PrimitiveInt>(seed: Seed) -> RandomPrimitiveInts<T> {
    RandomPrimitiveInts {
        rng: seed.get_rng(),
        state: T::new_state(),
    }
}

/// Uniformly generates random positive unsigned integers.
///
/// $$
/// P(x) = \\begin{cases}
///     \\frac{1}{2^W-1} & \text{if} \\quad x > 0, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $W$ is the width of the type.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_positive_unsigneds;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(random_positive_unsigneds::<u8>(EXAMPLE_SEED), 10),
///     "[113, 239, 69, 108, 228, 210, 168, 161, 87, 32, ...]"
/// )
/// ```
#[inline]
pub fn random_positive_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
) -> NonzeroValues<RandomPrimitiveInts<T>> {
    nonzero_values(random_primitive_ints(seed))
}

/// Uniformly generates random positive signed integers.
///
/// $$
/// P(x) = \\begin{cases}
///     \\frac{1}{2^{W-1}-1} & \text{if} \\quad x > 0, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $W$ is the width of the type.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_positive_signeds;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(random_positive_signeds::<i8>(EXAMPLE_SEED), 10),
///     "[113, 94, 23, 98, 70, 92, 52, 84, 33, 47, ...]"
/// )
/// ```
#[inline]
pub fn random_positive_signeds<T: PrimitiveSigned>(
    seed: Seed,
) -> NonzeroValues<RandomSignedBitChunks<T>> {
    nonzero_values(random_natural_signeds(seed))
}

/// Uniformly generates random negative signed integers.
///
/// $$
/// P(x) = \\begin{cases}
///     2^{1-W} & \text{if} \\quad x < 0, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $W$ is the width of the type.
///
/// The output length is infinite.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_negative_signeds;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(random_negative_signeds::<i8>(EXAMPLE_SEED), 10),
///     "[-15, -34, -105, -30, -58, -36, -76, -44, -95, -81, ...]"
/// )
/// ```
#[inline]
pub fn random_negative_signeds<T: PrimitiveSigned>(
    seed: Seed,
) -> RandomHighestBitSetValues<RandomSignedBitChunks<T>> {
    RandomHighestBitSetValues {
        xs: random_signed_bit_chunks(seed, T::WIDTH - 1),
        mask: T::MIN,
    }
}

/// Uniformly generates random natural (non-negative) signed integers.
///
/// $$
/// P(x) = \\begin{cases}
///     2^{1-W} & \text{if} \\quad x \geq 0, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $W$ is the width of the type.
///
/// The output length is infinite.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_natural_signeds;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(random_natural_signeds::<i8>(EXAMPLE_SEED), 10),
///     "[113, 94, 23, 98, 70, 92, 52, 84, 33, 47, ...]"
/// )
/// ```
#[inline]
pub fn random_natural_signeds<T: PrimitiveSigned>(seed: Seed) -> RandomSignedBitChunks<T> {
    random_signed_bit_chunks(seed, T::WIDTH - 1)
}

/// Uniformly generates random nonzero signed integers.
///
/// $$
/// P(x) = \\begin{cases}
///     \\frac{1}{2^W-1} & \text{if} \\quad x \\neq 0, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $W$ is the width of the type.
///
/// The output length is infinite.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_nonzero_signeds;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(random_nonzero_signeds::<i8>(EXAMPLE_SEED), 10),
///     "[113, -17, 69, 108, -28, -46, -88, -95, 87, 32, ...]"
/// )
/// ```
#[inline]
pub fn random_nonzero_signeds<T: PrimitiveSigned>(
    seed: Seed,
) -> NonzeroValues<RandomPrimitiveInts<T>> {
    nonzero_values(random_primitive_ints(seed))
}

/// Uniformly generates random unsigned integers less than a positive limit.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{\\ell} & \text{if} \\quad x < \\ell, \\\\
///     0 & \\text{otherwise,}
/// \\end{cases}
/// $$
/// where $\ell$ is `limit`.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `limit` is 0.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_unsigneds_less_than;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(random_unsigneds_less_than::<u8>(EXAMPLE_SEED, 10), 10),
///     "[1, 7, 5, 4, 6, 4, 2, 8, 1, 7, ...]"
/// )
/// ```
pub fn random_unsigneds_less_than<T: PrimitiveUnsigned>(
    seed: Seed,
    limit: T,
) -> RandomUnsignedsLessThan<T> {
    if limit == T::ZERO {
        panic!("limit cannot be 0.");
    } else if limit == T::ONE {
        RandomUnsignedsLessThan::One
    } else {
        RandomUnsignedsLessThan::AtLeastTwo(
            random_unsigned_bit_chunks(seed, limit.ceiling_log_base_2()),
            limit,
        )
    }
}

/// Uniformly generates random unsigned integers in the half-open interval $[a, b)$.
///
/// $a$ must be less than $b$. This function cannot create a range that includes `T::MAX`; for that,
/// use [`random_unsigned_inclusive_range`].
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{b-a} & \text{if} \\quad a \leq x < b, \\\\
///     0 & \\text{otherwise.}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_unsigned_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(random_unsigned_range::<u8>(EXAMPLE_SEED, 10, 20), 10),
///     "[11, 17, 15, 14, 16, 14, 12, 18, 11, 17, ...]"
/// )
/// ```
pub fn random_unsigned_range<T: PrimitiveUnsigned>(
    seed: Seed,
    a: T,
    b: T,
) -> RandomUnsignedRange<T> {
    assert!(a < b, "a must be less than b. a: {a}, b: {b}");
    RandomUnsignedRange {
        xs: random_unsigneds_less_than(seed, b - a),
        a,
    }
}

/// Uniformly generates random unsigned integers in the closed interval $[a, b]$.
///
/// $a$ must be less than or equal to $b$.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{b-a+1} & \text{if} \\quad a \leq x \leq b, \\\\
///     0 & \\text{otherwise.}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         random_unsigned_inclusive_range::<u8>(EXAMPLE_SEED, 10, 19),
///         10
///     ),
///     "[11, 17, 15, 14, 16, 14, 12, 18, 11, 17, ...]"
/// )
/// ```
pub fn random_unsigned_inclusive_range<T: PrimitiveUnsigned>(
    seed: Seed,
    a: T,
    b: T,
) -> RandomUnsignedInclusiveRange<T> {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    if a == T::ZERO && b == T::MAX {
        RandomUnsignedInclusiveRange::All(random_primitive_ints(seed))
    } else {
        RandomUnsignedInclusiveRange::NotAll(random_unsigneds_less_than(seed, b - a + T::ONE), a)
    }
}

/// Uniformly generates random signed integers in the half-open interval $[a, b)$.
///
/// $a$ must be less than $b$. This function cannot create a range that includes `T::MAX`; for that,
/// use [`random_signed_inclusive_range`].
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{b-a} & \text{if} \\quad a \leq x < b, \\\\
///     0 & \\text{otherwise.}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_signed_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(random_signed_range::<i8>(EXAMPLE_SEED, -100, 100), 10),
///     "[13, -31, 8, 68, 61, -13, -68, 10, -17, 88, ...]"
/// )
/// ```
#[inline]
pub fn random_signed_range<T: PrimitiveSigned>(seed: Seed, a: T, b: T) -> RandomSignedRange<T> {
    assert!(a < b, "a must be less than b. a: {a}, b: {b}");
    RandomSignedRange {
        xs: T::new_unsigned_range(seed, a, b),
    }
}

/// Uniformly generates random signed integers in the closed interval $[a, b]$.
///
/// $a$ must be less than or equal to $b$.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{b-a+1} & \text{if} \\quad a \leq x \leq b, \\\\
///     0 & \\text{otherwise.}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_signed_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         random_signed_inclusive_range::<i8>(EXAMPLE_SEED, -100, 99),
///         10
///     ),
///     "[13, -31, 8, 68, 61, -13, -68, 10, -17, 88, ...]"
/// )
/// ```
#[inline]
pub fn random_signed_inclusive_range<T: PrimitiveSigned>(
    seed: Seed,
    a: T,
    b: T,
) -> RandomSignedInclusiveRange<T> {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    RandomSignedInclusiveRange {
        xs: T::new_unsigned_inclusive_range(seed, a, b),
    }
}

/// Uniformly generates unsigned integers containing some maximum number of bits.
///
/// $$
/// P(x) = \\begin{cases}
///     2^{-c} & \text{if} \\quad 0 \\leq x < 2^c, \\\\
///     0 & \\text{otherwise,}
/// \\end{cases}
/// $$
/// where $c$ is `chunk_size`.
///
/// The output length is infinite.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `chunk_size` is zero or greater than the width of the type.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_unsigned_bit_chunks;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(random_unsigned_bit_chunks::<u8>(EXAMPLE_SEED, 3), 10),
///     "[1, 6, 5, 7, 6, 3, 1, 2, 4, 5, ...]"
/// )
/// ```
pub fn random_unsigned_bit_chunks<T: PrimitiveUnsigned>(
    seed: Seed,
    chunk_size: u64,
) -> RandomUnsignedBitChunks<T> {
    RandomUnsignedBitChunks {
        xs: iterator_to_bit_chunks(random_primitive_ints(seed), T::WIDTH, chunk_size),
    }
}

/// Uniformly generates signed integers containing some maximum number of bits.
///
/// The generated values will all be non-negative unless `chunk_size` is equal to the width of the
/// type.
///
/// $$
/// P(x) = \\begin{cases}
///     2^{-c} & \text{if} \\quad c = W \\ \\text{or}
///         \\ (c < W \\ \\text{and} \\ 0 \\leq x < 2^c), \\\\
///     0 & \\text{otherwise,}
/// \\end{cases}
/// $$
/// where $c$ is `chunk_size` and $W$ is the width of the type.
///
/// The output length is infinite.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `chunk_size` is zero or greater than the width of the type.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_signed_bit_chunks;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(random_signed_bit_chunks::<i8>(EXAMPLE_SEED, 3), 10),
///     "[1, 6, 5, 7, 6, 3, 1, 2, 4, 5, ...]"
/// )
/// ```
pub fn random_signed_bit_chunks<T: PrimitiveSigned>(
    seed: Seed,
    chunk_size: u64,
) -> RandomSignedBitChunks<T> {
    assert!(chunk_size <= T::WIDTH);
    RandomSignedBitChunks {
        xs: T::new_absolute_chunks(seed, chunk_size),
    }
}

/// Uniformly generates unsigned integers whose highest bit is set.
///
/// $$
/// P(x) = \\begin{cases}
///     2^{1-W} & \text{if} \\quad 2^{W-1} \\leq x < 2^W ,\\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $W$ is the width of the type.
///
/// The output length is infinite.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::random_highest_bit_set_unsigneds;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(random_highest_bit_set_unsigneds::<u8>(EXAMPLE_SEED), 10),
///     "[241, 222, 151, 226, 198, 220, 180, 212, 161, 175, ...]"
/// )
/// ```
#[inline]
pub fn random_highest_bit_set_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
) -> RandomHighestBitSetValues<RandomUnsignedBitChunks<T>> {
    RandomHighestBitSetValues {
        xs: random_unsigned_bit_chunks(seed, T::WIDTH - 1),
        mask: T::power_of_2(T::WIDTH - 1),
    }
}

/// Generates random primitive floats in the half-open interval $[a, b)$.
///
/// This `struct` is created by [`random_primitive_float_range`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomPrimitiveFloatRange<T: PrimitiveFloat> {
    phantom: PhantomData<*const T>,
    xs: RandomUnsignedRange<u64>,
}

impl<T: PrimitiveFloat> Iterator for RandomPrimitiveFloatRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.xs.next().map(T::from_ordered_representation)
    }
}

/// Generates random primitive floats in the closed interval $[a, b]$.
///
/// This `struct` is created by [`random_primitive_float_inclusive_range`]; see its documentation
/// for more.
#[derive(Clone, Debug)]
pub struct RandomPrimitiveFloatInclusiveRange<T: PrimitiveFloat> {
    phantom: PhantomData<*const T>,
    xs: RandomUnsignedInclusiveRange<u64>,
}

impl<T: PrimitiveFloat> Iterator for RandomPrimitiveFloatInclusiveRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.xs.next().map(T::from_ordered_representation)
    }
}

/// Generates random primitive floats in the half-open interval $[a, b)$.
///
/// Every float within the range has an equal probability of being chosen. This does not mean that
/// the distribution approximates a uniform distribution over the reals. For example, if the range
/// is $[0, 2)$, a float in $[1/4, 1/2)$ is as likely to be chosen as a float in $[1, 2)$, since
/// these subranges contain an equal number of floats.
///
/// Positive and negative zero are treated as two distinct values, with negative zero being smaller
/// than zero.
///
/// `NaN` is never generated.
///
/// $a$ must be less than $b$. This function cannot create a range that includes `T::INFINITY`; for
/// that, use [`random_primitive_float_inclusive_range`].
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::random_primitive_float_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         random_primitive_float_range::<f32>(EXAMPLE_SEED, -0.1, 0.1).map(NiceFloat),
///         10
///     ),
///     "[5.664681e-11, 1.2492925e-35, 2.3242339e-29, 4.699183e-7, -2.8244436e-36, -2.264039e-37, \
///     -0.0000017299129, 1.40616e-23, 2.7418007e-27, 1.5418819e-16, ...]"
/// );
/// ```
#[inline]
pub fn random_primitive_float_range<T: PrimitiveFloat>(
    seed: Seed,
    a: T,
    b: T,
) -> RandomPrimitiveFloatRange<T> {
    assert!(!a.is_nan());
    assert!(!b.is_nan());
    assert!(
        NiceFloat(a) < NiceFloat(b),
        "a must be less than b. a: {}, b: {}",
        NiceFloat(a),
        NiceFloat(b)
    );
    RandomPrimitiveFloatRange {
        phantom: PhantomData,
        xs: random_unsigned_range(
            seed,
            a.to_ordered_representation(),
            b.to_ordered_representation(),
        ),
    }
}

/// Generates random primitive floats in the closed interval $[a, b]$.
///
/// Every float within the range has an equal probability of being chosen. This does not mean that
/// the distribution approximates a uniform distribution over the reals. For example, if the range
/// is $[0, 2]$, a float in $[1/4, 1/2)$ is as likely to be chosen as a float in $[1, 2)$, since
/// these subranges contain an equal number of floats.
///
/// Positive and negative zero are treated as two distinct values, with negative zero being smaller
/// than zero.
///
/// $a$ must be less than or equal to $b$.
///
/// `NaN` is never generated.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::random_primitive_float_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         random_primitive_float_inclusive_range::<f32>(EXAMPLE_SEED, -0.1, 0.1).map(NiceFloat),
///         10
///     ),
///     "[5.664681e-11, 1.2492925e-35, 2.3242339e-29, 4.699183e-7, -2.8244436e-36, -2.264039e-37, \
///     -0.0000017299129, 1.40616e-23, 2.7418007e-27, 1.5418819e-16, ...]"
/// );
/// ```
#[inline]
pub fn random_primitive_float_inclusive_range<T: PrimitiveFloat>(
    seed: Seed,
    a: T,
    b: T,
) -> RandomPrimitiveFloatInclusiveRange<T> {
    assert!(!a.is_nan());
    assert!(!b.is_nan());
    assert!(
        NiceFloat(a) <= NiceFloat(b),
        "a must be less than or equal to b. a: {}, b: {}",
        NiceFloat(a),
        NiceFloat(b)
    );
    RandomPrimitiveFloatInclusiveRange {
        phantom: PhantomData,
        xs: random_unsigned_inclusive_range(
            seed,
            a.to_ordered_representation(),
            b.to_ordered_representation(),
        ),
    }
}

/// Generates random finite positive primitive floats.
///
/// Every float within the range has an equal probability of being chosen. This does not mean that
/// the distribution approximates a uniform distribution over the reals. For example, a float in
/// $[1/4, 1/2)$ is as likely to be chosen as a float in $[1, 2)$, since these subranges contain an
/// equal number of floats.
///
/// Positive zero is generated; negative zero is not. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::random_positive_finite_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         random_positive_finite_primitive_floats::<f32>(EXAMPLE_SEED).map(NiceFloat),
///         10
///     ),
///     "[9.5715654e26, 209.6476, 386935780.0, 7.965817e30, 0.00021030706, 0.0027270128, \
///     3.4398167e-34, 2.3397111e14, 44567765000.0, 2.3479653e21, ...]"
/// );
/// ```
#[inline]
pub fn random_positive_finite_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
) -> RandomPrimitiveFloatInclusiveRange<T> {
    random_primitive_float_inclusive_range(seed, T::MIN_POSITIVE_SUBNORMAL, T::MAX_FINITE)
}

/// Generates random finite negative primitive floats.
///
/// Every float within the range has an equal probability of being chosen. This does not mean that
/// the distribution approximates a uniform distribution over the reals. For example, a float in
/// $(-1/2, 1/4]$ is as likely to be chosen as a float in $(-2, -1]$, since these subranges contain
/// an equal number of floats.
///
/// Negative zero is generated; positive zero is not. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::random_negative_finite_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         random_negative_finite_primitive_floats::<f32>(EXAMPLE_SEED).map(NiceFloat),
///         10
///     ),
///     "[-2.3484663e-27, -0.010641626, -5.8060583e-9, -2.8182442e-31, -10462.532, -821.12994, \
///     -6.303163e33, -9.50376e-15, -4.9561126e-11, -8.565163e-22, ...]"
/// );
/// ```
#[inline]
pub fn random_negative_finite_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
) -> RandomPrimitiveFloatInclusiveRange<T> {
    random_primitive_float_inclusive_range(seed, -T::MAX_FINITE, -T::MIN_POSITIVE_SUBNORMAL)
}

/// Generates random finite nonzero primitive floats.
///
/// Every float within the range has an equal probability of being chosen. This does not mean that
/// the distribution approximates a uniform distribution over the reals. For example, a float in
/// $[1/4, 1/2)$ is as likely to be chosen as a float in $[1, 2)$, since these subranges contain an
/// equal number of floats.
///
/// Neither positive nor negative zero are generated. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::random_nonzero_finite_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         random_nonzero_finite_primitive_floats::<f32>(EXAMPLE_SEED).map(NiceFloat),
///         10
///     ),
///     "[-2.3484663e-27, 2.287989e-18, -2.0729893e-12, 3.360012e28, -9.021723e-32, 3564911.2, \
///     -0.0000133769445, -1.8855448e18, 8.2494555e-29, 2.2178014e-38, ...]"
/// );
/// ```
#[inline]
pub fn random_nonzero_finite_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
) -> NonzeroValues<RandomPrimitiveFloatInclusiveRange<T>> {
    nonzero_values(random_finite_primitive_floats(seed))
}

/// Generates random finite primitive floats.
///
/// Every float within the range has an equal probability of being chosen. This does not mean that
/// the distribution approximates a uniform distribution over the reals. For example, a float in
/// $[1/4, 1/2)$ is as likely to be chosen as a float in $[1, 2)$, since these subranges contain an
/// equal number of floats.
///
/// Positive zero and negative zero are both generated. `NaN` is not.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::random_finite_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         random_finite_primitive_floats::<f32>(EXAMPLE_SEED).map(NiceFloat),
///         10
///     ),
///     "[-2.3484663e-27, 2.287989e-18, -2.0729893e-12, 3.360012e28, -9.021723e-32, 3564911.2, \
///     -0.0000133769445, -1.8855448e18, 8.2494555e-29, 2.2178014e-38, ...]"
/// );
/// ```
#[inline]
pub fn random_finite_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
) -> RandomPrimitiveFloatInclusiveRange<T> {
    random_primitive_float_inclusive_range(seed, -T::MAX_FINITE, T::MAX_FINITE)
}

/// Generates random positive primitive floats.
///
/// Every float within the range has an equal probability of being chosen. This does not mean that
/// the distribution approximates a uniform distribution over the reals. For example, a float in
/// $[1/4, 1/2)$ is as likely to be chosen as a float in $[1, 2)$, since these subranges contain an
/// equal number of floats.
///
/// Positive zero is generated; negative zero is not. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::random_positive_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         random_positive_primitive_floats::<f32>(EXAMPLE_SEED).map(NiceFloat),
///         10
///     ),
///     "[9.5715654e26, 209.6476, 386935780.0, 7.965817e30, 0.00021030706, 0.0027270128, \
///     3.4398167e-34, 2.3397111e14, 44567765000.0, 2.3479653e21, ...]"
/// );
/// ```
#[inline]
pub fn random_positive_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
) -> RandomPrimitiveFloatInclusiveRange<T> {
    random_primitive_float_inclusive_range(seed, T::MIN_POSITIVE_SUBNORMAL, T::INFINITY)
}

/// Generates random negative primitive floats.
///
/// Every float within the range has an equal probability of being chosen. This does not mean that
/// the distribution approximates a uniform distribution over the reals. For example, a float in
/// $(-1/2, -1/4]$ is as likely to be chosen as a float in $(-2, -1]$, since these subranges contain
/// an equal number of floats.
///
/// Negative zero is generated; positive zero is not. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::random_negative_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         random_negative_primitive_floats::<f32>(EXAMPLE_SEED).map(NiceFloat),
///         10
///     ),
///     "[-2.3484665e-27, -0.010641627, -5.8060587e-9, -2.8182444e-31, -10462.533, -821.13, \
///     -6.3031636e33, -9.5037605e-15, -4.956113e-11, -8.565164e-22, ...]"
/// );
/// ```
#[inline]
pub fn random_negative_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
) -> RandomPrimitiveFloatInclusiveRange<T> {
    random_primitive_float_inclusive_range(seed, T::NEGATIVE_INFINITY, -T::MIN_POSITIVE_SUBNORMAL)
}

/// Generates random nonzero primitive floats.
///
/// Every float within the range has an equal probability of being chosen. This does not mean that
/// the distribution approximates a uniform distribution over the reals. For example, a float in
/// $[1/4, 1/2)$ is as likely to be chosen as a float in $[1, 2)$, since these subranges contain an
/// equal number of floats.
///
/// Neither positive nor negative zero are generated. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::random_nonzero_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         random_nonzero_primitive_floats::<f32>(EXAMPLE_SEED).map(NiceFloat),
///         10
///     ),
///     "[-2.3484665e-27, 2.2879888e-18, -2.0729896e-12, 3.3600117e28, -9.0217234e-32, 3564911.0, \
///     -0.000013376945, -1.885545e18, 8.249455e-29, 2.2178013e-38, ...]",
/// );
/// ```
#[inline]
pub fn random_nonzero_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
) -> NonzeroValues<RandomPrimitiveFloats<T>> {
    nonzero_values(random_primitive_floats(seed))
}

/// Generates random primitive floats.
///
/// This `struct` is created by [`random_primitive_floats`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomPrimitiveFloats<T: PrimitiveFloat> {
    phantom: PhantomData<*const T>,
    pub(crate) xs: RandomUnsignedInclusiveRange<u64>,
    nan: u64,
}

impl<T: PrimitiveFloat> Iterator for RandomPrimitiveFloats<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.xs.next().map(|x| {
            if x == self.nan {
                T::NAN
            } else {
                T::from_ordered_representation(x)
            }
        })
    }
}

/// Generates random finite primitive floats.
///
/// Every float has an equal probability of being chosen. This does not mean that the distribution
/// approximates a uniform distribution over the reals. For example, a float in $[1/4, 1/2)$ is as
/// likely to be chosen as a float in $[1, 2)$, since these subranges contain an equal number of
/// floats.
///
/// Positive zero, negative zero, and `NaN` are all generated.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::random_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         random_primitive_floats::<f32>(EXAMPLE_SEED).map(NiceFloat),
///         10
///     ),
///     "[-2.3484665e-27, 2.2879888e-18, -2.0729896e-12, 3.3600117e28, -9.0217234e-32, 3564911.0, \
///     -0.000013376945, -1.885545e18, 8.249455e-29, 2.2178013e-38, ...]"
/// );
/// ```
#[inline]
pub fn random_primitive_floats<T: PrimitiveFloat>(seed: Seed) -> RandomPrimitiveFloats<T> {
    let nan = T::INFINITY.to_ordered_representation() + 1;
    RandomPrimitiveFloats {
        phantom: PhantomData,
        xs: random_unsigned_inclusive_range(seed, 0, nan),
        nan,
    }
}

/// Generates positive finite primitive floats.
///
/// This `struct` is created by [`special_random_positive_finite_primitive_floats`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct SpecialRandomPositiveFiniteFloats<T: PrimitiveFloat> {
    seed: Seed,
    sci_exponents: GeometricRandomSignedRange<i64>,
    range_map: HashMap<i64, GeometricRandomNaturalValues<u64>>,
    ranges: VariableRangeGenerator,
    mean_precision_n: u64,
    mean_precision_d: u64,
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveFloat> Iterator for SpecialRandomPositiveFiniteFloats<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let sci_exponent = self.sci_exponents.next().unwrap();
        let mean_precision_n = self.mean_precision_n;
        let mean_precision_d = self.mean_precision_d;
        let seed = self.seed;
        let precisions = self.range_map.entry(sci_exponent).or_insert_with(move || {
            geometric_random_unsigned_inclusive_range(
                seed.fork(&sci_exponent.to_string()),
                1,
                T::max_precision_for_sci_exponent(sci_exponent),
                mean_precision_n,
                mean_precision_d,
            )
        });
        let precision = precisions.next().unwrap();
        let mantissa = if precision == 1 {
            1
        } else {
            // e.g. if precision is 4, generate odd values from 1001 through 1111, inclusive
            let x = self.ranges.next_in_range(
                u64::power_of_2(precision - 2),
                u64::power_of_2(precision - 1),
            );
            (x << 1) | 1
        };
        T::from_integer_mantissa_and_exponent(
            mantissa,
            sci_exponent - i64::wrapping_from(precision) + 1,
        )
    }
}

/// Generates positive finite primitive floats.
///
/// Simpler floats (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the mean absolute sci-exponent and precision by passing the numerators
/// and denominators of their means.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// Positive zero is generated; negative zero is not. `NaN` is not generated either. TODO: don't
/// generate any zeros!
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::special_random_positive_finite_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         special_random_positive_finite_primitive_floats::<f32>(EXAMPLE_SEED, 10, 1, 10, 1)
///             .map(NiceFloat),
///         20
///     ),
///     "[0.80126953, 0.0000013709068, 0.015609741, 0.98552704, 65536.0, 0.008257866, \
///     0.017333984, 2.25, 7.7089844, 0.00004425831, 0.40625, 24576.0, 37249.0, 1.1991882, \
///     32.085938, 0.4375, 0.0012359619, 1536.0, 0.22912993, 0.0015716553, ...]"
/// );
/// ```
pub fn special_random_positive_finite_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
    mean_sci_exponent_numerator: u64,
    mean_sci_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
) -> SpecialRandomPositiveFiniteFloats<T> {
    assert_ne!(mean_precision_denominator, 0);
    assert!(mean_precision_numerator > mean_precision_denominator);
    SpecialRandomPositiveFiniteFloats {
        seed: seed.fork("precisions"),
        sci_exponents: geometric_random_signed_inclusive_range(
            EXAMPLE_SEED.fork("exponents"),
            T::MIN_EXPONENT,
            T::MAX_EXPONENT,
            mean_sci_exponent_numerator,
            mean_sci_exponent_denominator,
        ),
        range_map: HashMap::new(),
        ranges: VariableRangeGenerator::new(seed.fork("ranges")),
        mean_precision_n: mean_precision_numerator,
        mean_precision_d: mean_precision_denominator,
        phantom: PhantomData,
    }
}

/// Generates negative finite primitive floats.
///
/// This `struct` is created by [`special_random_negative_finite_primitive_floats`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct SpecialRandomNegativeFiniteFloats<T: PrimitiveFloat>(
    SpecialRandomPositiveFiniteFloats<T>,
);

impl<T: PrimitiveFloat> Iterator for SpecialRandomNegativeFiniteFloats<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.0.next().map(|f| -f)
    }
}

/// Generates negative finite primitive floats.
///
/// Simpler floats (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the mean absolute sci-exponent and precision by passing the numerators
/// and denominators of their means.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// Negative zero is generated; positive zero is not. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::special_random_negative_finite_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         special_random_negative_finite_primitive_floats::<f32>(EXAMPLE_SEED, 10, 1, 10, 1)
///             .map(NiceFloat),
///         20
///     ),
///     "[-0.80126953, -0.0000013709068, -0.015609741, -0.98552704, -65536.0, -0.008257866, \
///     -0.017333984, -2.25, -7.7089844, -0.00004425831, -0.40625, -24576.0, -37249.0, \
///     -1.1991882, -32.085938, -0.4375, -0.0012359619, -1536.0, -0.22912993, -0.0015716553, ...]"
/// );
/// ```
#[inline]
pub fn special_random_negative_finite_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
    mean_sci_exponent_numerator: u64,
    mean_sci_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
) -> SpecialRandomNegativeFiniteFloats<T> {
    SpecialRandomNegativeFiniteFloats(special_random_positive_finite_primitive_floats(
        seed,
        mean_sci_exponent_numerator,
        mean_sci_exponent_denominator,
        mean_precision_numerator,
        mean_precision_denominator,
    ))
}

/// Generates nonzero finite primitive floats.
///
/// This `struct` is created by [`special_random_nonzero_finite_primitive_floats`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct SpecialRandomNonzeroFiniteFloats<T: PrimitiveFloat> {
    bs: RandomBools,
    xs: SpecialRandomPositiveFiniteFloats<T>,
}

impl<T: PrimitiveFloat> Iterator for SpecialRandomNonzeroFiniteFloats<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let x = self.xs.next().unwrap();
        Some(if self.bs.next().unwrap() { x } else { -x })
    }
}

/// Generates finite nonzero primitive floats.
///
/// Simpler floats (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the mean absolute sci-exponent and precision by passing the numerators
/// and denominators of their means.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// Neither positive not negative zero is generated. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::special_random_nonzero_finite_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         special_random_nonzero_finite_primitive_floats::<f32>(EXAMPLE_SEED, 10, 1, 10, 1)
///             .map(NiceFloat),
///         20
///     ),
///     "[-0.6328125, -9.536743e-7, -0.013671875, 0.6875, -70208.0, 0.01550293, -0.028625488, \
///     -3.3095703, -5.775879, 0.000034958124, 0.4375, 31678.0, -49152.0, -1.0, 49.885254, \
///     -0.40625, -0.0015869141, -1889.5625, -0.14140439, -0.001449585, ...]"
/// );
/// ```
#[inline]
pub fn special_random_nonzero_finite_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
    mean_sci_exponent_numerator: u64,
    mean_sci_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
) -> SpecialRandomNonzeroFiniteFloats<T> {
    SpecialRandomNonzeroFiniteFloats {
        bs: random_bools(seed.fork("bs")),
        xs: special_random_positive_finite_primitive_floats(
            seed.fork("xs"),
            mean_sci_exponent_numerator,
            mean_sci_exponent_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
        ),
    }
}

/// Generates finite primitive floats.
///
/// Simpler floats (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the numerator and denominator of the probability that a zero will be
/// generated. You can also specify the mean absolute sci-exponent and precision by passing the
/// numerators and denominators of their means of the nonzero floats.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// Positive and negative zero are both generated. `NaN` is not.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::special_random_finite_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         special_random_finite_primitive_floats::<f32>(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///             .map(NiceFloat),
///         20
///     ),
///     "[0.65625, 0.0000014255784, 0.013183594, 0.0, -0.8125, -74240.0, -0.0078125, -0.03060913, \
///     3.331552, 4.75, -0.000038146973, -0.3125, -27136.0, -0.0, -59392.0, -1.75, -41.1875, 0.0, \
///     0.30940247, -0.0009765625, ...]"
/// );
/// ```
#[inline]
pub fn special_random_finite_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
    mean_sci_exponent_numerator: u64,
    mean_sci_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_zero_p_numerator: u64,
    mean_zero_p_denominator: u64,
) -> WithSpecialValues<SpecialRandomNonzeroFiniteFloats<T>> {
    with_special_values(
        seed,
        vec![T::ZERO, T::NEGATIVE_ZERO],
        mean_zero_p_numerator,
        mean_zero_p_denominator,
        &|seed_2| {
            special_random_nonzero_finite_primitive_floats(
                seed_2,
                mean_sci_exponent_numerator,
                mean_sci_exponent_denominator,
                mean_precision_numerator,
                mean_precision_denominator,
            )
        },
    )
}

/// Generates positive primitive floats.
///
/// Simpler floats (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the numerator and denominator of the probability that positive infinity
/// will be generated. You can also specify the mean absolute sci-exponent and precision by passing
/// the numerators and denominators of their means of the finite floats.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// Positive zero is generated; negative zero is not. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::special_random_positive_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         special_random_positive_primitive_floats::<f32>(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///             .map(NiceFloat),
///         20
///     ),
///     "[0.6328125, 9.536743e-7, 0.013671875, Infinity, 0.6875, 70208.0, 0.01550293, \
///     0.028625488, 3.3095703, 5.775879, 0.000034958124, 0.4375, 31678.0, Infinity, 49152.0, \
///     1.0, 49.885254, Infinity, 0.40625, 0.0015869141, ...]"
/// );
/// ```
#[inline]
pub fn special_random_positive_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
    mean_sci_exponent_numerator: u64,
    mean_sci_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
) -> WithSpecialValue<SpecialRandomPositiveFiniteFloats<T>> {
    with_special_value(
        seed,
        T::INFINITY,
        mean_special_p_numerator,
        mean_special_p_denominator,
        &|seed_2| {
            special_random_positive_finite_primitive_floats(
                seed_2,
                mean_sci_exponent_numerator,
                mean_sci_exponent_denominator,
                mean_precision_numerator,
                mean_precision_denominator,
            )
        },
    )
}

/// Generates negative primitive floats.
///
/// Simpler floats (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the numerator and denominator of the probability that negative infinity
/// will be generated. You can also specify the mean absolute sci-exponent and precision by passing
/// the numerators and denominators of their means of the finite floats.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// Negative zero is generated; positive zero is not. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::special_random_negative_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         special_random_negative_primitive_floats::<f32>(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///             .map(NiceFloat),
///         20
///     ),
///     "[-0.6328125, -9.536743e-7, -0.013671875, -Infinity, -0.6875, -70208.0, -0.01550293, \
///     -0.028625488, -3.3095703, -5.775879, -0.000034958124, -0.4375, -31678.0, -Infinity, \
///     -49152.0, -1.0, -49.885254, -Infinity, -0.40625, -0.0015869141, ...]"
/// );
/// ```
#[inline]
pub fn special_random_negative_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
    mean_sci_exponent_numerator: u64,
    mean_sci_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
) -> WithSpecialValue<SpecialRandomNegativeFiniteFloats<T>> {
    with_special_value(
        seed,
        T::NEGATIVE_INFINITY,
        mean_special_p_numerator,
        mean_special_p_denominator,
        &|seed_2| {
            special_random_negative_finite_primitive_floats(
                seed_2,
                mean_sci_exponent_numerator,
                mean_sci_exponent_denominator,
                mean_precision_numerator,
                mean_precision_denominator,
            )
        },
    )
}

/// Generates nonzero primitive floats.
///
/// Simpler floats (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the numerator and denominator of the probability that an infinity will
/// be generated. You can also specify the mean absolute sci-exponent and precision by passing the
/// numerators and denominators of their means of the finite floats.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// Neither negative not positive zero is generated. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::special_random_nonzero_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         special_random_nonzero_primitive_floats::<f32>(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///             .map(NiceFloat),
///         20
///     ),
///     "[0.65625, 0.0000014255784, 0.013183594, Infinity, -0.8125, -74240.0, -0.0078125, \
///     -0.03060913, 3.331552, 4.75, -0.000038146973, -0.3125, -27136.0, -Infinity, -59392.0, \
///     -1.75, -41.1875, Infinity, 0.30940247, -0.0009765625, ...]"
/// );
/// ```
#[inline]
pub fn special_random_nonzero_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
    mean_sci_exponent_numerator: u64,
    mean_sci_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
) -> WithSpecialValues<SpecialRandomNonzeroFiniteFloats<T>> {
    with_special_values(
        seed,
        vec![T::INFINITY, T::NEGATIVE_INFINITY],
        mean_special_p_numerator,
        mean_special_p_denominator,
        &|seed_2| {
            special_random_nonzero_finite_primitive_floats(
                seed_2,
                mean_sci_exponent_numerator,
                mean_sci_exponent_denominator,
                mean_precision_numerator,
                mean_precision_denominator,
            )
        },
    )
}

/// Generates primitive floats.
///
/// Simpler floats (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the numerator and denominator of the probability that zero, infinity, or
/// NaN will be generated. You can also specify the mean absolute sci-exponent and precision by
/// passing the numerators and denominators of their means of the finite floats.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::special_random_primitive_floats;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         special_random_primitive_floats::<f32>(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///             .map(NiceFloat),
///         20
///     ),
///     "[0.65625, 0.0000014255784, 0.013183594, Infinity, -0.8125, -74240.0, -0.0078125, \
///     -0.03060913, 3.331552, 4.75, -0.000038146973, -0.3125, -27136.0, Infinity, -59392.0, \
///     -1.75, -41.1875, Infinity, 0.30940247, -0.0009765625, ...]"
/// );
/// ```
#[inline]
pub fn special_random_primitive_floats<T: PrimitiveFloat>(
    seed: Seed,
    mean_sci_exponent_numerator: u64,
    mean_sci_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
) -> WithSpecialValues<SpecialRandomNonzeroFiniteFloats<T>> {
    with_special_values(
        seed,
        vec![T::ZERO, T::NEGATIVE_ZERO, T::INFINITY, T::NEGATIVE_INFINITY, T::NAN],
        mean_special_p_numerator,
        mean_special_p_denominator,
        &|seed_2| {
            special_random_nonzero_finite_primitive_floats(
                seed_2,
                mean_sci_exponent_numerator,
                mean_sci_exponent_denominator,
                mean_precision_numerator,
                mean_precision_denominator,
            )
        },
    )
}

// normalized sci_exponent and raw mantissas in input, adjusted sci_exponent and mantissas in output
fn mantissas_inclusive<T: PrimitiveFloat>(
    mut sci_exponent: i64,
    mut am: u64,
    mut bm: u64,
    precision: u64,
) -> Option<(i64, u64, u64)> {
    assert_ne!(precision, 0);
    let p: u64 = if sci_exponent < T::MIN_NORMAL_EXPONENT {
        let ab = am.significant_bits();
        let bb = bm.significant_bits();
        assert_eq!(ab, bb);
        ab - precision
    } else {
        am.set_bit(T::MANTISSA_WIDTH);
        bm.set_bit(T::MANTISSA_WIDTH);
        T::MANTISSA_WIDTH + 1 - precision
    };
    let mut lo = am.shr_round(p, Up).0;
    if lo.even() {
        lo += 1;
    }
    let mut hi = bm.shr_round(p, Down).0;
    if hi == 0 {
        return None;
    } else if hi.even() {
        hi -= 1;
    }
    if sci_exponent >= T::MIN_NORMAL_EXPONENT {
        sci_exponent -= i64::wrapping_from(T::MANTISSA_WIDTH);
    }
    sci_exponent += i64::wrapping_from(p);
    if lo > hi {
        None
    } else {
        Some((sci_exponent, lo >> 1, hi >> 1))
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct SpecialRandomPositiveFiniteFloatInclusiveRange<T: PrimitiveFloat> {
    phantom: PhantomData<*const T>,
    am: u64, // raw mantissa
    bm: u64,
    ae: i64, // sci_exponent
    be: i64,
    sci_exponents: GeometricRandomSignedRange<i64>,
    precision_range_map: HashMap<i64, Vec<(i64, u64, u64)>>,
    precision_indices: GeometricRandomNaturalValues<usize>,
    ranges: VariableRangeGenerator,
}

impl<T: PrimitiveFloat> Iterator for SpecialRandomPositiveFiniteFloatInclusiveRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let sci_exponent = self.sci_exponents.next().unwrap();
        let ae = self.ae;
        let be = self.be;
        let am = self.am;
        let bm = self.bm;
        let precision_ranges = self
            .precision_range_map
            .entry(sci_exponent)
            .or_insert_with(|| {
                let am = if sci_exponent == ae {
                    am
                } else {
                    T::from_integer_mantissa_and_exponent(1, sci_exponent)
                        .unwrap()
                        .raw_mantissa()
                };
                let bm = if sci_exponent == be {
                    bm
                } else {
                    T::from_integer_mantissa_and_exponent(1, sci_exponent + 1)
                        .unwrap()
                        .next_lower()
                        .raw_mantissa()
                };
                (1..=T::max_precision_for_sci_exponent(sci_exponent))
                    .filter_map(|p| mantissas_inclusive::<T>(sci_exponent, am, bm, p))
                    .collect_vec()
            });
        assert!(!precision_ranges.is_empty());
        let i = self.precision_indices.next().unwrap() % precision_ranges.len();
        let t = precision_ranges[i];
        let mantissa = (self.ranges.next_in_inclusive_range(t.1, t.2) << 1) | 1;
        Some(T::from_integer_mantissa_and_exponent(mantissa, t.0).unwrap())
    }
}

fn special_random_positive_finite_float_inclusive_range<T: PrimitiveFloat>(
    seed: Seed,
    a: T,
    b: T,
    mean_sci_exponent_numerator: u64,
    mean_sci_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
) -> SpecialRandomPositiveFiniteFloatInclusiveRange<T> {
    assert!(a.is_finite());
    assert!(b.is_finite());
    assert!(a > T::ZERO);
    assert!(a <= b);
    let (am, ae) = a.raw_mantissa_and_exponent();
    let (bm, be) = b.raw_mantissa_and_exponent();
    let ae = if ae == 0 {
        i64::wrapping_from(am.significant_bits()) + T::MIN_EXPONENT - 1
    } else {
        i64::wrapping_from(ae) - T::MAX_EXPONENT
    };
    let be = if be == 0 {
        i64::wrapping_from(bm.significant_bits()) + T::MIN_EXPONENT - 1
    } else {
        i64::wrapping_from(be) - T::MAX_EXPONENT
    };
    SpecialRandomPositiveFiniteFloatInclusiveRange {
        phantom: PhantomData,
        am,
        bm,
        ae,
        be,
        sci_exponents: geometric_random_signed_inclusive_range(
            seed.fork("exponents"),
            ae,
            be,
            mean_sci_exponent_numerator,
            mean_sci_exponent_denominator,
        ),
        precision_range_map: HashMap::new(),
        precision_indices: geometric_random_unsigneds(
            seed.fork("precisions"),
            mean_precision_numerator,
            mean_precision_denominator,
        ),
        ranges: VariableRangeGenerator::new(seed.fork("ranges")),
    }
}

#[allow(clippy::large_enum_variant)]
#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum SpecialRandomFiniteFloatInclusiveRange<T: PrimitiveFloat> {
    AllPositive(SpecialRandomPositiveFiniteFloatInclusiveRange<T>),
    AllNegative(SpecialRandomPositiveFiniteFloatInclusiveRange<T>),
    PositiveAndNegative(
        RandomBools,
        SpecialRandomPositiveFiniteFloatInclusiveRange<T>,
        SpecialRandomPositiveFiniteFloatInclusiveRange<T>,
    ),
}

impl<T: PrimitiveFloat> Iterator for SpecialRandomFiniteFloatInclusiveRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            SpecialRandomFiniteFloatInclusiveRange::AllPositive(ref mut xs) => xs.next(),
            SpecialRandomFiniteFloatInclusiveRange::AllNegative(ref mut xs) => {
                xs.next().map(|x| -x)
            }
            SpecialRandomFiniteFloatInclusiveRange::PositiveAndNegative(
                ref mut bs,
                ref mut xs,
                ref mut ys,
            ) => {
                if bs.next().unwrap() {
                    xs.next()
                } else {
                    ys.next().map(|x| -x)
                }
            }
        }
    }
}

fn special_random_finite_float_inclusive_range<T: PrimitiveFloat>(
    seed: Seed,
    a: T,
    b: T,
    mean_sci_exponent_numerator: u64,
    mean_sci_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
) -> SpecialRandomFiniteFloatInclusiveRange<T> {
    assert!(a.is_finite());
    assert!(b.is_finite());
    assert_ne!(a, T::ZERO);
    assert_ne!(b, T::ZERO);
    assert!(a <= b);
    if a > T::ZERO {
        SpecialRandomFiniteFloatInclusiveRange::AllPositive(
            special_random_positive_finite_float_inclusive_range(
                seed,
                a,
                b,
                mean_sci_exponent_numerator,
                mean_sci_exponent_denominator,
                mean_precision_numerator,
                mean_precision_denominator,
            ),
        )
    } else if b < T::ZERO {
        SpecialRandomFiniteFloatInclusiveRange::AllNegative(
            special_random_positive_finite_float_inclusive_range(
                seed,
                -b,
                -a,
                mean_sci_exponent_numerator,
                mean_sci_exponent_denominator,
                mean_precision_numerator,
                mean_precision_denominator,
            ),
        )
    } else {
        SpecialRandomFiniteFloatInclusiveRange::PositiveAndNegative(
            random_bools(seed.fork("bs")),
            special_random_positive_finite_float_inclusive_range(
                seed,
                T::MIN_POSITIVE_SUBNORMAL,
                b,
                mean_sci_exponent_numerator,
                mean_sci_exponent_denominator,
                mean_precision_numerator,
                mean_precision_denominator,
            ),
            special_random_positive_finite_float_inclusive_range(
                seed,
                T::MIN_POSITIVE_SUBNORMAL,
                -a,
                mean_sci_exponent_numerator,
                mean_sci_exponent_denominator,
                mean_precision_numerator,
                mean_precision_denominator,
            ),
        )
    }
}

/// Generates random primitive floats in a range.
///
/// This `enum` is created by [`special_random_primitive_float_range`]; see its documentation for
/// more.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum SpecialRandomFloatInclusiveRange<T: PrimitiveFloat> {
    OnlySpecial(RandomValuesFromVec<T>),
    NoSpecial(Box<SpecialRandomFiniteFloatInclusiveRange<T>>),
    Special(Box<WithSpecialValues<SpecialRandomFiniteFloatInclusiveRange<T>>>),
}

impl<T: PrimitiveFloat> Iterator for SpecialRandomFloatInclusiveRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            SpecialRandomFloatInclusiveRange::OnlySpecial(ref mut xs) => xs.next(),
            SpecialRandomFloatInclusiveRange::NoSpecial(ref mut xs) => xs.next(),
            SpecialRandomFloatInclusiveRange::Special(ref mut xs) => xs.next(),
        }
    }
}

/// Generates random primitive floats in the half-open interval $[a, b)$.
///
/// Simpler floats (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the numerator and denominator of the probability that any special values
/// (positive or negative zero or infinity) are generated, provided that they are in the range. You
/// can also specify the mean absolute sci-exponent and precision by passing the numerators and
/// denominators of their means of the finite floats.
///
/// But note that the means are only approximate, since the distributions we are sampling are
/// truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual mean is lower than the specified means.
/// - However, increasing the approximate mean increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater the smallest absolute of any sci-exponent of a
///   float in the range, and the precision mean greater than 2, but they may be as high as you
///   like.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater the smallest absolute value of any
///   sci-exponent of a float in the range, and the precision mean greater than 2, but they may be
///   as high as you like.
///
/// `NaN` is never generated.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if $a$ or $b$ are `NaN`, if $a$ is greater than or equal to $b$ in the `NiceFloat`
/// ordering, if any of the denominators are zero, if the special probability is greater than 1, if
/// the mean precision is less than 2, or if the mean sci-exponent is less than or equal to the
/// minimum absolute value of any sci-exponent in the range.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::special_random_primitive_float_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         special_random_primitive_float_range::<f32>(
///             EXAMPLE_SEED,
///             core::f32::consts::E,
///             core::f32::consts::PI,
///             10,
///             1,
///             10,
///             1,
///             1,
///             100
///         )
///         .map(NiceFloat),
///         20
///     ),
///     "[2.9238281, 2.953125, 3.0, 2.8671875, 2.8125, 3.125, 3.015625, 2.8462658, 3.140625, \
///     2.875, 3.0, 2.75, 3.0, 2.71875, 2.75, 3.0214844, 2.970642, 3.0179443, 2.968872, 2.75, ...]"
/// );
/// ```
pub fn special_random_primitive_float_range<T: PrimitiveFloat>(
    seed: Seed,
    a: T,
    b: T,
    mean_sci_exponent_numerator: u64,
    mean_sci_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
) -> SpecialRandomFloatInclusiveRange<T> {
    assert!(!a.is_nan());
    assert!(!b.is_nan());
    assert!(NiceFloat(a) < NiceFloat(b));
    special_random_primitive_float_inclusive_range(
        seed,
        a,
        b.next_lower(),
        mean_sci_exponent_numerator,
        mean_sci_exponent_denominator,
        mean_precision_numerator,
        mean_precision_denominator,
        mean_special_p_numerator,
        mean_special_p_denominator,
    )
}

/// Generates random primitive floats in the closed interval $[a, b]$.
///
/// Simpler floats (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the numerator and denominator of the probability that any special values
/// (positive or negative zero or infinity) are generated, provided that they are in the range. You
/// can also specify the mean absolute sci-exponent and precision by passing the numerators and
/// denominators of their means of the finite floats.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater the smallest absolute value of any
///   sci-exponent of a float in the range, and the precision mean greater than 2, but they may be
///   as high as you like.
///
/// `NaN` is never generated.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if $a$ or $b$ are `NaN`, if $a$ is greater than $b$ in the `NiceFloat` ordering, if any
/// of the denominators are zero, if the special probability is greater than 1, if the mean
/// precision is less than 2, or if the mean sci-exponent is less than or equal to the minimum
/// absolute value of any sci-exponent in the range.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_base::num::random::special_random_primitive_float_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         special_random_primitive_float_inclusive_range::<f32>(
///             EXAMPLE_SEED,
///             core::f32::consts::E,
///             core::f32::consts::PI,
///             10,
///             1,
///             10,
///             1,
///             1,
///             100
///         )
///         .map(NiceFloat),
///         20
///     ),
///     "[2.9238281, 2.953125, 3.0, 2.8671875, 2.8125, 3.125, 3.015625, 2.8462658, 3.140625, \
///     2.875, 3.0, 2.75, 3.0, 2.71875, 2.75, 3.0214844, 2.970642, 3.0179443, 2.968872, 2.75, ...]"
/// );
/// ```
pub fn special_random_primitive_float_inclusive_range<T: PrimitiveFloat>(
    seed: Seed,
    mut a: T,
    mut b: T,
    mean_sci_exponent_numerator: u64,
    mean_sci_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
) -> SpecialRandomFloatInclusiveRange<T> {
    assert!(!a.is_nan());
    assert!(!b.is_nan());
    assert!(NiceFloat(a) <= NiceFloat(b));
    assert_ne!(mean_special_p_denominator, 0);
    assert!(mean_special_p_numerator <= mean_special_p_denominator);
    assert_ne!(mean_precision_denominator, 0);
    assert!(mean_precision_numerator > mean_precision_denominator);
    let only_special =
        a == T::INFINITY || b == T::NEGATIVE_INFINITY || a == T::ZERO && b == T::ZERO;
    let mut special_values = Vec::new();
    if a == T::NEGATIVE_INFINITY {
        special_values.push(a);
        a = -T::MAX_FINITE;
    }
    if b == T::INFINITY {
        special_values.push(b);
        b = T::MAX_FINITE;
    }
    if NiceFloat(a) <= NiceFloat(T::NEGATIVE_ZERO) && NiceFloat(b) >= NiceFloat(T::NEGATIVE_ZERO) {
        special_values.push(T::NEGATIVE_ZERO);
    }
    if NiceFloat(a) <= NiceFloat(T::ZERO) && NiceFloat(b) >= NiceFloat(T::ZERO) {
        special_values.push(T::ZERO);
    }
    if a == T::ZERO {
        a = T::MIN_POSITIVE_SUBNORMAL;
    }
    if b == T::ZERO {
        b = -T::MIN_POSITIVE_SUBNORMAL;
    }
    if only_special {
        SpecialRandomFloatInclusiveRange::OnlySpecial(random_values_from_vec(seed, special_values))
    } else if special_values.is_empty() {
        SpecialRandomFloatInclusiveRange::NoSpecial(Box::new(
            special_random_finite_float_inclusive_range(
                seed,
                a,
                b,
                mean_sci_exponent_numerator,
                mean_sci_exponent_denominator,
                mean_precision_numerator,
                mean_precision_denominator,
            ),
        ))
    } else {
        SpecialRandomFloatInclusiveRange::Special(Box::new(with_special_values(
            seed,
            special_values,
            mean_special_p_numerator,
            mean_special_p_denominator,
            &|seed| {
                special_random_finite_float_inclusive_range(
                    seed,
                    a,
                    b,
                    mean_sci_exponent_numerator,
                    mean_sci_exponent_denominator,
                    mean_precision_numerator,
                    mean_precision_denominator,
                )
            },
        )))
    }
}

/// Generates unsigneds sampled from ranges. A single generator can sample from different ranges of
/// different types.
///
/// This `struct` is created by [`VariableRangeGenerator::new`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct VariableRangeGenerator {
    xs: RandomPrimitiveInts<u32>,
    x: u32,
    in_inner_loop: bool,
    remaining_x_bits: u64,
}

impl VariableRangeGenerator {
    /// Generates unsigneds sampled from ranges. A single generator can sample from different ranges
    /// of different types.
    ///
    /// If you only need to generate values from a single range, it is slightly more efficient to
    /// use [`random_unsigned_bit_chunks`], [`random_unsigneds_less_than`],
    /// [`random_unsigned_range`], or [`random_unsigned_inclusive_range`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::random::VariableRangeGenerator;
    /// use malachite_base::random::EXAMPLE_SEED;
    ///
    /// let mut generator = VariableRangeGenerator::new(EXAMPLE_SEED);
    /// assert_eq!(generator.next_bit_chunk::<u16>(10), 881);
    /// assert_eq!(generator.next_less_than::<u8>(100), 34);
    /// assert_eq!(generator.next_in_range::<u32>(10, 20), 16);
    /// assert_eq!(generator.next_in_inclusive_range::<u64>(10, 20), 14);
    /// ```
    pub fn new(seed: Seed) -> VariableRangeGenerator {
        VariableRangeGenerator {
            xs: random_primitive_ints(seed),
            x: 0,
            in_inner_loop: false,
            remaining_x_bits: 0,
        }
    }

    /// Uniformly generates a `bool`.
    ///
    /// $$
    /// $P(\text{false}) = P(\text{true}) = \frac{1}{2}$.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::random::VariableRangeGenerator;
    /// use malachite_base::random::EXAMPLE_SEED;
    ///
    /// let mut xs = Vec::with_capacity(10);
    /// let mut generator = VariableRangeGenerator::new(EXAMPLE_SEED);
    /// for _ in 0..10 {
    ///     xs.push(generator.next_bool());
    /// }
    /// assert_eq!(
    ///     xs,
    ///     &[true, false, true, false, true, true, true, true, true, false]
    /// );
    /// ```
    pub fn next_bool(&mut self) -> bool {
        self.xs.next().unwrap().odd()
    }

    /// Uniformly generates an unsigned integer with up to some number of bits.
    ///
    /// $$
    /// P(x) = \\begin{cases}
    ///     2^{-c} & \text{if} \\quad 0 \\leq x < 2^c, \\\\
    ///     0 & \text{if} \\quad \\text{otherwise,}
    /// \\end{cases}
    /// $$
    /// where $c$ is `chunk_size`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `chunk_size` is zero or greater than the width of the type.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::random::VariableRangeGenerator;
    /// use malachite_base::random::EXAMPLE_SEED;
    ///
    /// let mut xs = Vec::with_capacity(10);
    /// let mut generator = VariableRangeGenerator::new(EXAMPLE_SEED);
    /// for _ in 0..10 {
    ///     xs.push(generator.next_bit_chunk::<u8>(3));
    /// }
    /// assert_eq!(xs, &[1, 6, 5, 7, 6, 3, 1, 2, 4, 5]);
    /// ```
    pub fn next_bit_chunk<T: PrimitiveUnsigned>(&mut self, chunk_size: u64) -> T {
        assert_ne!(chunk_size, 0);
        assert!(chunk_size <= T::WIDTH);
        let mut y = T::ZERO;
        let mut remaining_y_bits = chunk_size;
        loop {
            if !self.in_inner_loop {
                self.x = self.xs.next().unwrap();
                self.remaining_x_bits = u32::WIDTH;
                self.in_inner_loop = true;
            }
            while self.remaining_x_bits != 0 {
                let y_index = chunk_size - remaining_y_bits;
                if self.remaining_x_bits <= remaining_y_bits {
                    y |= T::wrapping_from(self.x) << y_index;
                    remaining_y_bits -= self.remaining_x_bits;
                    self.remaining_x_bits = 0;
                } else {
                    y |= T::wrapping_from(self.x).mod_power_of_2(remaining_y_bits) << y_index;
                    self.x >>= remaining_y_bits;
                    self.remaining_x_bits -= remaining_y_bits;
                    remaining_y_bits = 0;
                }
                if remaining_y_bits == 0 {
                    return y;
                }
            }
            self.in_inner_loop = false;
        }
    }

    /// Uniformly generates a random unsigned integer less than a positive limit.
    ///
    /// $$
    /// P(x) = \\begin{cases}
    ///     \frac{1}{\\ell} & \text{if} \\quad x < \\ell \\\\
    ///     0 & \\text{otherwise}
    /// \\end{cases}
    /// $$
    /// where $\ell$ is `limit`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `limit` is 0.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::random::VariableRangeGenerator;
    /// use malachite_base::random::EXAMPLE_SEED;
    ///
    /// let mut xs = Vec::with_capacity(10);
    /// let mut generator = VariableRangeGenerator::new(EXAMPLE_SEED);
    /// for _ in 0..10 {
    ///     xs.push(generator.next_less_than(10u8));
    /// }
    /// assert_eq!(xs, &[1, 7, 5, 4, 6, 4, 2, 8, 1, 7]);
    /// ```
    pub fn next_less_than<T: PrimitiveUnsigned>(&mut self, limit: T) -> T {
        assert_ne!(limit, T::ZERO);
        if limit == T::ONE {
            T::ZERO
        } else {
            let chunk_size = limit.ceiling_log_base_2();
            loop {
                let x = self.next_bit_chunk(chunk_size);
                if x < limit {
                    return x;
                }
            }
        }
    }

    /// Uniformly generates a random unsigned integer in the half-open interval $[a, b)$.
    ///
    /// $a$ must be less than $b$. This function cannot create a range that includes `T::MAX`; for
    /// that, use [`next_in_inclusive_range`](Self::next_in_inclusive_range).
    ///
    /// $$
    /// P(x) = \\begin{cases}
    ///     \frac{1}{b-a} & \text{if} \\quad a \leq x < b, \\\\
    ///     0 & \\text{otherwise.}
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if $a \geq b$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::random::VariableRangeGenerator;
    /// use malachite_base::random::EXAMPLE_SEED;
    ///
    /// let mut xs = Vec::with_capacity(10);
    /// let mut generator = VariableRangeGenerator::new(EXAMPLE_SEED);
    /// for _ in 0..10 {
    ///     xs.push(generator.next_in_range(10u8, 20));
    /// }
    /// assert_eq!(xs, &[11, 17, 15, 14, 16, 14, 12, 18, 11, 17]);
    /// ```
    pub fn next_in_range<T: PrimitiveUnsigned>(&mut self, a: T, b: T) -> T {
        self.next_less_than(b - a) + a
    }

    /// Uniformly generates a random unsigned integer in the closed interval $[a, b]$.
    ///
    /// $a$ must be less than or equal to $b$.
    ///
    /// $$
    /// P(x) = \\begin{cases}
    ///     \frac{1}{b-a+1} & \text{if} \\quad a \leq x \leq b, \\\\
    ///     0 & \\text{otherwise.}
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if $a > b$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::random::VariableRangeGenerator;
    /// use malachite_base::random::EXAMPLE_SEED;
    ///
    /// let mut xs = Vec::with_capacity(10);
    /// let mut generator = VariableRangeGenerator::new(EXAMPLE_SEED);
    /// for _ in 0..10 {
    ///     xs.push(generator.next_in_inclusive_range(10u8, 19));
    /// }
    /// assert_eq!(xs, &[11, 17, 15, 14, 16, 14, 12, 18, 11, 17]);
    /// ```
    pub fn next_in_inclusive_range<T: PrimitiveUnsigned>(&mut self, a: T, b: T) -> T {
        if a == T::ZERO && b == T::MAX {
            self.next_bit_chunk(T::WIDTH)
        } else {
            self.next_less_than(b - a + T::ONE) + a
        }
    }
}

/// Iterators that generate primitive integers from geometric-like distributions.
pub mod geometric;

/// Iterators that generate primitive integers that tend to have long runs of binary 0s and 1s.
///
/// Integers with long runs of 0s and 1s are good for testing; they're more likely to result in
/// carries and borrows than uniformly random integers. This idea was inspired by GMP's
/// `mpz_rrandomb` function, although striped integer generators are more general: they can also
/// produce integers with runs that are shorter than average, so that they tend to contain
/// alternating bits like $1010101$.
///
/// Let the average length of a run of 0s and 1s be $m$. The functions in this module allow the user
/// to specify a rational $m$ through the parameters `m_numerator` and `m_denominator`. Since any
/// binary sequence has an average run length of at least 1, $m$ must be at least 1; but if it is
/// exactly 1 then the sequence is strictly alternating and no longer random, so 1 is not allowed
/// either. if $m$ is between 1 and 2, the sequence is less likely to have two equal adjacent bits
/// than a uniformly random sequence. If $m$ is 2, the sequence is uniformly random. If $m$ is
/// greater than 2 (the most useful case), the sequence tends to have long runs of 0s and 1s.
///
/// # Details
///
/// A random striped sequence with parameter $m \geq 1$ is an infinite sequence of bits, defined as
/// follows. The first bit is 0 or 1 with equal probability. Every subsequent bit has a $1/m$
/// probability of being different than the preceding bit. Notice that every sequence has an equal
/// probability as its negation. Also, if $m > 1$, any sequence has a nonzero probability of
/// occurring.
///
/// * $m=1$ is disallowed. If it were allowed, the sequence would be either
///   $01010101010101010101\ldots$ or $10101010101010101010\ldots$.
/// * If $1<m<2$, the sequence tends to alternate between 0 and 1 more often than a uniformly random
///   sequence. A sample sequence with $m=33/32$ is
///   $1010101010101010101010110101010101010101\ldots$.
/// * If $m=2$, the sequence is uniformly random. A sample sequence with $m=2$ is
///   $1100110001101010100101101001000001100001\ldots$.
/// * If $m>2$, the sequence tends to have longer runs of 0s and 1s than a uniformly random
///   sequence. A sample sequence with $m=32$ is $1111111111111111110000000011111111111111\ldots$.
///
/// An alternative way to generate a striped sequence is to start with 0 or 1 with equal probability
/// and then determine the length of each block of equal bits using a geometric distribution with
/// mean $m$. In practice, this isn't any more efficient than the naive algorithm.
///
/// We can generate a random striped unsigned integer of type `T` by taking the first $W$ bits of a
/// striped sequence. Fixing the parameter $m$ defines a distribution over `T`s. A few things can be
/// said about the probability $P_m(n)$ of an unsigned integer $n$ of width $W$ being generated:
/// * $P_m(n) = P_m(\lnot n)$
/// * $P_m(0) = P_m(2^W-1) = \frac{1}{2} \left ( 1-\frac{1}{m} \right )^{W-1}$. If $m>2$, this is
///   the maximum probability achieved; if $m<2$, the minimum.
/// * $P_m(\lfloor 2^W/3 \rfloor) = P_m(\lfloor 2^{W+1}/3 \rfloor) = 1/(2m^{W-1})$. If $m>2$, this
///   is the minimum probability achieved; if $m<2$, the maximum.
/// * Because of these distributions' symmetry, their mean is $(2^W-1)/2$ and their skewness is 0.
///   It's hard to say anything about their standard deviations or excess kurtoses, although these
///   can be computed quickly for specific values of $m$ when $W$ is 8 or 16.
///
/// We can similarly generate random striped signed integers of width $W$. The sign bit is chosen
/// uniformly, and the remaining $W-1$ are taken from a striped sequence.
///
/// To generate striped integers from a range, the integers are constructed one bit at a time. Some
/// bits are forced; they must be 0 or 1 in order for the final integer to be within the specified
/// range. If a bit is _not_ forced, it is different from the preceding bit with probability $1/m$.
pub mod striped;
