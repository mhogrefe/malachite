use iterators::{nonzero_values, NonzeroValues};
use num::basic::integers::PrimitiveInt;
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;
use num::iterator::{iterator_to_bit_chunks, IteratorToBitChunks};
use num::logic::traits::BitAccess;
use rand::Rng;
use rand_chacha::ChaCha20Rng;
use random::Seed;
use std::convert::identity;
use std::fmt::Debug;

/// Uniformly generates random primitive integers.
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
impl_trivial_random_primitive_ints!(u32);
impl_trivial_random_primitive_ints!(u64);
impl_trivial_random_primitive_ints!(u128);
impl_trivial_random_primitive_ints!(usize);
impl_trivial_random_primitive_ints!(i32);
impl_trivial_random_primitive_ints!(i64);
impl_trivial_random_primitive_ints!(i128);
impl_trivial_random_primitive_ints!(isize);

fn _get_random<T: PrimitiveInt>(rng: &mut ChaCha20Rng, state: &mut ThriftyRandomState) -> T {
    if state.bits_left == 0 {
        state.x = rng.gen();
        state.bits_left = 32 - T::WIDTH;
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
                _get_random(rng, state)
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
/// This `struct` is created by the `random_primitive_ints` method. See its documentation for
/// more.
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
/// This `enum` is created by the `random_unsigneds_less_than` method. See its documentation for
/// more.
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
/// This `struct` is created by the `random_unsigned_range` method. See its documentation for more.
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
/// This `struct` is created by the `random_unsigned_inclusive_range` method. See its documentation
/// for more.
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
apply_to_unsigned_signed_pair!(impl_has_random_signed_range);

/// Uniformly generates random signed integers in the half-open interval $[a, b)$.
///
/// This `struct` is created by the `random_signed_range` method. See its documentation for more.
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
/// This `struct` is created by the `random_signed_inclusive_range` method. See its documentation
/// for more.
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

/// Uniformly generates unsigned integers of up to `chunk_size` bits.
///
/// This `struct` is created by the `random_unsigned_bit_chunks` method. See its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct RandomUnsignedBitChunks<T: PrimitiveUnsigned> {
    xs: IteratorToBitChunks<RandomPrimitiveInts<T>, T, T>,
}

impl<T: PrimitiveUnsigned> Iterator for RandomUnsignedBitChunks<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.xs.next_with_wrapping(identity)
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
apply_to_unsigned_signed_pair!(impl_random_signed_chunkable);

/// Uniformly generates signed integers of up to `chunk_size` bits.
///
/// This `struct` is created by the `random_signed_bit_chunks` method. See its documentation for
/// more.
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

/// Generates an iterator's values, but with the highest bit set.
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
/// $P(x) = 2^{-W}$, where $W$ is `T::WIDTH`.
///
/// The output length is infinite.
///
/// # Complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_primitive_ints::<u8>(EXAMPLE_SEED).take(10).collect::<Vec<_>>(),
///     &[113, 239, 69, 108, 228, 210, 168, 161, 87, 32]
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
///     \\frac{1}{2^W-1} & x > 0 \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $W$ is `T::WIDTH`.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_positive_unsigneds;
///
/// assert_eq!(
///     random_positive_unsigneds::<u8>(EXAMPLE_SEED).take(10).collect::<Vec<_>>(),
///     &[113, 239, 69, 108, 228, 210, 168, 161, 87, 32]
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
///     \\frac{1}{2^{W-1}-1} & x > 0 \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $W$ is `T::WIDTH`.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_positive_signeds;
///
/// assert_eq!(
///     random_positive_signeds::<i8>(EXAMPLE_SEED).take(10).collect::<Vec<_>>(),
///     &[113, 94, 23, 98, 70, 92, 52, 84, 33, 47]
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
///     2^{1-W} & x < 0\\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $W$ is `T::WIDTH`.
///
/// The output length is infinite.
///
/// # Complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_negative_signeds;
///
/// assert_eq!(
///     random_negative_signeds::<i8>(EXAMPLE_SEED).take(10).collect::<Vec<_>>(),
///     &[-15, -34, -105, -30, -58, -36, -76, -44, -95, -81]
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
///     2^{1-W} & x \geq 0 \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $W$ is `T::WIDTH`.
///
/// The output length is infinite.
///
/// # Complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_natural_signeds;
///
/// assert_eq!(
///     random_natural_signeds::<i8>(EXAMPLE_SEED).take(10).collect::<Vec<_>>(),
///     &[113, 94, 23, 98, 70, 92, 52, 84, 33, 47]
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
///     \\frac{1}{2^W-1} & x \\neq 0 \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $W$ is `T::WIDTH`.
///
/// The output length is infinite.
///
/// # Complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_nonzero_signeds;
///
/// assert_eq!(
///     random_nonzero_signeds::<i8>(EXAMPLE_SEED).take(10).collect::<Vec<_>>(),
///     &[113, -17, 69, 108, -28, -46, -88, -95, 87, 32]
/// )
/// ```
#[inline]
pub fn random_nonzero_signeds<T: PrimitiveSigned>(
    seed: Seed,
) -> NonzeroValues<RandomPrimitiveInts<T>> {
    nonzero_values(random_primitive_ints(seed))
}

/// Uniformly generates random unsigned integers less than a positive `limit`.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{\\ell} & x < \\ell \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $\ell$ is `limit`.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
///
/// Panics if `limit` is 0.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_unsigneds_less_than;
///
/// assert_eq!(
///     random_unsigneds_less_than::<u8>(EXAMPLE_SEED, 10).take(10).collect::<Vec<_>>(),
///     &[1, 7, 5, 4, 6, 4, 2, 8, 1, 7]
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
            random_unsigned_bit_chunks(seed, limit.ceiling_log_two()),
            limit,
        )
    }
}

