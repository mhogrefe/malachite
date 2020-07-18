use std::marker::PhantomData;

use bools::random::{random_bools, RandomBools};
use iterators::{nonzero_values, NonzeroValues};
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::random::geometric::mean_to_p_with_min;
use num::random::random_unsigneds_less_than;
use num::random::random_unsigneds_less_than::RandomUnsignedsLessThan;
use random::seed::Seed;

/// A `StripedBitSource` generates bits in a way that lets you control the mean length of a run (a
/// consecutive series of 0s or 1s). If the mean is set to 2, the bits are generated as if by fair
/// coin flips. If the mean is greater than 2, each bit is more likely to be the same as the
/// previous bit, resulting in long runs of bits (or stripes). If the mean is less than 2, each bit
/// is more likely to be different from the previous bit, resulting in a sequence that prefers to
/// alternate between 0 and 1. The mean must be at least 1; if it is exactly 1, then the sequence is
/// 010101... or 101010... with equal probability.
///
/// The first bit is 0 or 1 with equal probability.
///
/// This class is useful for generating numbers for testing arithmetic functions; they are likely to
/// trigger carries.
#[derive(Clone, Debug)]
pub struct StripedBitSource {
    first_bit_of_block: bool,
    previous_bit: bool,
    bs: RandomBools,
    xs: RandomUnsignedsLessThan<u64>,
    numerator: u64,
}

impl StripedBitSource {
    /// Creates a new `StripedBitSource` with mean run length `m_numerator` / `m_denominator`.
    ///
    /// Time: O(1)
    ///
    /// Additional memory: O(1)
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
    /// for _ in 0..40 {
    ///     if bit_source.get() {
    ///         string.push('1');
    ///     } else {
    ///         string.push('0');
    ///     }
    /// }
    /// assert_eq!(string, "0000000101100110000000011110000000001111");
    /// ```
    pub fn new(seed: Seed, m_numerator: u64, m_denominator: u64) -> StripedBitSource {
        assert_ne!(m_denominator, 0);
        assert!(m_numerator >= m_denominator);
        let (numerator, denominator) = mean_to_p_with_min(m_numerator, m_denominator, 1u64);
        StripedBitSource {
            first_bit_of_block: true,
            previous_bit: false,
            bs: random_bools(seed.fork("bs")),
            xs: random_unsigneds_less_than(seed.fork("xs"), denominator),
            numerator,
        }
    }

    /// Gets a bit from this `StripedBitSource`. If this function is being called for the first
    /// time, the probabilities of a `true` or a `false` are equal. On subsequent calls, the
    /// probability of getting a bit different from the previous one is 1 / m.
    ///
    /// To reset the bit source, so that the next call to `get` has equal probabilities of `true` or
    /// `false`, call `end_block`.
    ///
    /// Time: O(1)
    ///
    /// Additional memory: O(1)
    pub fn get(&mut self) -> bool {
        self.previous_bit = if self.first_bit_of_block {
            self.first_bit_of_block = false;
            self.bs.next().unwrap()
        } else {
            self.previous_bit ^ (self.xs.next().unwrap() < self.numerator)
        };
        self.previous_bit
    }

    /// Resets this `StripedBitSource`, so that the next time `get` is called, the probabilities of
    /// `true` or `false` will be equal.
    ///
    /// Time: O(1)
    ///
    /// Additional memory: O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::random::EXAMPLE_SEED;
    /// use malachite_base::num::random::striped::StripedBitSource;
    ///
    /// fn generate_string(bit_source: &mut StripedBitSource) -> String {
    ///     let mut string = String::with_capacity(40);
    ///     for _ in 0..40 {
    ///         if bit_source.get() {
    ///             string.push('1');
    ///         } else {
    ///             string.push('0');
    ///         }
    ///     }
    ///     string
    /// }
    ///
    /// let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, 1_000_000, 1);
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

#[derive(Clone, Debug)]
pub struct StripedRandomUnsigneds<T: PrimitiveUnsigned> {
    phantom_data: PhantomData<T>,
    bits: StripedBitSource,
}

impl<T: PrimitiveUnsigned> Iterator for StripedRandomUnsigneds<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.bits.end_block();
        let mut x = T::ZERO;
        for _ in 0..T::WIDTH {
            x <<= 1;
            if self.bits.get() {
                x |= T::ONE;
            }
        }
        Some(x)
    }
}

