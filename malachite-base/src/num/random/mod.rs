use iterators::{nonzero_values, NonzeroValues};
use num::basic::integers::PrimitiveInteger;
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::random::random_bit_chunks::{RandomSignedBitChunks, RandomUnsignedBitChunks};
use num::random::random_highest_bit_set_values::RandomHighestBitSetValues;
use num::random::random_unsigneds_less_than::RandomUnsignedsLessThan;
use num::random::thrifty_random::RandomPrimitiveIntegers;
use random::seed::Seed;

/// Generates the an iterator's values but with all but the lowest `pow` bits cleared.
///
/// Time per iteration: O(1)
///
/// Additional memory per iteration: O(1)
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_unsigned_bit_chunks;
///
/// assert_eq!(
///     random_unsigned_bit_chunks::<u8>(EXAMPLE_SEED, 3).take(10).collect::<Vec<u8>>(),
///     &[1, 6, 5, 7, 6, 3, 1, 2, 4, 5]
/// )
/// ```
pub fn random_unsigned_bit_chunks<T: PrimitiveUnsigned>(
    seed: Seed,
    chunk_size: u64,
) -> RandomUnsignedBitChunks<T> {
    RandomUnsignedBitChunks {
        xs: random_primitive_integers(seed),
        x: T::ZERO,
        bits_left: 0,
        chunk_size,
        mask: T::low_mask(chunk_size),
        high_bits: None,
    }
}

pub fn random_signed_bit_chunks<T: PrimitiveSigned>(
    seed: Seed,
    chunk_size: u64,
) -> RandomSignedBitChunks<T> {
    RandomSignedBitChunks {
        xs: T::new_absolute_chunks(seed, chunk_size),
    }
}

/// Generates the an iterator's values but with the highest bit set.
///
/// Time per iteration: O(1)
///
/// Additional memory per iteration: O(1)
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_highest_bit_set_unsigneds;
///
/// assert_eq!(
///     random_highest_bit_set_unsigneds::<u8>(EXAMPLE_SEED).take(10).collect::<Vec<u8>>(),
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

/// Generates random values from a distribution defined by `T`'s `StandardRand` trait.
///
/// Length is infinite.
///
/// Time per iteration: worst case time for `<T as StandardRand>::gen::<T>(ChaCha20Rng)`
///
/// Additional memory per iteration: worst case additional memory for
/// `<T as StandardRand>::gen::<T>(ChaCha20Rng)`
///
/// # Examples
/// ```
/// use malachite_base::num::random::random_primitive_integers;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_primitive_integers::<u8>(EXAMPLE_SEED).take(10).collect::<Vec<u8>>(),
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

/// Generates random positive unsigned integers from a uniform distribution across all possible
/// values.
///
/// Length is infinite.
///
/// Time per iteration: O(1)
///
/// Additional memory per iteration: O(1)
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_positive_unsigneds;
///
/// assert_eq!(
///     random_positive_unsigneds::<u8>(EXAMPLE_SEED).take(10).collect::<Vec<u8>>(),
///     &[113, 239, 69, 108, 228, 210, 168, 161, 87, 32]
/// )
/// ```
#[inline]
pub fn random_positive_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
) -> NonzeroValues<RandomPrimitiveIntegers<T>> {
    nonzero_values(random_primitive_integers(seed))
}

/// Generates random positive signed integers from a uniform distribution across all possible
/// values.
///
/// Length is infinite.
///
/// Time per iteration: O(1)
///
/// Additional memory per iteration: O(1)
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_positive_signeds;
///
/// assert_eq!(
///     random_positive_signeds::<i8>(EXAMPLE_SEED).take(10).collect::<Vec<i8>>(),
///     &[113, 94, 23, 98, 70, 92, 52, 84, 33, 47]
/// )
/// ```
#[inline]
pub fn random_positive_signeds<T: PrimitiveSigned>(
    seed: Seed,
) -> NonzeroValues<RandomSignedBitChunks<T>> {
    nonzero_values(random_natural_signeds(seed))
}

/// Generates random negative signed integers from a uniform distribution across all possible
/// values.
///
/// Length is infinite.
///
/// Time per iteration: O(1)
///
/// Additional memory per iteration: O(1)
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_negative_signeds;
///
/// assert_eq!(
///     random_negative_signeds::<i8>(EXAMPLE_SEED).take(10).collect::<Vec<i8>>(),
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

/// Generates random natural (i.e. non-negative) signed integers from a uniform distribution across
/// all possible values.
///
/// Length is infinite.
///
/// Time per iteration: O(1)
///
/// Additional memory per iteration: O(1)
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_natural_signeds;
///
/// assert_eq!(
///     random_natural_signeds::<i8>(EXAMPLE_SEED).take(10).collect::<Vec<i8>>(),
///     &[113, 94, 23, 98, 70, 92, 52, 84, 33, 47]
/// )
/// ```
#[inline]
pub fn random_natural_signeds<T: PrimitiveSigned>(seed: Seed) -> RandomSignedBitChunks<T> {
    random_signed_bit_chunks(seed, T::WIDTH - 1)
}

/// Generates random nonzero signed integers from a uniform distribution across all possible values.
///
/// Length is infinite.
///
/// Time per iteration: O(1)
///
/// Additional memory per iteration: O(1)
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::random_nonzero_signeds;
///
/// assert_eq!(
///     random_nonzero_signeds::<i8>(EXAMPLE_SEED).take(10).collect::<Vec<i8>>(),
///     &[113, -17, 69, 108, -28, -46, -88, -95, 87, 32]
/// )
/// ```
#[inline]
pub fn random_nonzero_signeds<T: PrimitiveSigned>(
    seed: Seed,
) -> NonzeroValues<RandomPrimitiveIntegers<T>> {
    nonzero_values(random_primitive_integers(seed))
}

#[inline]
pub fn random_unsigneds_less_than<T: PrimitiveUnsigned>(
    seed: Seed,
    limit: T,
) -> RandomUnsignedsLessThan<T> {
    let chunk_size = if limit == T::ZERO {
        T::WIDTH
    } else {
        limit.ceiling_log_two()
    };
    RandomUnsignedsLessThan {
        xs: random_unsigned_bit_chunks(seed, chunk_size),
        limit,
    }
}

pub mod geometric;
pub mod random_bit_chunks;
pub mod random_highest_bit_set_values;
pub mod random_unsigneds_less_than;
pub mod thrifty_random;
