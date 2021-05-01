use bools::random::{random_bools, weighted_random_bools, RandomBools, WeightedRandomBools};
use iterators::{nonzero_values, NonzeroValues};
use itertools::Itertools;
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::{ExactFrom, WrappingFrom};
use num::random::geometric::{
    geometric_random_unsigned_inclusive_range, geometric_random_unsigneds, mean_to_p_with_min,
    GeometricRandomNaturalValues,
};
use num::random::{
    random_unsigned_inclusive_range, random_unsigned_range, RandomUnsignedInclusiveRange,
    RandomUnsignedRange,
};
use random::Seed;
use std::iter::{repeat, Repeat};
use std::marker::PhantomData;

/// Generates bits from a striped random sequence.
///
/// See the module-level documentation.
#[derive(Clone, Debug)]
pub struct StripedBitSource {
    first_bit_of_block: bool,
    previous_bit: bool,
    bs: RandomBools,
    xs: WeightedRandomBools,
}

impl Iterator for StripedBitSource {
    type Item = bool;

    /// Gets a bit from this `StripedBitSource`. If this function is being called for the first
    /// time, the probabilities of a `true` or a `false` are equal. On subsequent calls, the
    /// probability of getting a bit different from the previous one is $1 / m$.
    ///
    /// To reset the bit source, so that the next call to `next` has equal probabilities of `true`
    /// or `false`, call `end_block`.
    ///
    /// # Expected worst-case complexity
    /// Constant time and additional memory.
    #[inline]
    fn next(&mut self) -> Option<bool> {
        self.previous_bit = if self.first_bit_of_block {
            self.first_bit_of_block = false;
            self.bs.next().unwrap()
        } else {
            self.previous_bit ^ self.xs.next().unwrap()
        };
        Some(self.previous_bit)
    }
}

impl StripedBitSource {
    /// Creates a new `StripedBitSource` with mean run length $m$, where $m$ is
    /// `m_numerator` / `m_denominator`.
    ///
    /// # Expected worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `m_denominator` is zero or if `m_numerator` < `m_denominator`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::random::EXAMPLE_SEED;
    /// use malachite_base::num::random::striped::StripedBitSource;
    ///
    /// let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, 4, 1);
    /// let mut string = String::with_capacity(40);
    /// for bit in bit_source.take(40) {
    ///     if bit {
    ///         string.push('1');
    ///     } else {
    ///         string.push('0');
    ///     }
    /// }
    /// assert_eq!(string, "0000000101100110000000011110000000001111");
    /// ```
    pub fn new(seed: Seed, m_numerator: u64, m_denominator: u64) -> StripedBitSource {
        assert_ne!(m_denominator, 0);
        assert!(m_numerator > m_denominator);
        let (numerator, denominator) = mean_to_p_with_min(1u64, m_numerator, m_denominator);
        StripedBitSource {
            first_bit_of_block: true,
            previous_bit: false,
            bs: random_bools(seed.fork("bs")),
            xs: weighted_random_bools(seed.fork("xs"), numerator, numerator + denominator),
        }
    }

    /// Resets this `StripedBitSource`, so that the next time `next` is called, the probabilities of
    /// `true` or `false` will be equal.
    ///
    /// # Expected worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::random::EXAMPLE_SEED;
    /// use malachite_base::num::random::striped::StripedBitSource;
    ///
    /// fn generate_string(bit_source: &mut StripedBitSource) -> String {
    ///     let mut string = String::with_capacity(40);
    ///     for bit in bit_source.take(40) {
    ///         if bit {
    ///             string.push('1');
    ///         } else {
    ///             string.push('0');
    ///         }
    ///     }
    ///     string
    /// }
    ///
    /// let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, 1000000, 1);
    /// let mut strings = Vec::with_capacity(5);
    /// for _ in 0..5 {
    ///     strings.push(generate_string(&mut bit_source));
    ///     bit_source.end_block();
    /// }
    /// assert_eq!(
    ///     strings,
    ///     &[
    ///         "0000000000000000000000000000000000000000",
    ///         "0000000000000000000000000000000000000000",
    ///         "0000000000000000000000000000000000000000",
    ///         "1111111111111111111111111111111111111111",
    ///         "0000000000000000000000000000000000000000"
    ///     ]
    /// );
    /// ```
    pub fn end_block(&mut self) {
        self.first_bit_of_block = true;
    }
}

/// Generates random unsigned integers from a random striped distribution.
#[derive(Clone, Debug)]
pub struct StripedRandomUnsignedBitChunks<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
    bits: StripedBitSource,
    chunk_size: usize,
}

impl<T: PrimitiveUnsigned> Iterator for StripedRandomUnsignedBitChunks<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.bits.end_block();
        let mut x = T::ZERO;
        for bit in (&mut self.bits).take(self.chunk_size) {
            x <<= 1;
            if bit {
                x |= T::ONE;
            }
        }
        Some(x)
    }
}

