use std::iter::{once, Chain, Once, Rev};

use itertools::{Interleave, Itertools};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};

use integer::Integer;

/// Generates all `Integer`s in a finite interval, in ascending order.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IntegerIncreasingRange {
    a: Integer,
    b: Integer,
}

impl Iterator for IntegerIncreasingRange {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        if self.a == self.b {
            None
        } else {
            let result = self.a.clone();
            self.a += Integer::ONE;
            Some(result)
        }
    }
}

impl DoubleEndedIterator for IntegerIncreasingRange {
    fn next_back(&mut self) -> Option<Integer> {
        if self.a == self.b {
            None
        } else {
            self.b -= Integer::ONE;
            Some(self.b.clone())
        }
    }
}

/// Generates all `Integer`s in the half-open interval [`a`, `b`), in ascending order. `a` must be
/// less than or equal to `b`. If `a` and `b` are equal, the range is empty. To generate all
/// `Integer`s in an infinite interval in ascending or descending order, use
/// `integer_increasing_range_to_infinity` or `integer_decreasing_range_to_negative_infinity`.
///
/// Length is `b` - `a`.
///
/// Time for the ith iteration: worst case O(i)
///
/// Additional memory for the ith iteration: amortized O(1)
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::integer_increasing_range;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     integer_increasing_range(Integer::from(-4), Integer::from(4))
///         .collect::<Vec<_>>().to_debug_string(),
///     "[-4, -3, -2, -1, 0, 1, 2, 3]"
/// )
/// ```
#[inline]
pub fn integer_increasing_range(a: Integer, b: Integer) -> IntegerIncreasingRange {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    IntegerIncreasingRange { a, b }
}

/// Generates all `Integer`s greater than or equal to some `Integer`, in ascending order.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IntegerIncreasingRangeToInfinity {
    a: Integer,
}

impl Iterator for IntegerIncreasingRangeToInfinity {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let result = self.a.clone();
        self.a += Integer::ONE;
        Some(result)
    }
}

/// Generates all `Integer`s greater than or equal to `a`, in ascending order.
///
/// Length is infinite.
///
/// Time for the ith iteration: worst case O(i)
///
/// Additional memory for the ith iteration: amortized O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::integer_increasing_range_to_infinity;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     integer_increasing_range_to_infinity(Integer::from(-4)).take(10)
///         .collect::<Vec<_>>().to_debug_string(),
///     "[-4, -3, -2, -1, 0, 1, 2, 3, 4, 5]"
/// )
/// ```
#[inline]
pub const fn integer_increasing_range_to_infinity(a: Integer) -> IntegerIncreasingRangeToInfinity {
    IntegerIncreasingRangeToInfinity { a }
}

/// Generates all `Integer`s less than or equal to some `Integer`, in descending order.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IntegerDecreasingRangeToNegativeInfinity {
    a: Integer,
}

impl Iterator for IntegerDecreasingRangeToNegativeInfinity {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let result = self.a.clone();
        self.a -= Integer::ONE;
        Some(result)
    }
}

/// Generates all `Integer`s less than or equal to `a`, in descending order.
///
/// Length is infinite.
///
/// Time for the ith iteration: worst case O(i)
///
/// Additional memory for the ith iteration: amortized O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::integer_decreasing_range_to_negative_infinity;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     integer_decreasing_range_to_negative_infinity(Integer::from(4)).take(10)
///         .collect::<Vec<_>>().to_debug_string(),
///     "[4, 3, 2, 1, 0, -1, -2, -3, -4, -5]"
/// )
/// ```
#[inline]
pub const fn integer_decreasing_range_to_negative_infinity(
    a: Integer,
) -> IntegerDecreasingRangeToNegativeInfinity {
    IntegerDecreasingRangeToNegativeInfinity { a }
}

/// Generates all `Integer`s in a half-open interval, in order of ascending absolute value. When two
/// numbers have the same absolute value, the positive one comes first.
#[derive(Clone, Debug)]
pub enum ExhaustiveIntegerRange {
    NonNegative(IntegerIncreasingRange),
    NonPositive(Rev<IntegerIncreasingRange>),
    BothSigns(
        Chain<Once<Integer>, Interleave<IntegerIncreasingRange, Rev<IntegerIncreasingRange>>>,
    ),
}

impl Iterator for ExhaustiveIntegerRange {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        match self {
            ExhaustiveIntegerRange::NonNegative(ref mut xs) => xs.next(),
            ExhaustiveIntegerRange::NonPositive(ref mut xs) => xs.next(),
            ExhaustiveIntegerRange::BothSigns(ref mut xs) => xs.next(),
        }
    }
}

