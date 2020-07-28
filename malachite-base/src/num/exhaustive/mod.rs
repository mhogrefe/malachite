use std::iter::Rev;
use std::iter::{once, Chain, Once};

use itertools::{Interleave, Itertools};

use num::basic::integers::PrimitiveInteger;
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;

/// Generates all values of a primitive integer type in an interval, in ascending order.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PrimitiveIntegerIncreasingRange<T: PrimitiveInteger> {
    a: Option<T>,
    b: Option<T>,
}

impl<T: PrimitiveInteger> Iterator for PrimitiveIntegerIncreasingRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.a == self.b {
            None
        } else {
            let result = self.a;
            self.a = result.and_then(|x| x.checked_add(T::ONE));
            result
        }
    }
}

impl<T: PrimitiveInteger> DoubleEndedIterator for PrimitiveIntegerIncreasingRange<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.a == self.b {
            None
        } else {
            self.b = Some(if let Some(b) = self.b {
                b - T::ONE
            } else {
                T::MAX
            });
            self.b
        }
    }
}

/// Generates all values of a primitive integer type in the half-open interval [`a`, `b`), in
/// ascending order. `a` must be less than or equal to `b`. If `a` and `b` are equal, the range is
/// empty. This function cannot create a range that includes `T::MAX`; for that, use
/// `primitive_integer_increasing_range_to_max`.
///
/// Length is `b` - `a`.
///
/// Time: worst case O(1) per iteration
///
/// Additional memory: worst case O(1) per iteration
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::primitive_integer_increasing_range;
///
/// assert_eq!(
///     primitive_integer_increasing_range::<i8>(-5, 5).collect::<Vec<_>>(),
///     &[-5, -4, -3, -2, -1, 0, 1, 2, 3, 4]
/// )
/// ```
#[inline]
pub fn primitive_integer_increasing_range<T: PrimitiveInteger>(
    a: T,
    b: T,
) -> PrimitiveIntegerIncreasingRange<T> {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    PrimitiveIntegerIncreasingRange {
        a: Some(a),
        b: Some(b),
    }
}

/// Generates all values of a primitive integer type in the closed interval [`a`, `T::MAX`], in
/// ascending order.
///
/// Length is 2<sup>`T::WIDTH`</sup> - `a`.
///
/// Time: worst case O(1) per iteration
///
/// Additional memory: worst case O(1) per iteration
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::primitive_integer_increasing_range_to_max;
///
/// assert_eq!(
///     primitive_integer_increasing_range_to_max::<i8>(120).collect::<Vec<_>>(),
///     &[120, 121, 122, 123, 124, 125, 126, 127]
/// )
/// ```
#[inline]
pub fn primitive_integer_increasing_range_to_max<T: PrimitiveInteger>(
    a: T,
) -> PrimitiveIntegerIncreasingRange<T> {
    PrimitiveIntegerIncreasingRange {
        a: Some(a),
        b: None,
    }
}

/// Generates all values of a signed integer type in an interval, in order of ascending absolute
/// value. When two numbers have the same absolute value, the positive one comes first.
#[derive(Clone, Debug)]
pub enum ExhaustiveSignedRange<T: PrimitiveSigned> {
    NonNegative(PrimitiveIntegerIncreasingRange<T>),
    NonPositive(Rev<PrimitiveIntegerIncreasingRange<T>>),
    BothSigns(Chain<Once<T>, PrimitiveIntegerUpDown<T>>),
}

impl<T: PrimitiveSigned> Iterator for ExhaustiveSignedRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            ExhaustiveSignedRange::NonNegative(ref mut xs) => xs.next(),
            ExhaustiveSignedRange::NonPositive(ref mut xs) => xs.next(),
            ExhaustiveSignedRange::BothSigns(ref mut xs) => xs.next(),
        }
    }
}