/// Generates random signed integers from a random striped distribution.
///
/// This `struct` is created by the `striped_random_signeds` function. See its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct StripedRandomSigneds<T: PrimitiveSigned> {
    phantom: PhantomData<*const T>,
    bits: StripedBitSource,
    bs: RandomBools,
}

impl<T: PrimitiveSigned> Iterator for StripedRandomSigneds<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.bits.end_block();
        let mut x = T::ZERO;
        for bit in (&mut self.bits).take(usize::wrapping_from(T::WIDTH) - 1) {
            x <<= 1;
            if bit {
                x |= T::ONE;
            }
        }
        if self.bs.next().unwrap() {
            x.set_bit(T::WIDTH - 1);
        }
        Some(x)
    }
}

/// Generates random natural (non-negative) signed integers from a random striped distribution.
///
/// This `struct` is created by the `striped_random_natural_signeds` function. See its documentation
/// for more.
#[derive(Clone, Debug)]
pub struct StripedRandomNaturalSigneds<T: PrimitiveSigned> {
    phantom: PhantomData<*const T>,
    bits: StripedBitSource,
}

impl<T: PrimitiveSigned> Iterator for StripedRandomNaturalSigneds<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.bits.end_block();
        let mut x = T::ZERO;
        for bit in (&mut self.bits).take(usize::wrapping_from(T::WIDTH) - 1) {
            x <<= 1;
            if bit {
                x |= T::ONE;
            }
        }
        Some(x)
    }
}

/// Generates random negative signed integers from a random striped distribution.
///
/// This `struct` is created by the `striped_random_negative_signeds` function. See its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct StripedRandomNegativeSigneds<T: PrimitiveSigned> {
    phantom: PhantomData<*const T>,
    bits: StripedBitSource,
}

impl<T: PrimitiveSigned> Iterator for StripedRandomNegativeSigneds<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.bits.end_block();
        let mut x = T::ZERO;
        for bit in (&mut self.bits).take(usize::wrapping_from(T::WIDTH) - 1) {
            x <<= 1;
            if bit {
                x |= T::ONE;
            }
        }
        x.set_bit(T::WIDTH - 1);
        Some(x)
    }
}

/// Generates random unsigned integers less than a positive limit, from a random striped
/// distribution.
///
/// This `struct` is created by the `striped_random_unsigneds_less_than` function. See its
/// documentation for more.
#[derive(Clone, Debug)]
struct StripedRandomUnsignedsLessThan<T: PrimitiveUnsigned> {
    pub(crate) xs: StripedRandomUnsignedBitChunks<T>,
    pub(crate) limit: T,
}

impl<T: PrimitiveUnsigned> Iterator for StripedRandomUnsignedsLessThan<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        loop {
            let x = self.xs.next();
            if x.unwrap() < self.limit {
                return x;
            }
        }
    }
}

/// Generates random unsigned integers from a random striped distribution.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `m_numerator` / `m_denominator`. See the module-level documentation.
///
/// The output length is infinite.
///
/// # Expected worst-case complexity
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `m_denominator` is zero or if `m_numerator` <= `m_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_unsigneds;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_unsigneds::<u8>(EXAMPLE_SEED, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect_vec(),
///     &["1", "1001100", "1111111", "11000011", "0", "10000000", "1111", "1110110", "0",
///     "11111000"]
/// )
/// ```
#[inline]
pub fn striped_random_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
    m_numerator: u64,
    m_denominator: u64,
) -> StripedRandomUnsignedBitChunks<T> {
    striped_random_unsigned_bit_chunks(seed, T::WIDTH, m_numerator, m_denominator)
}

/// Generates random positive unsigned integers from a random striped distribution.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `m_numerator` / `m_denominator`. See the module-level documentation.
///
/// The output length is infinite.
///
/// # Expected worst-case complexity
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `m_denominator` is zero or if `m_numerator` <= `m_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_positive_unsigneds;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_positive_unsigneds::<u8>(EXAMPLE_SEED, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect_vec(),
///     &["1", "1001100", "1111111", "11000011", "10000000", "1111", "1110110", "11111000",
///     "11111111", "11111101"]
/// )
/// ```
pub fn striped_random_positive_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
    m_numerator: u64,
    m_denominator: u64,
) -> NonzeroValues<StripedRandomUnsignedBitChunks<T>> {
    nonzero_values(striped_random_unsigneds(seed, m_numerator, m_denominator))
}

/// Generates random signed integers from a random striped distribution.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `m_numerator` / `m_denominator`. See the module-level documentation.
///
/// The output length is infinite.
///
/// # Expected worst-case complexity
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `m_denominator` is zero or if `m_numerator` <= `m_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_signeds;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_signeds::<i8>(EXAMPLE_SEED, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect_vec(),
///     &["1100001", "1000000", "1100000", "10000111", "1111", "10000001", "1111000", "100011",
///     "111101", "11111100"]
/// )
/// ```
pub fn striped_random_signeds<T: PrimitiveSigned>(
    seed: Seed,
    m_numerator: u64,
    m_denominator: u64,
) -> StripedRandomSigneds<T> {
    StripedRandomSigneds {
        phantom: PhantomData,
        bits: StripedBitSource::new(seed.fork("bits"), m_numerator, m_denominator),
        bs: random_bools(seed.fork("bs")),
    }
}