/// Generates all `Integer`s in the half-open interval [`a`, `b`), in order of ascending absolute
/// value. When two numbers have the same absolute value, the positive one comes first. `a` must be
/// less than or equal to `b`. If `a` and `b` are equal, the range is empty. To generate all
/// `Integer`s in an infinite interval, use `exhaustive_integer_range_to_infinity`,
/// `exhaustive_integer_range_to_negative_infinity`, or `exhaustive_integers`.
///
/// Length is `b` - `a`.
///
/// Time for the ith iteration: worst case O(i)
///
/// Additional memory for the ith iteration: amortized O(1)
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::exhaustive_integer_range;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     exhaustive_integer_range(Integer::from(-4), Integer::from(4))
///         .collect::<Vec<_>>().to_debug_string(),
///     "[0, 1, -1, 2, -2, 3, -3, -4]"
/// )
/// ```
pub fn exhaustive_integer_range(a: Integer, b: Integer) -> ExhaustiveIntegerRange {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    if a >= 0 {
        ExhaustiveIntegerRange::NonNegative(integer_increasing_range(a, b))
    } else if b <= 0 {
        ExhaustiveIntegerRange::NonPositive(integer_increasing_range(a, b).rev())
    } else {
        ExhaustiveIntegerRange::BothSigns(
            once(Integer::ZERO).chain(
                integer_increasing_range(Integer::ONE, b)
                    .interleave(integer_increasing_range(a, Integer::ZERO).rev()),
            ),
        )
    }
}

/// Generates all `Integer`s greater than or equal to some `Integer`, in order of ascending absolute
/// value. When two numbers have the same absolute value, the positive one comes first.
#[derive(Clone, Debug)]
pub enum ExhaustiveIntegerRangeToInfinity {
    NonNegative(IntegerIncreasingRangeToInfinity),
    BothSigns(
        Chain<
            Once<Integer>,
            Interleave<IntegerIncreasingRangeToInfinity, Rev<IntegerIncreasingRange>>,
        >,
    ),
}

impl Iterator for ExhaustiveIntegerRangeToInfinity {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        match self {
            ExhaustiveIntegerRangeToInfinity::NonNegative(ref mut xs) => xs.next(),
            ExhaustiveIntegerRangeToInfinity::BothSigns(ref mut xs) => xs.next(),
        }
    }
}

/// Generates all `Integer`s greater than or equal to `a`, in order of ascending absolute value.
/// When two numbers have the same absolute value, the positive one comes first.
///
/// Length is infinite.
///
/// Time for the ith iteration: worst case O(i)
///
/// Additional memory for the ith iteration: amortized O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::exhaustive_integer_range_to_infinity;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     exhaustive_integer_range_to_infinity(Integer::from(-2)).take(10)
///         .collect::<Vec<_>>().to_debug_string(),
///     "[0, 1, -1, 2, -2, 3, 4, 5, 6, 7]"
/// )
/// ```
#[inline]
pub fn exhaustive_integer_range_to_infinity(a: Integer) -> ExhaustiveIntegerRangeToInfinity {
    if a >= 0 {
        ExhaustiveIntegerRangeToInfinity::NonNegative(integer_increasing_range_to_infinity(a))
    } else {
        ExhaustiveIntegerRangeToInfinity::BothSigns(
            once(Integer::ZERO).chain(
                integer_increasing_range_to_infinity(Integer::ONE)
                    .interleave(integer_increasing_range(a, Integer::ZERO).rev()),
            ),
        )
    }
}

/// Generates all `Integer`s less than or equal to some `Integer`, in order of ascending absolute
/// value. When two numbers have the same absolute value, the positive one comes first.
#[derive(Clone, Debug)]
pub enum ExhaustiveIntegerRangeToNegativeInfinity {
    NonPositive(IntegerDecreasingRangeToNegativeInfinity),
    BothSigns(
        Chain<
            Once<Integer>,
            Interleave<IntegerIncreasingRange, IntegerDecreasingRangeToNegativeInfinity>,
        >,
    ),
}

impl Iterator for ExhaustiveIntegerRangeToNegativeInfinity {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        match self {
            ExhaustiveIntegerRangeToNegativeInfinity::NonPositive(ref mut xs) => xs.next(),
            ExhaustiveIntegerRangeToNegativeInfinity::BothSigns(ref mut xs) => xs.next(),
        }
    }
}