/// Uniformly generates random unsigned integers in the half-open interval $[a, b)$.
///
/// `a` must be less than `b`. This function cannot create a range that includes `T::MAX`; for that,
/// use `random_unsigned_inclusive_range`.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{b-a} & a \leq x < b \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
///
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_unsigned_range;
///
/// assert_eq!(
///     random_unsigned_range::<u8>(EXAMPLE_SEED, 10, 20).take(10).collect::<Vec<_>>(),
///     &[11, 17, 15, 14, 16, 14, 12, 18, 11, 17]
/// )
/// ```
pub fn random_unsigned_range<T: PrimitiveUnsigned>(
    seed: Seed,
    a: T,
    b: T,
) -> RandomUnsignedRange<T> {
    if a >= b {
        panic!("a must be less than b. a: {}, b: {}", a, b);
    }
    RandomUnsignedRange {
        xs: random_unsigneds_less_than(seed, b - a),
        a,
    }
}

/// Uniformly generates random unsigned integers in the closed interval $[a, b]$.
///
/// `a` must be less than or equal to `b`.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{b-a+1} & a \leq x \leq b \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
///
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
///
/// assert_eq!(
///     random_unsigned_inclusive_range::<u8>(EXAMPLE_SEED, 10, 19).take(10).collect::<Vec<_>>(),
///     &[11, 17, 15, 14, 16, 14, 12, 18, 11, 17]
/// )
/// ```
pub fn random_unsigned_inclusive_range<T: PrimitiveUnsigned>(
    seed: Seed,
    a: T,
    b: T,
) -> RandomUnsignedInclusiveRange<T> {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    if a == T::ZERO && b == T::MAX {
        RandomUnsignedInclusiveRange::All(random_primitive_ints(seed))
    } else {
        RandomUnsignedInclusiveRange::NotAll(random_unsigneds_less_than(seed, b - a + T::ONE), a)
    }
}

/// Uniformly generates random signed integers in the half-open interval $[a, b)$.
///
/// `a` must be less than `b`. This function cannot create a range that includes `T::MAX`; for that,
/// use `random_signed_inclusive_range`.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{b-a} & a \leq x < b \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
///
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_signed_range;
///
/// assert_eq!(
///     random_signed_range::<i8>(EXAMPLE_SEED, -100, 100).take(10).collect::<Vec<_>>(),
///     &[13, -31, 8, 68, 61, -13, -68, 10, -17, 88]
/// )
/// ```
#[inline]
pub fn random_signed_range<T: PrimitiveSigned>(seed: Seed, a: T, b: T) -> RandomSignedRange<T> {
    if a >= b {
        panic!("a must be less than b. a: {}, b: {}", a, b);
    }
    RandomSignedRange {
        xs: T::new_unsigned_range(seed, a, b),
    }
}

/// Uniformly generates random signed integers in the closed interval $[a, b]$.
///
/// `a` must be less than or equal to `b`.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}{b-a+1} & a \leq x \leq b \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
///
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_signed_inclusive_range;
///
/// assert_eq!(
///     random_signed_inclusive_range::<i8>(EXAMPLE_SEED, -100, 99).take(10).collect::<Vec<_>>(),
///     &[13, -31, 8, 68, 61, -13, -68, 10, -17, 88]
/// )
/// ```
#[inline]
pub fn random_signed_inclusive_range<T: PrimitiveSigned>(
    seed: Seed,
    a: T,
    b: T,
) -> RandomSignedInclusiveRange<T> {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RandomSignedInclusiveRange {
        xs: T::new_unsigned_inclusive_range(seed, a, b),
    }
}

/// Uniformly generates unsigned integers of up to `chunk_size` bits.
///
/// $$
/// P(x) = \\begin{cases}
///     2^{-c} & 0 \\leq x < 2^c \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $c$ is `chunk_size`.
///
/// The output length is infinite.
///
/// # Complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `chunk_size` is zero or greater than `T::WIDTH`.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_unsigned_bit_chunks;
///
/// assert_eq!(
///     random_unsigned_bit_chunks::<u8>(EXAMPLE_SEED, 3).take(10).collect::<Vec<_>>(),
///     &[1, 6, 5, 7, 6, 3, 1, 2, 4, 5]
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