/// Generates random natural (non-negative) signed integers from a random striped distribution.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `m_numerator` / `m_denominator`. See the module-level documentation.
///
/// The output length is infinite.
///
/// # Expected worst-case complexity
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `m_denominator` is zero or if `m_numerator` <= `m_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_natural_signeds;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_natural_signeds::<i8>(EXAMPLE_SEED, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect_vec(),
///     &["0", "101100", "110000", "1111100", "1111", "1111110", "0", "111", "11101", "1100000"]
/// )
/// ```
pub fn striped_random_natural_signeds<T: PrimitiveSigned>(
    seed: Seed,
    m_numerator: u64,
    m_denominator: u64,
) -> StripedRandomNaturalSigneds<T> {
    StripedRandomNaturalSigneds {
        phantom: PhantomData,
        bits: StripedBitSource::new(seed, m_numerator, m_denominator),
    }
}

/// Generates random positive signed integers from a random striped distribution.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `m_numerator` / `m_denominator`. See the module-level documentation.
///
/// The output length is infinite.
///
/// # Expected worst-case complexity
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `m_denominator` is zero or if `m_numerator` <= `m_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_positive_signeds;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_positive_signeds::<i8>(EXAMPLE_SEED, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect_vec(),
///     &["101100", "110000", "1111100", "1111", "1111110", "111", "11101", "1100000", "1111111",
///     "1100000"]
/// )
/// ```
pub fn striped_random_positive_signeds<T: PrimitiveSigned>(
    seed: Seed,
    m_numerator: u64,
    m_denominator: u64,
) -> NonzeroValues<StripedRandomNaturalSigneds<T>> {
    nonzero_values(striped_random_natural_signeds(
        seed,
        m_numerator,
        m_denominator,
    ))
}

/// Generates random negative signed integers from a random striped distribution.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `m_numerator` / `m_denominator`. See the module-level documentation.
///
/// The output length is infinite.
///
/// # Expected worst-case complexity
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `m_denominator` is zero or if `m_numerator` <= `m_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_negative_signeds;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_negative_signeds::<i8>(EXAMPLE_SEED, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect_vec(),
///     &["10000000", "10101100", "10110000", "11111100", "10001111", "11111110", "10000000",
///     "10000111", "10011101", "11100000"]
/// )
/// ```
pub fn striped_random_negative_signeds<T: PrimitiveSigned>(
    seed: Seed,
    m_numerator: u64,
    m_denominator: u64,
) -> StripedRandomNegativeSigneds<T> {
    StripedRandomNegativeSigneds {
        phantom: PhantomData,
        bits: StripedBitSource::new(seed, m_numerator, m_denominator),
    }
}

/// Generates random nonzero signed integers from a random striped distribution.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `m_numerator` / `m_denominator`. See the module-level documentation.
///
/// The output length is infinite.
///
/// # Expected worst-case complexity
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `m_denominator` is zero or if `m_numerator` <= `m_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_nonzero_signeds;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_nonzero_signeds::<i8>(EXAMPLE_SEED, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect_vec(),
///     &["1100001", "1000000", "1100000", "10000111", "1111", "10000001", "1111000", "100011",
///     "111101", "11111100"]
/// )
/// ```
pub fn striped_random_nonzero_signeds<T: PrimitiveSigned>(
    seed: Seed,
    m_numerator: u64,
    m_denominator: u64,
) -> NonzeroValues<StripedRandomSigneds<T>> {
    nonzero_values(striped_random_signeds(seed, m_numerator, m_denominator))
}

/// Generates random unsigned integers of up to `chunk_size` bits from a random striped
/// distribution.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `m_numerator` / `m_denominator`. See the module-level documentation.
///
/// The output length is infinite.
///
/// # Expected worst-case complexity
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `m_denominator` is zero, if `m_numerator` <= `m_denominator`, or if `chunk_size` is
/// greater than `T::WIDTH`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_unsigned_bit_chunks;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_unsigned_bit_chunks::<u8>(EXAMPLE_SEED, 3, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect_vec(),
///     &["0", "0", "0", "101", "11", "100", "11", "11", "0", "111"]
/// )
/// ```
pub fn striped_random_unsigned_bit_chunks<T: PrimitiveUnsigned>(
    seed: Seed,
    chunk_size: u64,
    m_numerator: u64,
    m_denominator: u64,
) -> StripedRandomUnsignedBitChunks<T> {
    assert!(chunk_size <= T::WIDTH);
    StripedRandomUnsignedBitChunks {
        phantom: PhantomData,
        bits: StripedBitSource::new(seed, m_numerator, m_denominator),
        chunk_size: usize::exact_from(chunk_size),
    }
}