#[derive(Clone, Debug)]
pub struct StripedRandomSigneds<T: PrimitiveSigned> {
    phantom_data: PhantomData<T>,
    bits: StripedBitSource,
    bs: RandomBools,
}

impl<T: PrimitiveSigned> Iterator for StripedRandomSigneds<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.bits.end_block();
        let mut x = T::ZERO;
        for _ in 0..T::WIDTH - 1 {
            x <<= 1;
            if self.bits.get() {
                x |= T::ONE;
            }
        }
        if self.bs.next().unwrap() {
            x.set_bit(T::WIDTH - 1);
        }
        Some(x)
    }
}

#[derive(Clone, Debug)]
pub struct StripedRandomNaturalSigneds<T: PrimitiveSigned> {
    phantom_data: PhantomData<T>,
    bits: StripedBitSource,
}

impl<T: PrimitiveSigned> Iterator for StripedRandomNaturalSigneds<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.bits.end_block();
        let mut x = T::ZERO;
        for _ in 0..T::WIDTH - 1 {
            x <<= 1;
            if self.bits.get() {
                x |= T::ONE;
            }
        }
        Some(x)
    }
}

#[derive(Clone, Debug)]
pub struct StripedRandomNegativeSigneds<T: PrimitiveSigned> {
    phantom_data: PhantomData<T>,
    bits: StripedBitSource,
}

impl<T: PrimitiveSigned> Iterator for StripedRandomNegativeSigneds<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.bits.end_block();
        let mut x = T::ZERO;
        for _ in 0..T::WIDTH - 1 {
            x <<= 1;
            if self.bits.get() {
                x |= T::ONE;
            }
        }
        x.set_bit(T::WIDTH - 1);
        Some(x)
    }
}

pub fn striped_random_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
    m_numerator: u64,
    m_denominator: u64,
) -> StripedRandomUnsigneds<T> {
    StripedRandomUnsigneds {
        phantom_data: PhantomData,
        bits: StripedBitSource::new(seed, m_numerator, m_denominator),
    }
}

pub fn striped_random_positive_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
    m_numerator: u64,
    m_denominator: u64,
) -> NonzeroValues<StripedRandomUnsigneds<T>> {
    nonzero_values(striped_random_unsigneds(seed, m_numerator, m_denominator))
}

pub fn striped_random_signeds<T: PrimitiveSigned>(
    seed: Seed,
    m_numerator: u64,
    m_denominator: u64,
) -> StripedRandomSigneds<T> {
    StripedRandomSigneds {
        phantom_data: PhantomData,
        bits: StripedBitSource::new(seed.fork("bits"), m_numerator, m_denominator),
        bs: random_bools(seed.fork("bs")),
    }
}

pub fn striped_random_natural_signeds<T: PrimitiveSigned>(
    seed: Seed,
    m_numerator: u64,
    m_denominator: u64,
) -> StripedRandomNaturalSigneds<T> {
    StripedRandomNaturalSigneds {
        phantom_data: PhantomData,
        bits: StripedBitSource::new(seed, m_numerator, m_denominator),
    }
}

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

pub fn striped_random_negative_signeds<T: PrimitiveSigned>(
    seed: Seed,
    m_numerator: u64,
    m_denominator: u64,
) -> StripedRandomNegativeSigneds<T> {
    StripedRandomNegativeSigneds {
        phantom_data: PhantomData,
        bits: StripedBitSource::new(seed, m_numerator, m_denominator),
    }
}

pub fn striped_random_nonzero_signeds<T: PrimitiveSigned>(
    seed: Seed,
    m_numerator: u64,
    m_denominator: u64,
) -> NonzeroValues<StripedRandomSigneds<T>> {
    nonzero_values(striped_random_signeds(seed, m_numerator, m_denominator))
}