/// Uniformly generates signed integers of up to `chunk_size` bits.
///
/// The generated values will all be
/// non-negative unless `chunk_size` is equal to `T::WIDTH`.
///
/// $$
/// P(x) = \\begin{cases}
///     2^{-c} & c = W \\ \\text{or} \\ (c < W \\ \\text{and} \\ 0 \\leq x < 2^c) \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $c$ is `chunk_size` and $W$ is `T::WIDTH`.
///
/// The output length is infinite.
///
/// # Complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `chunk_size` is zero or greater than `T::WIDTH`.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_signed_bit_chunks;
///
/// assert_eq!(
///     random_signed_bit_chunks::<i8>(EXAMPLE_SEED, 3).take(10).collect::<Vec<_>>(),
///     &[1, 6, 5, 7, 6, 3, 1, 2, 4, 5]
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
///     2^{1-W} & 2^{W-1} \\leq x < 2^W \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $W$ is `T::WIDTH`.
///
/// The output length is infinite.
///
/// # Complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_highest_bit_set_unsigneds;
///
/// assert_eq!(
///     random_highest_bit_set_unsigneds::<u8>(EXAMPLE_SEED).take(10).collect::<Vec<_>>(),
///     &[241, 222, 151, 226, 198, 220, 180, 212, 161, 175],
/// )
/// ```
#[inline]
pub fn random_highest_bit_set_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
) -> RandomHighestBitSetValues<RandomUnsignedBitChunks<T>> {
    RandomHighestBitSetValues {
        xs: random_unsigned_bit_chunks(seed, T::WIDTH - 1),
        mask: T::power_of_two(T::WIDTH - 1),
    }
}

/// This module contains iterators that generate primitive integers from geometric-like
/// distributions.
pub mod geometric;

/// This module contains iterators that generate primitive integers that tend to have long runs of
/// binary 0s and 1s.
///
/// Integers with long runs of 0s and 1s are good for testing; they're more likely to result in
/// carries and borrows than uniformly random integers. This idea was inspired by GMP's
/// `mpz_rrandomb` function, although striped integers are more general: they can also have runs
/// that are shorter than average, so that they tend to contain alternating blocks like $1010101$.
///
/// Let the average length of a run of 0s and 1s be $m$. The functions in this module allow the user
/// to specify a rational $m$ through the parameters `m_numerator` and `m_denominator`. Since any
/// binary sequence has an average run length of at least 1, $m$ must be at least 1; but if it is
/// exactly 1 then the sequence is strictly alternating and no longer random, so 1 is excluded. if
/// $m$ is between 1 and 2, the sequence is less likely to have two equal adjacent bits than a
/// uniformly random sequence. If $m$ is 2, the sequence is uniformly random. If $m$ is greater than
/// 2 (the most useful case), the sequence tends to have long runs of 0s and 1s.
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
///   sequence. A sample sequence with $m=32$ is
///   $1111111111111111110000000011111111111111\ldots$.
///
/// An alternative way to generate a striped sequence is to start with 0 or 1 with equal probability
/// and then determine the length of each block of equal bits using a geometric distribution with
/// mean $m$. In practice, this isn't any more efficient than the naive algorithm.
///
/// We can generate a random striped unsigned integer of type `T` by taking the first `T::WIDTH`
/// bits of a striped sequence. Fixing the parameter $m$ defines a distribution over `T`s. A few
/// things can be said about the probability $P_m(n)$ of an unsigned integer $n$ of width $W$ being
/// generated :
/// * $P_m(n) = P_m(\lnot n)$
/// * $P_m(0) = P_m(2^W-1) = \frac{1}{2} \left ( 1-\frac{1}{m} \right )^{W-1}$. If $m>2$, this is
///   the maximum probability achieved; if $m<2$, the minimum.
/// * $P_m(\lfloor 2^W/3 \rfloor) = P_m(\lfloor 2^{W+1}/3 \rfloor) = 1/(2m^{W-1})$. If $m>2$,
///   this is the minimum probability achieved; if $m<2$, the maximum.
/// * Because of these distributions' symmetry, their mean is $(2^W-1)/2$ and their skewness is 0.
///   It's hard to say anything about their standard deviations or excess kurtoses, although these
///   can be computed quickly for specific values of $m$ when $W$ is 8 or 16.
///
/// We can similarly generate random striped signed integers of width `T`. The sign bit is chosen
/// uniformly, and the remaining `T::WIDTH - 1` are taken from a striped sequence.
///
/// Generating striped random values from an interval seems difficult. We can't shift the interval
/// by adding, since addition destroys the the values' stripiness. For this reason, iterators that
/// generate striped random values from an interval are not provided.
pub mod striped;