/// Generates a striped `Vec<bool>`, with a given length, from a `StripedBitSource`.
///
/// See the module-level documentation.
///
/// The output length is `len`.
///
/// # Expected worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and `n` is `len`.
///
/// # Examples
/// ```
/// use malachite_base::num::random::striped::{get_striped_bool_vec, StripedBitSource};
/// use malachite_base::random::EXAMPLE_SEED;
///
/// let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, 10, 1);
/// let bits: String = get_striped_bool_vec(&mut bit_source, 50)
///         .into_iter().map(|b| if b { '1' } else { '0' }).collect();
/// assert_eq!(bits, "00011111111111000000011111111111111000000000001111");
/// ```
pub fn get_striped_bool_vec(bit_source: &mut StripedBitSource, len: u64) -> Vec<bool> {
    bit_source.end_block();
    bit_source.take(usize::exact_from(len)).collect()
}

/// Generates random striped `Vec<bool>`s.
#[derive(Clone, Debug)]
pub struct StripedRandomBoolVecs<I: Iterator<Item = u64>> {
    lengths: I,
    bit_source: StripedBitSource,
}

impl<I: Iterator<Item = u64>> Iterator for StripedRandomBoolVecs<I> {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Vec<bool>> {
        Some(get_striped_bool_vec(
            &mut self.bit_source,
            self.lengths.next().unwrap(),
        ))
    }
}

/// Generates random striped `Vec<bool>`s, with lengths from an iterator.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `mean_stripe_numerator` / `mean_stripe_denominator`. See the module-level documentation.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero or if
/// `mean_stripe_numerator` <= `mean_stripe_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::random::striped::striped_random_bool_vecs_from_length_iterator;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
/// use malachite_base::vecs::random_values_from_vec;
///
/// let bss = striped_random_bool_vecs_from_length_iterator(
///     EXAMPLE_SEED,
///     &|seed| random_values_from_vec(seed, vec![0, 2, 4]),
///     10,
///     1,
/// )
///     .take(20)
///     .map(|bs| bs.into_iter().map(|b| if b { '1' } else { '0' }).collect())
///     .collect_vec();
/// assert_eq!(
///     bss.iter().map(String::as_str).collect_vec(),
///     &[
///         "00", "0000", "00", "0000", "0000", "11", "", "00", "", "1111", "0001", "11", "1100",
///         "00", "0000", "0000", "1110", "", "0000", ""
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_bool_vecs_from_length_iterator<I: Iterator<Item = u64>>(
    seed: Seed,
    lengths_gen: &dyn Fn(Seed) -> I,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
) -> StripedRandomBoolVecs<I> {
    StripedRandomBoolVecs {
        lengths: lengths_gen(seed.fork("lengths")),
        bit_source: StripedBitSource::new(
            seed.fork("bit_source"),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
    }
}

/// Generates random striped `Vec<bool>`s of a given length.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `mean_stripe_numerator` / `mean_stripe_denominator`. See the module-level documentation.
///
/// If `len` is 0, the output consists of the empty list, repeated.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::random::striped::striped_random_fixed_length_bool_vecs;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
///
/// let bss = striped_random_fixed_length_bool_vecs(EXAMPLE_SEED, 10, 1, 5)
///     .take(20)
///     .map(|bs| bs.into_iter().map(|b| if b { '1' } else { '0' }).collect())
///     .collect_vec();
/// assert_eq!(
///     bss.iter().map(String::as_str).collect_vec(),
///     &[
///         "00000", "00000", "00000", "00000", "00011", "11000", "00000", "11111", "01111",
///         "11111", "10000", "00011", "00000", "00000", "11000", "00000", "11111", "00000",
///         "00000", "11111"
///     ]
/// );
/// ```
pub fn striped_random_fixed_length_bool_vecs(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    len: u64,
) -> StripedRandomBoolVecs<Repeat<u64>> {
    striped_random_bool_vecs_from_length_iterator(
        seed,
        &|_| repeat(len),
        mean_stripe_numerator,
        mean_stripe_denominator,
    )
}

/// Generates random striped `Vec<bool>`s.
///
/// The lengths of the `Vec`s are sampled from a geometric distribution with a specified mean $m$,
/// equal to `mean_length_numerator / mean_length_denominator`. $m$ must be greater than 0.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `mean_stripe_numerator` / `mean_stripe_denominator`. See the module-level documentation.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is
/// `mean_length_numerator` / `mean_length_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator` <= `mean_stripe_denominator`, if `mean_length_numerator` or
/// `mean_length_denominator` are zero, or, if after being reduced to lowest terms, their sum is
/// greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::random::striped::striped_random_bool_vecs;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
///
/// let bss = striped_random_bool_vecs(EXAMPLE_SEED, 10, 1, 2, 1)
///     .take(20)
///     .map(|bs| bs.into_iter().map(|b| if b { '1' } else { '0' }).collect())
///     .collect_vec();
/// assert_eq!(
///     bss.iter().map(String::as_str).collect_vec(),
///     &[
///         "000000", "0", "00000000", "0", "00000001110000", "", "11111", "0000", "1", "",
///         "011111", "11", "", "", "1", "000", "", "0", "", "0"
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_bool_vecs(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StripedRandomBoolVecs<GeometricRandomNaturalValues<u64>> {
    striped_random_bool_vecs_from_length_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigneds(seed_2, mean_length_numerator, mean_length_denominator)
        },
        mean_stripe_numerator,
        mean_stripe_denominator,
    )
}

