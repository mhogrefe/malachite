use iterators::{nonzero_values, NonzeroValues};
use num::basic::integers::PrimitiveInteger;
use num::basic::signeds::PrimitiveSigned;
use num::basic::traits::Zero;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::logic::traits::{BitAccess, LowMask};
use random::{standard_random_values, Seed, StandardRandomValues};

pub mod standard_rand;

/// Generates the an iterator's values but with all but the lowest `pow` bits cleared.
#[derive(Clone, Debug)]
pub struct RandomMaskedValues<I: Iterator>
where
    I::Item: PrimitiveInteger,
{
    xs: I,
    mask: I::Item,
}

impl<I: Iterator> Iterator for RandomMaskedValues<I>
where
    I::Item: PrimitiveInteger,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        self.xs.next().map(|x| x & self.mask)
    }
}

/// Generates the an iterator's values but with all but the lowest `pow` bits cleared.
///
/// Time per iteration: O(1)
///
/// Additional memory per iteration: O(1)
///
/// # Examples
/// ```
/// use malachite_base::random::{EXAMPLE_SEED, standard_random_values};
/// use malachite_base::num::random::random_masked_values;
///
/// assert_eq!(
///     random_masked_values(standard_random_values::<u8>(EXAMPLE_SEED), 3)
///         .take(10).collect::<Vec<u8>>(),
///     &[1, 4, 7, 4, 5, 5, 5, 7, 7, 0]
/// )
/// ```
#[inline]
pub fn random_masked_values<I: Iterator>(xs: I, pow: u64) -> RandomMaskedValues<I>
where
    I::Item: PrimitiveInteger,
{
    RandomMaskedValues {
        xs,
        mask: I::Item::low_mask(pow),
    }
}

/// Generates the an iterator's values but with the highest bit set.
#[derive(Clone, Debug)]
pub struct RandomHighestBitSetValues<I: Iterator>
where
    I::Item: PrimitiveInteger,
{
    xs: I,
    mask: I::Item,
}

impl<I: Iterator> Iterator for RandomHighestBitSetValues<I>
where
    I::Item: PrimitiveInteger,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        self.xs.next().map(|x| x | self.mask)
    }
}
/// Generates the an iterator's values but with the highest bit set.
///
/// Assuming `xs` takes constant time and memory per iteration, as it does for all
/// `PrimitiveInteger` implementations provided by Malachite:
///
/// Time per iteration: O(1)
///
/// Additional memory per iteration: O(1)
///
/// # Examples
/// ```
/// use malachite_base::random::{EXAMPLE_SEED, standard_random_values};
/// use malachite_base::num::random::random_highest_bit_set_values;
///
/// assert_eq!(
///     random_highest_bit_set_values(standard_random_values::<u8>(EXAMPLE_SEED))
///         .take(10).collect::<Vec<u8>>(),
///     &[241, 228, 215, 188, 221, 189, 245, 151, 135, 200],
/// )
/// ```
#[inline]
pub fn random_highest_bit_set_values<I: Iterator>(xs: I) -> RandomHighestBitSetValues<I>
where
    I::Item: PrimitiveInteger,
{
    let mut mask = I::Item::ZERO;
    mask.set_bit(I::Item::WIDTH - 1);
    RandomHighestBitSetValues { xs, mask }
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
///     &[113, 228, 87, 188, 93, 189, 117, 151, 7, 72]
/// )
/// ```
#[inline]
pub fn random_positive_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
) -> NonzeroValues<StandardRandomValues<T>> {
    nonzero_values(standard_random_values(seed))
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
///     &[113, 100, 87, 60, 93, 61, 117, 23, 7, 72]
/// )
/// ```
#[inline]
pub fn random_positive_signeds<T: PrimitiveSigned>(
    seed: Seed,
) -> NonzeroValues<RandomMaskedValues<StandardRandomValues<T>>> {
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
///     &[-15, -28, -41, -68, -35, -67, -11, -105, -121, -56]
/// )
/// ```
#[inline]
pub fn random_negative_signeds<T: PrimitiveSigned>(
    seed: Seed,
) -> RandomHighestBitSetValues<StandardRandomValues<T>> {
    random_highest_bit_set_values(standard_random_values(seed))
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
///     &[113, 100, 87, 60, 93, 61, 117, 23, 7, 72]
/// )
/// ```
#[inline]
pub fn random_natural_signeds<T: PrimitiveSigned>(
    seed: Seed,
) -> RandomMaskedValues<StandardRandomValues<T>> {
    random_masked_values(standard_random_values(seed), T::WIDTH - 1)
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
///     &[113, -28, 87, -68, 93, -67, 117, -105, 7, 72]
/// )
/// ```
#[inline]
pub fn random_nonzero_signeds<T: PrimitiveSigned>(
    seed: Seed,
) -> NonzeroValues<StandardRandomValues<T>> {
    nonzero_values(standard_random_values(seed))
}