/// Generates all values of a signed integer type in the half-open interval [`a`, `b`), in order of
/// ascending absolute value. When two numbers have the same absolute value, the positive one comes
/// first. `a` must be less than or equal to `b`. If `a` and `b` are equal, the range is empty. This
/// function cannot create a range that includes `T::MAX`; for that, use
/// `exhaustive_signed_range_to_max`.
///
/// Length is `b` - `a`.
///
/// Time: worst case O(1) per iteration
///
/// Additional memory: worst case O(1) per iteration
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_signed_range;
///
/// assert_eq!(
///     exhaustive_signed_range::<i8>(-5, 5).collect::<Vec<_>>(),
///     &[0, 1, -1, 2, -2, 3, -3, 4, -4, -5]
/// )
/// ```
pub fn exhaustive_signed_range<T: PrimitiveSigned>(a: T, b: T) -> ExhaustiveSignedRange<T> {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    if a >= T::ZERO {
        ExhaustiveSignedRange::NonNegative(primitive_integer_increasing_range(a, b))
    } else if b <= T::ZERO {
        ExhaustiveSignedRange::NonPositive(primitive_integer_increasing_range(a, b).rev())
    } else {
        ExhaustiveSignedRange::BothSigns(
            once(T::ZERO).chain(
                primitive_integer_increasing_range(T::ONE, b)
                    .interleave(primitive_integer_increasing_range(a, T::ZERO).rev()),
            ),
        )
    }
}

/// Generates all values of a signed integer type in the closed interval [`a`, `T::MAX`], in order
/// of ascending absolute value. When two numbers have the same absolute value, the positive one
/// comes first.
///
/// Length is 2<sup>`T::WIDTH`</sup> - `a`.
///
/// Time: worst case O(1) per iteration
///
/// Additional memory: worst case O(1) per iteration
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_signed_range_to_max;
///
/// assert_eq!(
///     exhaustive_signed_range_to_max::<i8>(-2).take(10).collect::<Vec<_>>(),
///     &[0, 1, -1, 2, -2, 3, 4, 5, 6, 7]
/// )
/// ```
pub fn exhaustive_signed_range_to_max<T: PrimitiveSigned>(a: T) -> ExhaustiveSignedRange<T> {
    if a >= T::ZERO {
        ExhaustiveSignedRange::NonNegative(primitive_integer_increasing_range_to_max(a))
    } else {
        ExhaustiveSignedRange::BothSigns(
            once(T::ZERO).chain(
                primitive_integer_increasing_range_to_max(T::ONE)
                    .interleave(primitive_integer_increasing_range(a, T::ZERO).rev()),
            ),
        )
    }
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
///     exhaustive_unsigneds::<u8>().take(10).collect::<Vec<_>>(),
///     &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
/// )
/// ```
#[inline]
pub fn exhaustive_unsigneds<T: PrimitiveUnsigned>() -> PrimitiveIntegerIncreasingRange<T> {
    primitive_integer_increasing_range_to_max(T::ZERO)
}

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
///     exhaustive_positive_primitives::<u8>().take(10).collect::<Vec<_>>(),
///     &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
/// )
/// ```
#[inline]
pub fn exhaustive_positive_primitives<T: PrimitiveInteger>() -> PrimitiveIntegerIncreasingRange<T> {
    primitive_integer_increasing_range_to_max(T::ONE)
}

pub type PrimitiveIntegerUpDown<T> =
    Interleave<PrimitiveIntegerIncreasingRange<T>, Rev<PrimitiveIntegerIncreasingRange<T>>>;

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
///     exhaustive_signeds::<i8>().take(10).collect::<Vec<_>>(),
///     &[0, 1, -1, 2, -2, 3, -3, 4, -4, 5]
/// )
/// ```
#[inline]
pub fn exhaustive_signeds<T: PrimitiveSigned>() -> Chain<Once<T>, PrimitiveIntegerUpDown<T>> {
    once(T::ZERO).chain(exhaustive_nonzero_signeds())
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
///     exhaustive_natural_signeds::<i8>().take(10).collect::<Vec<_>>(),
///     &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
/// )
/// ```
#[inline]
pub fn exhaustive_natural_signeds<T: PrimitiveSigned>() -> PrimitiveIntegerIncreasingRange<T> {
    primitive_integer_increasing_range_to_max(T::ZERO)
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
///     exhaustive_negative_signeds::<i8>().take(10).collect::<Vec<_>>(),
///     &[-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]
/// )
/// ```
#[inline]
pub fn exhaustive_negative_signeds<T: PrimitiveSigned>() -> Rev<PrimitiveIntegerIncreasingRange<T>>
{
    primitive_integer_increasing_range(T::MIN, T::ZERO).rev()
}

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
///     exhaustive_nonzero_signeds::<i8>().take(10).collect::<Vec<_>>(),
///     &[1, -1, 2, -2, 3, -3, 4, -4, 5, -5]
/// )
/// ```
#[inline]
pub fn exhaustive_nonzero_signeds<T: PrimitiveSigned>() -> PrimitiveIntegerUpDown<T> {
    exhaustive_positive_primitives().interleave(exhaustive_negative_signeds())
}