/// Generates random striped `Vec<bool>`s, with a minimum length.
///
/// The lengths of the `Vec`s are sampled from a geometric distribution with a specified mean $m$,
/// equal to `mean_length_numerator / mean_length_denominator`. $m$ must be greater than
/// `min_length`.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `mean_stripe_numerator` / `mean_stripe_denominator`. See the module-level documentation.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is
/// `mean_length_numerator` / `mean_length_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator` <= `mean_stripe_denominator`, if `mean_length_numerator` or
/// `mean_length_denominator` are zero, if their ratio is less than or equal to `min_length`, or if
/// they are too large and manipulating them leads to arithmetic overflow.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::random::striped::striped_random_bool_vecs_min_length;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
///
/// let bss = striped_random_bool_vecs_min_length(EXAMPLE_SEED, 10, 1, 3, 5, 1)
///     .take(20)
///     .map(|bs| bs.into_iter().map(|b| if b { '1' } else { '0' }).collect())
///     .collect_vec();
/// assert_eq!(
///     bss.iter().map(String::as_str).collect_vec(),
///     &[
///         "000000000", "0000", "00000000111", "0111", "00000000011111111", "100", "00000111",
///         "1111111", "0001", "111", "111111111", "00000", "000", "000", "1111", "000000", "111",
///         "0011", "000", "1111"
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_bool_vecs_min_length(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    min_length: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StripedRandomBoolVecs<GeometricRandomNaturalValues<u64>> {
    striped_random_bool_vecs_from_length_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigned_inclusive_range(
                seed_2,
                min_length,
                u64::MAX,
                mean_length_numerator,
                mean_length_denominator,
            )
        },
        mean_stripe_numerator,
        mean_stripe_denominator,
    )
}

/// Generates random striped `Vec<bool>`s, with lengths in $[a, b)$.
///
/// The lengths of the `Vec`s are sampled from a uniform distribution on $[a, b)$. $a$ must be less
/// than $b$.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `mean_stripe_numerator` / `mean_stripe_denominator`. See the module-level documentation.
///
/// $$
/// P((x_0, x_1, \ldots, x_{n-1})) = \\begin{cases}
///     \frac{1}{b-a}\prod_{i=0}^{n-1}P(x_i) & a \leq n < b \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// # Expected complexity per iteration
/// $T(n) = O(a + b)$
///
/// $M(n) = O(a + b)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator` <= `mean_stripe_denominator`, or if $a \geq b$.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::random::striped::striped_random_bool_vecs_length_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
///
/// let bss = striped_random_bool_vecs_length_range(EXAMPLE_SEED, 10, 1, 4, 10)
///     .take(20)
///     .map(|bs| bs.into_iter().map(|b| if b { '1' } else { '0' }).collect())
///     .collect_vec();
/// assert_eq!(
///     bss.iter().map(String::as_str).collect_vec(),
///     &[
///         "000000000", "000000000", "000111000", "000000000", "0111", "11111", "00111111",
///         "1000000", "00000011", "111111111", "111111", "00000000", "00000000", "001111",
///         "111111111", "000000000", "110000", "0001111", "0000000", "111101111"
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_bool_vecs_length_range(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    a: u64,
    b: u64,
) -> StripedRandomBoolVecs<RandomUnsignedRange<u64>> {
    striped_random_bool_vecs_from_length_iterator(
        seed,
        &|seed_2| random_unsigned_range(seed_2, a, b),
        mean_stripe_numerator,
        mean_stripe_denominator,
    )
}