/// Generates all `Integer`s less than or equal to `a`, in order of ascending absolute value. When
/// two numbers have the same absolute value, the positive one comes first.
///
/// Length is infinite.
///
/// Time for the ith iteration: worst case O(i)
///
/// Additional memory for the ith iteration: amortized O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::exhaustive_integer_range_to_negative_infinity;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     exhaustive_integer_range_to_negative_infinity(Integer::from(2)).take(10)
///         .collect::<Vec<_>>().to_debug_string(),
///     "[0, 1, -1, 2, -2, -3, -4, -5, -6, -7]"
/// )
/// ```
#[inline]
pub fn exhaustive_integer_range_to_negative_infinity(
    a: Integer,
) -> ExhaustiveIntegerRangeToNegativeInfinity {
    if a <= 0 {
        ExhaustiveIntegerRangeToNegativeInfinity::NonPositive(
            integer_decreasing_range_to_negative_infinity(a),
        )
    } else {
        ExhaustiveIntegerRangeToNegativeInfinity::BothSigns(once(Integer::ZERO).chain(
            integer_increasing_range(Integer::ONE, a + Integer::ONE).interleave(
                integer_decreasing_range_to_negative_infinity(Integer::NEGATIVE_ONE),
            ),
        ))
    }
}

pub type IntegerUpDown =
    Interleave<IntegerIncreasingRangeToInfinity, IntegerDecreasingRangeToNegativeInfinity>;

/// Generates all `Integer`s, in order of ascending absolute value. When two numbers have the same
/// absolute value, the positive one comes first.
///
/// Length is infinite.
///
/// Time for the ith iteration: worst case O(i)
///
/// Additional memory for the ith iteration: amortized O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::exhaustive_integers;
///
/// assert_eq!(
///     exhaustive_integers().take(10).collect::<Vec<_>>().to_debug_string(),
///     "[0, 1, -1, 2, -2, 3, -3, 4, -4, 5]"
/// )
/// ```
#[inline]
pub fn exhaustive_integers() -> Chain<Once<Integer>, IntegerUpDown> {
    once(Integer::ZERO).chain(exhaustive_nonzero_integers())
}

/// Generates all natural `Integer`s in ascending order.
///
/// Length is infinite.
///
/// Time for the ith iteration: worst case O(i)
///
/// Additional memory for the ith iteration: amortized O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::exhaustive_natural_integers;
///
/// assert_eq!(
///     exhaustive_natural_integers().take(10).collect::<Vec<_>>().to_debug_string(),
///     "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]"
/// )
/// ```
#[inline]
pub const fn exhaustive_natural_integers() -> IntegerIncreasingRangeToInfinity {
    integer_increasing_range_to_infinity(Integer::ZERO)
}

/// Generates all positive `Integer`s in ascending order.
///
/// Length is infinite.
///
/// Time for the ith iteration: worst case O(i)
///
/// Additional memory for the ith iteration: amortized O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::exhaustive_positive_integers;
///
/// assert_eq!(
///     exhaustive_positive_integers().take(10).collect::<Vec<_>>().to_debug_string(),
///     "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"
/// )
/// ```
#[inline]
pub const fn exhaustive_positive_integers() -> IntegerIncreasingRangeToInfinity {
    integer_increasing_range_to_infinity(Integer::ONE)
}

/// Generates all negative `Integer`s in descending order.
///
/// Length is infinite.
///
/// Time for the ith iteration: worst case O(i)
///
/// Additional memory for the ith iteration: amortized O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::exhaustive_negative_integers;
///
/// assert_eq!(
///     exhaustive_negative_integers().take(10).collect::<Vec<_>>().to_debug_string(),
///     "[-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]"
/// )
/// ```
#[inline]
pub const fn exhaustive_negative_integers() -> IntegerDecreasingRangeToNegativeInfinity {
    integer_decreasing_range_to_negative_infinity(Integer::NEGATIVE_ONE)
}

/// Generates all nonzero `Integer`s, in order of ascending absolute value. When two numbers have
/// the same absolute value, the positive one comes first.
///
/// Length is infinite.
///
/// Time for the ith iteration: worst case O(i)
///
/// Additional memory for the ith iteration: amortized O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::exhaustive_nonzero_integers;
///
/// assert_eq!(
///     exhaustive_nonzero_integers().take(10).collect::<Vec<_>>().to_debug_string(),
///     "[1, -1, 2, -2, 3, -3, 4, -4, 5, -5]"
/// )
/// ```
#[inline]
pub fn exhaustive_nonzero_integers() -> IntegerUpDown {
    exhaustive_positive_integers().interleave(exhaustive_negative_integers())
}
