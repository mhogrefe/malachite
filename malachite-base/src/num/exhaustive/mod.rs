use exhaustive::range::{range_decreasing, range_increasing, RangeDecreasing, RangeIncreasing};
use itertools::{Interleave, Itertools};
use num::basic::integers::PrimitiveInteger;
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use std::iter::{once, Chain, Once};

/// Generates all positive values of a primitive integer type, in ascending order.
///
/// Length is 2 ^ `T::WIDTH` - 1 if `T` is unsigned, and 2 ^ (`T::WIDTH` - 1) - 1 if `T` is signed.
///
/// Time: worst case O(1) per iteration
///
/// Additional memory: worst case O(1) per iteration
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_positive_primitives;
///
/// assert_eq!(
///     exhaustive_positive_primitives::<u8>().take(10).collect::<Vec<u8>>(),
///     &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
/// )
/// ```
#[inline]
pub fn exhaustive_positive_primitives<T: PrimitiveInteger>() -> RangeIncreasing<T> {
    range_increasing(T::ONE, T::MAX)
}

/// Generates all values of an unsigned integer type, in ascending order.
///
/// Length is 2 ^ `T::WIDTH`.
///
/// Time: worst case O(1) per iteration
///
/// Additional memory: worst case O(1) per iteration
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
///
/// assert_eq!(
///     exhaustive_unsigneds::<u8>().take(10).collect::<Vec<u8>>(),
///     &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
/// )
/// ```
#[inline]
pub fn exhaustive_unsigneds<T: PrimitiveUnsigned>() -> RangeIncreasing<T> {
    range_increasing(T::ZERO, T::MAX)
}

/// Generates all negative values of a signed integer type, in descending order.
///
/// Length is 2 ^ (`T::WIDTH` - 1).
///
/// Time: worst case O(1) per iteration
///
/// Additional memory: worst case O(1) per iteration
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_negative_signeds;
///
/// assert_eq!(
///     exhaustive_negative_signeds::<i8>().take(10).collect::<Vec<i8>>(),
///     &[-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]
/// )
/// ```
#[inline]
pub fn exhaustive_negative_signeds<T: PrimitiveSigned>() -> RangeDecreasing<T> {
    range_decreasing(T::MIN, T::NEGATIVE_ONE)
}

/// Generates all natural values of a signed integer type, in ascending order.
///
/// Length is 2 ^ (`T::WIDTH` - 1).
///
/// Time: worst case O(1) per iteration
///
/// Additional memory: worst case O(1) per iteration
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_natural_signeds;
///
/// assert_eq!(
///     exhaustive_natural_signeds::<i8>().take(10).collect::<Vec<i8>>(),
///     &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
/// )
/// ```
#[inline]
pub fn exhaustive_natural_signeds<T: PrimitiveSigned>() -> RangeIncreasing<T> {
    range_increasing(T::ZERO, T::MAX)
}

pub type UpDown<T> = Interleave<RangeIncreasing<T>, RangeDecreasing<T>>;

/// Generates all nonzero values of a signed integer type, in order of ascending absolute value.
/// When two numbers have the same absolute value, the positive one comes first.
///
/// Length is 2 ^ `T::WIDTH` - 1.
///
/// Time: worst case O(1) per iteration
///
/// Additional memory: worst case O(1) per iteration
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_nonzero_signeds;
///
/// assert_eq!(
///     exhaustive_nonzero_signeds::<i8>().take(10).collect::<Vec<i8>>(),
///     &[1, -1, 2, -2, 3, -3, 4, -4, 5, -5]
/// )
/// ```
#[inline]
pub fn exhaustive_nonzero_signeds<T: PrimitiveSigned>() -> UpDown<T> {
    exhaustive_positive_primitives().interleave(exhaustive_negative_signeds())
}

/// Generates all values of a signed integer type, in order of ascending absolute value. When two
/// numbers have the same absolute value, the positive one comes first.
///
/// Length is 2 ^ `T::WIDTH`.
///
/// Time: worst case O(1) per iteration
///
/// Additional memory: worst case O(1) per iteration
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_signeds;
///
/// assert_eq!(
///     exhaustive_signeds::<i8>().take(10).collect::<Vec<i8>>(),
///     &[0, 1, -1, 2, -2, 3, -3, 4, -4, 5]
/// )
/// ```
#[inline]
pub fn exhaustive_signeds<T: PrimitiveSigned>() -> Chain<Once<T>, UpDown<T>> {
    once(T::ZERO).chain(exhaustive_nonzero_signeds())
}
