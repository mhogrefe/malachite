use std::marker::PhantomData;

use bools::random::{random_bools, weighted_random_bools, RandomBools, WeightedRandomBools};
use iterators::{nonzero_values, NonzeroValues};
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::random::geometric::mean_to_p_with_min;
use random::Seed;

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

impl StripedBitSource {
    /// Creates a new `StripedBitSource` with mean run length $m$, where $m$ is
    /// `m_numerator` / `m_denominator`.
    ///
    /// # Expected worst-case complexity
    ///
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
        assert!(m_numerator > m_denominator);
        let (numerator, denominator) = mean_to_p_with_min(1u64, m_numerator, m_denominator);
        StripedBitSource {
            first_bit_of_block: true,
            previous_bit: false,
            bs: random_bools(seed.fork("bs")),
            xs: weighted_random_bools(seed.fork("xs"), numerator, denominator),
        }
    }

    /// Gets a bit from this `StripedBitSource`. If this function is being called for the first
    /// time, the probabilities of a `true` or a `false` are equal. On subsequent calls, the
    /// probability of getting a bit different from the previous one is $1 / m$.
    ///
    /// To reset the bit source, so that the next call to `get` has equal probabilities of `true` or
    /// `false`, call `end_block`.
    ///
    /// # Expected worst-case complexity
    ///
    /// Constant time and additional memory.
    pub fn get(&mut self) -> bool {
        self.previous_bit = if self.first_bit_of_block {
            self.first_bit_of_block = false;
            self.bs.next().unwrap()
        } else {
            self.previous_bit ^ self.xs.next().unwrap()
        };
        self.previous_bit
    }

    /// Resets this `StripedBitSource`, so that the next time `get` is called, the probabilities of
    /// `true` or `false` will be equal.
    ///
    /// # Expected worst-case complexity
    ///
    /// Constant time and additional memory.
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
    phantom_data: PhantomData<*const T>,
    bits: StripedBitSource,
    chunk_size: u64,
}

impl<T: PrimitiveUnsigned> Iterator for StripedRandomUnsignedBitChunks<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.bits.end_block();
        let mut x = T::ZERO;
        for _ in 0..self.chunk_size {
            x <<= 1;
            if self.bits.get() {
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
    phantom_data: PhantomData<*const T>,
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

/// Generates random natural (non-negative) signed integers from a random striped distribution.
///
/// This `struct` is created by the `striped_random_natural_signeds` function. See its documentation
/// for more.
#[derive(Clone, Debug)]
pub struct StripedRandomNaturalSigneds<T: PrimitiveSigned> {
    phantom_data: PhantomData<*const T>,
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

/// Generates random negative signed integers from a random striped distribution.
///
/// This `struct` is created by the `striped_random_negative_signeds` function. See its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct StripedRandomNegativeSigneds<T: PrimitiveSigned> {
    phantom_data: PhantomData<*const T>,
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

/// Generates random unsigned integers less than a positive limit, from a random striped
/// distribution.
///
/// This `struct` is created by the `striped_random_unsigneds_less_than` function. See its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct StripedRandomUnsignedsLessThan<T: PrimitiveUnsigned> {
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
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_unsigneds;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_unsigneds::<u8>(EXAMPLE_SEED, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect::<Vec<_>>(),
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
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_positive_unsigneds;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_positive_unsigneds::<u8>(EXAMPLE_SEED, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect::<Vec<_>>(),
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
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_signeds;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_signeds::<i8>(EXAMPLE_SEED, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect::<Vec<_>>(),
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
        phantom_data: PhantomData,
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
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_natural_signeds;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_natural_signeds::<i8>(EXAMPLE_SEED, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect::<Vec<_>>(),
///     &["0", "101100", "110000", "1111100", "1111", "1111110", "0", "111", "11101", "1100000"]
/// )
/// ```
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
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_positive_signeds;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_positive_signeds::<i8>(EXAMPLE_SEED, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect::<Vec<_>>(),
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
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_negative_signeds;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_negative_signeds::<i8>(EXAMPLE_SEED, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect::<Vec<_>>(),
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
        phantom_data: PhantomData,
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
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_nonzero_signeds;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_nonzero_signeds::<i8>(EXAMPLE_SEED, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect::<Vec<_>>(),
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

///
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
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::striped::striped_random_unsigned_bit_chunks;
/// use malachite_base::strings::ToBinaryString;
///
/// assert_eq!(
///     striped_random_unsigned_bit_chunks::<u8>(EXAMPLE_SEED, 3, 4, 1).take(10)
///         .map(|x| x.to_binary_string()).collect::<Vec<_>>(),
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
        phantom_data: PhantomData,
        bits: StripedBitSource::new(seed, m_numerator, m_denominator),
        chunk_size,
    }
}