/// Generates random striped `Vec<bool>`s, with lengths in $[a, b]$.
///
/// The lengths of the `Vec`s are sampled from a uniform distribution on $[a, b]$. $a$ must be less
/// than $b$.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `mean_stripe_numerator` / `mean_stripe_denominator`. See the module-level documentation.
///
/// $$
/// P((x_0, x_1, \ldots, x_{n-1})) = \\begin{cases}
///     \frac{1}{b-a+1}\prod_{i=0}^{n-1}P(x_i) & a \leq n \leq b \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// # Expected complexity per iteration
/// $T(n) = O(a + b)$
///
/// $M(n) = O(a + b)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator` <= `mean_stripe_denominator`, or if $a \geq b$.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::random::striped::striped_random_bool_vecs_length_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
///
/// let bss = striped_random_bool_vecs_length_inclusive_range(EXAMPLE_SEED, 10, 1, 4, 9)
///     .take(20)
///     .map(|bs| bs.into_iter().map(|b| if b { '1' } else { '0' }).collect())
///     .collect_vec();
/// assert_eq!(
///     bss.iter().map(String::as_str).collect_vec(),
///     &[
///         "000000000", "000000000", "000111000", "000000000", "0111", "11111", "00111111",
///         "1000000", "00000011", "111111111", "111111", "00000000", "00000000", "001111",
///         "111111111", "000000000", "110000", "0001111", "0000000", "111101111"
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_bool_vecs_length_inclusive_range(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    a: u64,
    b: u64,
) -> StripedRandomBoolVecs<RandomUnsignedInclusiveRange<u64>> {
    striped_random_bool_vecs_from_length_iterator(
        seed,
        &|seed_2| random_unsigned_inclusive_range(seed_2, a, b),
        mean_stripe_numerator,
        mean_stripe_denominator,
    )
}

/// Generates a striped unsigned `Vec`, with a given length, from a `StripedBitSource`.
///
/// See the module-level documentation.
///
/// The output length is ceil(`bit_len` / `T::WIDTH`).
///
/// # Expected worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and `n` is `bit_len`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::random::striped::{get_striped_unsigned_vec, StripedBitSource};
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
///
/// let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, 10, 1);
/// let xs = get_striped_unsigned_vec::<u8>(&mut bit_source, 100)
///     .iter().map(u8::to_binary_string).collect_vec();
/// assert_eq!(
///     xs,
///     &[
///         "11111000", "111111", "11100000", "11111111", "111", "11000000", "11111111", "0", "0",
///         "11111000", "11111111", "11111111", "11",
///     ]
/// );
/// ```
pub fn get_striped_unsigned_vec<T: PrimitiveUnsigned>(
    bit_source: &mut StripedBitSource,
    bit_len: u64,
) -> Vec<T> {
    bit_source.end_block();
    bit_source
        .take(usize::exact_from(bit_len))
        .chunks(usize::wrapping_from(T::WIDTH))
        .into_iter()
        .map(T::from_bits_asc)
        .collect()
}

/// Generates random striped `Vec`s of unsigneds.
#[derive(Clone, Debug)]
pub struct StripedRandomUnsignedVecs<T: PrimitiveUnsigned, I: Iterator<Item = u64>> {
    phantom: PhantomData<*const T>,
    lengths: I,
    bit_source: StripedBitSource,
}

impl<T: PrimitiveUnsigned, I: Iterator<Item = u64>> Iterator for StripedRandomUnsignedVecs<T, I> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        Some(get_striped_unsigned_vec(
            &mut self.bit_source,
            self.lengths.next().unwrap() << T::LOG_WIDTH,
        ))
    }
}

/// Generates random striped `Vec`s of unsigneds, with lengths from an iterator.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `mean_stripe_numerator` / `mean_stripe_denominator`. See the module-level documentation.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero or if
/// `mean_stripe_numerator` <= `mean_stripe_denominator`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::random::striped::striped_random_unsigned_vecs_from_length_iterator;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
/// use malachite_base::vecs::random_values_from_vec;
///
/// let xss = striped_random_unsigned_vecs_from_length_iterator::<u8, _>(
///     EXAMPLE_SEED,
///     &|seed| random_values_from_vec(seed, vec![0, 2, 4]),
///     10,
///     1,
/// )
///     .take(10)
///     .map(|xs| xs.into_iter().map(|x: u8| x.to_binary_string()).collect_vec())
///     .collect_vec();
/// let xss = xss
///     .iter()
///     .map(|xs| xs.iter().map(String::as_str).collect_vec())
///     .collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         &["0", "0"][..], &["1110000", "0", "11111100", "11"], &["11111110", "1111"],
///         &["0", "0", "0", "11111000"], &["0", "0", "1111110", "0"], &["11011111", "11111111"],
///         &[], &["11110000", "11111111"], &[], &["11111111", "11000011", "11111", "0"]
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_unsigned_vecs_from_length_iterator<
    T: PrimitiveUnsigned,
    I: Iterator<Item = u64>,
