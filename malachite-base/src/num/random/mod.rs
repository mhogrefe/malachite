use iterators::{nonzero_values, NonzeroValues};
use num::basic::integers::PrimitiveInteger;
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::random::random_bit_chunks::{RandomSignedBitChunks, RandomUnsignedBitChunks};
use num::random::random_highest_bit_set_values::RandomHighestBitSetValues;
use num::random::random_primitive_integers::RandomPrimitiveIntegers;
use num::random::random_signed_range::{RandomSignedInclusiveRange, RandomSignedRange};
use num::random::random_unsigned_range::{RandomUnsignedInclusiveRange, RandomUnsignedRange};
use num::random::random_unsigneds_less_than::RandomUnsignedsLessThan;
use random::seed::Seed;

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
/// $T(i) = \mathcal{O}(1)$
///
/// $M(i) = \mathcal{O}(1)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Panics
/// Panics if `chunk_size` is greater than `T::WIDTH`.
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
    assert!(chunk_size <= T::WIDTH);
    RandomUnsignedBitChunks {
        xs: random_primitive_integers(seed),
        x: T::ZERO,
        bits_left: 0,
        chunk_size,
        mask: T::low_mask(chunk_size),
        high_bits: None,
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
/// $T(i) = \mathcal{O}(1)$
///
/// $M(i) = \mathcal{O}(1)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Panics
/// Panics if `chunk_size` is greater than `T::WIDTH`.
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
/// $T(i) = \mathcal{O}(1)$
///
/// $M(i) = \mathcal{O}(1)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
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

/// Uniformly generates random primitive integers.
///
/// $P(x) = 2^{-W}$, where $W$ is `T::WIDTH`.
///
/// The output length is infinite.
///
/// # Complexity per iteration
/// $T(i) = \mathcal{O}(1)$
///
/// $M(i) = \mathcal{O}(1)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Examples
/// ```
/// use malachite_base::num::random::random_primitive_integers;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_primitive_integers::<u8>(EXAMPLE_SEED).take(10).collect::<Vec<_>>(),
///     &[113, 239, 69, 108, 228, 210, 168, 161, 87, 32]
/// )
/// ```
#[inline]
pub fn random_primitive_integers<T: PrimitiveInteger>(seed: Seed) -> RandomPrimitiveIntegers<T> {
    RandomPrimitiveIntegers {
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
/// $E[T(i)] = \mathcal{O}(1)$
///
/// $E[M(i)] = \mathcal{O}(1)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
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
) -> NonzeroValues<RandomPrimitiveIntegers<T>> {
    nonzero_values(random_primitive_integers(seed))
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
/// $E[T(i)] = \mathcal{O}(1)$
///
/// $E[M(i)] = \mathcal{O}(1)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
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
/// $T(i) = \mathcal{O}(1)$
///
/// $M(i) = \mathcal{O}(1)$
///
/// where $i$ is the iteration number.
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
/// $T(i) = \mathcal{O}(1)$
///
/// $M(i) = \mathcal{O}(1)$
///
/// where $i$ is the iteration number.
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
/// $T(i) = \mathcal{O}(1)$
///
/// $M(i) = \mathcal{O}(1)$
///
/// where $i$ is the iteration number.
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
) -> NonzeroValues<RandomPrimitiveIntegers<T>> {
    nonzero_values(random_primitive_integers(seed))
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
/// $E[T(i)] = \mathcal{O}(1)$
///
/// $E[M(i)] = \mathcal{O}(1)$
///
/// where $i$ is the iteration number.
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
    }
    RandomUnsignedsLessThan {
        xs: random_unsigned_bit_chunks(seed, limit.ceiling_log_two()),
        limit,
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
/// $E[T(i)] = \mathcal{O}(1)$
///
/// $E[M(i)] = \mathcal{O}(1)$
///
/// where $i$ is the iteration number.
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
/// $E[T(i)] = \mathcal{O}(1)$
///
/// $E[M(i)] = \mathcal{O}(1)$
///
/// where $i$ is the iteration number.
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
        RandomUnsignedInclusiveRange::All(random_primitive_integers(seed))
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
/// $E[T(i)] = \mathcal{O}(1)$
///
/// $E[M(i)] = \mathcal{O}(1)$
///
/// where $i$ is the iteration number.
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
/// $E[T(i)] = \mathcal{O}(1)$
///
/// $E[M(i)] = \mathcal{O}(1)$
///
/// where $i$ is the iteration number.
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

pub mod geometric;
pub mod random_bit_chunks;
pub mod random_highest_bit_set_values;
pub mod random_primitive_integers;
pub mod random_signed_range;
pub mod random_unsigned_range;
pub mod random_unsigneds_less_than;
pub mod striped;