>(
    seed: Seed,
    lengths_gen: &dyn Fn(Seed) -> I,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
) -> StripedRandomUnsignedVecs<T, I> {
    StripedRandomUnsignedVecs {
        phantom: PhantomData,
        lengths: lengths_gen(seed.fork("lengths")),
        bit_source: StripedBitSource::new(
            seed.fork("bit_source"),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
    }
}

/// Generates random striped unsigned `Vec`s of a given length.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `mean_stripe_numerator` / `mean_stripe_denominator`. See the module-level documentation.
///
/// If `len` is 0, the output consists of the empty list, repeated.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::random::striped::striped_random_fixed_length_unsigned_vecs;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
///
/// let xss = striped_random_fixed_length_unsigned_vecs::<u8>(EXAMPLE_SEED, 10, 1, 3)
///     .take(10)
///     .map(|xs| xs.into_iter().map(|x: u8| x.to_binary_string()).collect_vec())
///     .collect_vec();
/// let xss = xss
///     .iter()
///     .map(|xs| xs.iter().map(String::as_str).collect_vec())
///     .collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         &["0", "0", "111000"], &["0", "11111100", "11"], &["11111110", "1111", "0"],
///         &["0", "0", "11111000"], &["0", "0", "1111110"], &["11111111", "11011111", "11111111"],
///         &["11110000", "11111111", "11111111"], &["11000011", "11111", "0"],
///         &["0", "10000000", "11111001"], &["11111111", "0", "0"]
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_fixed_length_unsigned_vecs<T: PrimitiveUnsigned>(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    len: u64,
) -> StripedRandomUnsignedVecs<T, Repeat<u64>> {
    striped_random_unsigned_vecs_from_length_iterator(
        seed,
        &|_| repeat(len),
        mean_stripe_numerator,
        mean_stripe_denominator,
    )
}

/// Generates random striped `Vec`s of unsigneds.
///
/// The lengths of the `Vec`s are sampled from a geometric distribution with a specified mean $m$,
/// equal to `mean_length_numerator / mean_length_denominator`. $m$ must be greater than 0.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `mean_stripe_numerator` / `mean_stripe_denominator`. See the module-level documentation.
///
/// $$
/// P((x_0, x_1, \ldots, x_{n-1})) = \frac{m^n}{(m+1)^{n+1}}\prod_{i=0}^{n-1}P(x_i).
/// $$
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is
/// `mean_length_numerator` / `mean_length_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator` <= `mean_stripe_denominator`, if `mean_length_numerator` or
/// `mean_length_denominator` are zero, or, if after being reduced to lowest terms, their sum is
/// greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::random::striped::striped_random_unsigned_vecs;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
///
/// let xss = striped_random_unsigned_vecs::<u8>(EXAMPLE_SEED, 10, 1, 2, 1)
///     .take(10)
///     .map(|xs| xs.into_iter().map(|x: u8| x.to_binary_string()).collect_vec())
///     .collect_vec();
/// let xss = xss
///     .iter()
///     .map(|xs| xs.iter().map(String::as_str).collect_vec())
///     .collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         &["0", "0", "111000", "0", "11111110", "10000001"][..], &["0"],
///         &["11110000", "11111111", "11111111", "11111111", "11", "0", "10000000", "11111"],
///         &["0"],
///         &[
///             "10000", "0", "11111100", "11111111", "1111111", "11111000", "11", "0", "0",
///             "10011000", "11111111", "111", "0", "0"
///         ], &[], &["11111111", "11111111", "11111111", "11111111", "10111111"],
///         &["0", "0", "0", "11110000"], &["11111111"], &[]
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_unsigned_vecs<T: PrimitiveUnsigned>(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StripedRandomUnsignedVecs<T, GeometricRandomNaturalValues<u64>> {
    striped_random_unsigned_vecs_from_length_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigneds(seed_2, mean_length_numerator, mean_length_denominator)
        },
        mean_stripe_numerator,
        mean_stripe_denominator,
    )
}

/// Generates random striped `Vec`s of unsigneds, with a minimum length.
///
/// The lengths of the `Vec`s are sampled from a geometric distribution with a specified mean $m$,
/// equal to `mean_length_numerator / mean_length_denominator`. $m$ must be greater than
/// `min_length`.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `mean_stripe_numerator` / `mean_stripe_denominator`. See the module-level documentation.
///
/// $$
/// P((x_0, x_1, \ldots, x_{n-1})) = \\begin{cases}
///     \frac{(m-a)^{n-a}}{(m+1-a)^{n+1-a}}\prod_{i=0}^{n-1}P(x_i) & n \geq a \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $a$ is `min_length`.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is
/// `mean_length_numerator` / `mean_length_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator` <= `mean_stripe_denominator`, if `mean_length_numerator` or
/// `mean_length_denominator` are zero, if their ratio is less than or equal to `min_length`, or if
/// they are too large and manipulating them leads to arithmetic overflow.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::random::striped::striped_random_unsigned_vecs_min_length;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
///
/// let xss = striped_random_unsigned_vecs_min_length::<u8>(EXAMPLE_SEED, 10, 1, 2, 3, 1)
///     .take(10)
///     .map(|xs| xs.into_iter().map(|x: u8| x.to_binary_string()).collect_vec())
///     .collect_vec();
/// let xss = xss
///     .iter()
///     .map(|xs| xs.iter().map(String::as_str).collect_vec())
///     .collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         &["0", "0", "111000"][..], &["0", "11111100", "11", "11111111"],
///         &["11110000", "11111111", "11111111", "11111111"],
///         &["11111000", "11111111", "11111111", "11000000"], &["0", "10000", "0"],
///         &["111", "0", "0", "1111"], &["11110000", "11111111"], &["11111111", "111111"],
///         &["110", "10000000", "11111111"], &["11111111", "11111111"]
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_unsigned_vecs_min_length<T: PrimitiveUnsigned>(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    min_length: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StripedRandomUnsignedVecs<T, GeometricRandomNaturalValues<u64>> {
    striped_random_unsigned_vecs_from_length_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigned_inclusive_range(
                seed_2,
                min_length,
                u64::MAX,
                mean_length_numerator,
                mean_length_denominator,
            )
        },
        mean_stripe_numerator,
        mean_stripe_denominator,
    )
}

/// Generates random striped `Vec`s of unsigneds, with lengths in $[a, b)$.
///
/// The lengths of the `Vec`s are sampled from a uniform distribution on $[a, b)$. $a$ must be less
/// than $b$.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `mean_stripe_numerator` / `mean_stripe_denominator`. See the module-level documentation.
///
/// $$
/// P((x_0, x_1, \ldots, x_{n-1})) = \\begin{cases}
///     \frac{1}{b-a}\prod_{i=0}^{n-1}P(x_i) & a \leq n < b \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// # Expected complexity per iteration
/// $T(n) = O(a + b)$
///
/// $M(n) = O(a + b)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator` <= `mean_stripe_denominator`, or if $a \geq b$.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::random::striped::striped_random_unsigned_vecs_length_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
///
/// let xss = striped_random_unsigned_vecs_length_range::<u8>(EXAMPLE_SEED, 10, 1, 2, 4)
///     .take(10)
///     .map(|xs| xs.into_iter().map(|x: u8| x.to_binary_string()).collect_vec())
///     .collect_vec();
/// let xss = xss
///     .iter()
///     .map(|xs| xs.iter().map(String::as_str).collect_vec())
///     .collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         &["0", "0", "111000"][..], &["0", "11111100"], &["11111000", "1", "11110000"],
///         &["0", "0", "0"], &["11110000", "11111111"], &["11111111", "11", "11111111"],
///         &["1000000", "0", "11110000"],
///         &["11111111", "11111111"], &["1111000", "11000000", "11111111"],
///         &["11111111", "11111111", "1100"]
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_unsigned_vecs_length_range<T: PrimitiveUnsigned>(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    a: u64,
    b: u64,
) -> StripedRandomUnsignedVecs<T, RandomUnsignedRange<u64>> {
    striped_random_unsigned_vecs_from_length_iterator(
        seed,
        &|seed_2| random_unsigned_range(seed_2, a, b),
        mean_stripe_numerator,
        mean_stripe_denominator,
    )
}

/// Generates random striped `Vec`s of unsigneds, with lengths in $[a, b]$.
///
/// The lengths of the `Vec`s are sampled from a uniform distribution on $[a, b]$. $a$ must be less
/// than $b$.
///
/// The mean run length (before the bit sequences are truncated) is
/// $m$ = `mean_stripe_numerator` / `mean_stripe_denominator`. See the module-level documentation.
///
/// $$
/// P((x_0, x_1, \ldots, x_{n-1})) = \\begin{cases}
///     \frac{1}{b-a+1}\prod_{i=0}^{n-1}P(x_i) & a \leq n \leq b \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
///
/// # Expected complexity per iteration
/// $T(n) = O(a + b)$
///
/// $M(n) = O(a + b)$
///
/// where $T$ is time and $M$ is additional memory.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator` <= `mean_stripe_denominator`, or if $a \geq b$.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::random::striped::striped_random_unsigned_vecs_length_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::ToBinaryString;
///
/// let xss = striped_random_unsigned_vecs_length_inclusive_range::<u8>(EXAMPLE_SEED, 10, 1, 2, 3)
///     .take(10)
///     .map(|xs| xs.into_iter().map(|x: u8| x.to_binary_string()).collect_vec())
///     .collect_vec();
/// let xss = xss
///     .iter()
///     .map(|xs| xs.iter().map(String::as_str).collect_vec())
///     .collect_vec();
/// assert_eq!(
///     xss,
///     &[
///         &["0", "0", "111000"][..], &["0", "11111100"], &["11111000", "1", "11110000"],
///         &["0", "0", "0"], &["11110000", "11111111"], &["11111111", "11", "11111111"],
///         &["1000000", "0", "11110000"],
///         &["11111111", "11111111"], &["1111000", "11000000", "11111111"],
///         &["11111111", "11111111", "1100"]
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_unsigned_vecs_length_inclusive_range<T: PrimitiveUnsigned>(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    a: u64,
    b: u64,
) -> StripedRandomUnsignedVecs<T, RandomUnsignedInclusiveRange<u64>> {
    striped_random_unsigned_vecs_from_length_iterator(
        seed,
        &|seed_2| random_unsigned_inclusive_range(seed_2, a, b),
        mean_stripe_numerator,
        mean_stripe_denominator,
    )
}
