// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::iter::{once, Chain, Once, Rev};
use itertools::{Interleave, Itertools};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};

/// Generates all [`Integer`]s in a finite interval, in ascending order.
///
/// This `struct` is created by the [`integer_increasing_range`] and
/// [`integer_increasing_inclusive_range`]; see their documentation for more.
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

/// Generates all [`Integer`]s greater than or equal to some [`Integer`], in ascending order.
///
/// This `struct` is created by [`integer_increasing_range_to_infinity`]; see its documentation for
/// more.
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

/// Generates all [`Integer`]s less than or equal to some [`Integer`], in ascending order.
///
/// This `struct` is created by [`integer_decreasing_range_to_negative_infinity`]; see its
/// documentation for more.
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

/// Generates all [`Integer`]s in a finite interval, in order of increasing absolute value.
///
/// This `struct` is created [`exhaustive_integer_range`] and
/// [`exhaustive_integer_inclusive_range`]; see their documentation for more.
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

/// Generates all [`Integer`]s greater than or equal to some [`Integer`], in order of increasing
/// absolute value.
///
/// This `struct` is created by [`exhaustive_integer_range_to_infinity`]; see its documentation for
/// more.
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

/// Generates all [`Integer`]s less than or equal to some [`Integer`], in order of increasing
/// absolute value.
///
/// This `struct` is created by [`exhaustive_integer_range_to_negative_infinity`]; see its
/// documentation for more.
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

#[doc(hidden)]
pub type IntegerUpDown =
    Interleave<IntegerIncreasingRangeToInfinity, IntegerDecreasingRangeToNegativeInfinity>;

/// Generates all [`Integer`]s, in order of increasing absolute value. When two [`Integer`]s have
/// the same absolute value, the positive one comes first.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(i)$
///
/// $M(i) = O(1)$, amortized.
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::integer::exhaustive::exhaustive_integers;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_integers(), 10),
///     "[0, 1, -1, 2, -2, 3, -3, 4, -4, 5, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_integers() -> Chain<Once<Integer>, IntegerUpDown> {
    once(Integer::ZERO).chain(exhaustive_nonzero_integers())
}

/// Generates all natural [`Integer`]s in ascending order.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(i)$
///
/// $M(i) = O(1)$, amortized.
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::integer::exhaustive::exhaustive_natural_integers;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_natural_integers(), 10),
///     "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, ...]"
/// )
/// ```
#[inline]
pub const fn exhaustive_natural_integers() -> IntegerIncreasingRangeToInfinity {
    integer_increasing_range_to_infinity(Integer::ZERO)
}

/// Generates all positive [`Integer`]s in ascending order.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(i)$
///
/// $M(i) = O(1)$, amortized.
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::integer::exhaustive::exhaustive_positive_integers;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_positive_integers(), 10),
///     "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, ...]"
/// )
/// ```
#[inline]
pub const fn exhaustive_positive_integers() -> IntegerIncreasingRangeToInfinity {
    integer_increasing_range_to_infinity(Integer::ONE)
}

/// Generates all negative [`Integer`]s in descending order.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(i)$
///
/// $M(i) = O(1)$, amortized.
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::integer::exhaustive::exhaustive_negative_integers;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_negative_integers(), 10),
///     "[-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, ...]"
/// )
/// ```
#[inline]
pub const fn exhaustive_negative_integers() -> IntegerDecreasingRangeToNegativeInfinity {
    integer_decreasing_range_to_negative_infinity(Integer::NEGATIVE_ONE)
}

/// Generates all nonzero [`Integer`]s, in order of increasing absolute value. When two [`Integer`]s
/// have the same absolute value, the positive one comes first.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(i)$
///
/// $M(i) = O(1)$, amortized.
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::integer::exhaustive::exhaustive_nonzero_integers;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_nonzero_integers(), 10),
///     "[1, -1, 2, -2, 3, -3, 4, -4, 5, -5, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_nonzero_integers() -> IntegerUpDown {
    exhaustive_positive_integers().interleave(exhaustive_negative_integers())
}

/// Generates all [`Integer`]s in the half-open interval $[a, b)$, in ascending order.
///
/// $a$ must be less than or equal to $b$. If $a$ and $b$ are equal, the range is empty. To generate
/// all [`Integer`]s in an infinite interval in ascending or descending order, use
/// [`integer_increasing_range_to_infinity`] or [`integer_decreasing_range_to_negative_infinity`].
///
/// The output is $(k)_{k=a}^{b-1}$.
///
/// The output length is $b - a$.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(i)$
///
/// $M(i) = O(i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// Although the time and space complexities are worst-case linear, the worst case is very rare. If
/// we exclude the cases where the the previously-generated value is positive and its
/// least-significant limb is `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::integer_increasing_range;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     integer_increasing_range(Integer::from(-4), Integer::from(4))
///         .collect_vec()
///         .to_debug_string(),
///     "[-4, -3, -2, -1, 0, 1, 2, 3]"
/// )
/// ```
#[inline]
pub fn integer_increasing_range(a: Integer, b: Integer) -> IntegerIncreasingRange {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    IntegerIncreasingRange { a, b }
}

/// Generates all [`Integer`]s in the closed interval $[a, b]$, in ascending order.
///
/// $a$ must be less than or equal to $b$. If $a$ and $b$ are equal, the range contains a single
/// element. To generate all [`Integer`]s in an infinite interval in ascending or descending order,
/// use [`integer_increasing_range_to_infinity`] or
/// [`integer_decreasing_range_to_negative_infinity`].
///
/// The output is $(k)_{k=a}^{b}$.
///
/// The output length is $b - a + 1$.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(i)$
///
/// $M(i) = O(i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// Although the time and space complexities are worst-case linear, the worst case is very rare. If
/// we exclude the cases where the the previously-generated value is positive and its
/// least-significant limb is `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::integer_increasing_inclusive_range;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     integer_increasing_inclusive_range(Integer::from(-4), Integer::from(4))
///         .collect_vec()
///         .to_debug_string(),
///     "[-4, -3, -2, -1, 0, 1, 2, 3, 4]"
/// )
/// ```
#[inline]
pub fn integer_increasing_inclusive_range(a: Integer, b: Integer) -> IntegerIncreasingRange {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    IntegerIncreasingRange {
        a,
        b: b + Integer::ONE,
    }
}

/// Generates all [`Integer`]s greater than or equal to some number $a$, in ascending order.
///
/// The output is $(k)_{k=a}^{\infty}$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(i)$
///
/// $M(i) = O(i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// Although the time and space complexities are worst-case linear, the worst case is very rare. If
/// we exclude the cases where the the previously-generated value is positive and its
/// least-significant limb is `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::integer::exhaustive::integer_increasing_range_to_infinity;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     prefix_to_string(integer_increasing_range_to_infinity(Integer::from(-4)), 10),
///     "[-4, -3, -2, -1, 0, 1, 2, 3, 4, 5, ...]"
/// )
/// ```
#[inline]
pub const fn integer_increasing_range_to_infinity(a: Integer) -> IntegerIncreasingRangeToInfinity {
    IntegerIncreasingRangeToInfinity { a }
}

/// Generates all [`Integer`]s less than or equal to some number $a$, in descending order.
///
/// The output is $(-k)_{k=-a}^{\infty}$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(i)$
///
/// $M(i) = O(i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// Although the time and space complexities are worst-case linear, the worst case is very rare. If
/// we exclude the cases where the the previously-generated value is negative and the
/// least-significant limb of its absolute value is `Limb::MAX`, the worst case space and time
/// complexities are constant.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::integer::exhaustive::integer_decreasing_range_to_negative_infinity;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     prefix_to_string(
///         integer_decreasing_range_to_negative_infinity(Integer::from(4)),
///         10
///     ),
///     "[4, 3, 2, 1, 0, -1, -2, -3, -4, -5, ...]"
/// )
/// ```
#[inline]
pub const fn integer_decreasing_range_to_negative_infinity(
    a: Integer,
) -> IntegerDecreasingRangeToNegativeInfinity {
    IntegerDecreasingRangeToNegativeInfinity { a }
}

/// Generates all [`Integer`]s in the half-open interval $[a, b)$, in order of increasing absolute
/// value.
///
/// When two [`Integer`]s have the same absolute value, the positive one comes first. $a$ must be
/// less than or equal to $b$. If $a$ and $b$ are equal, the range is empty.
///
/// The output satisfies $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|,
/// \operatorname{sgn}(-x_j))$ whenever $i, j \\in [0, b - a)$ and $i < j$.
///
/// The output length is $b - a$.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(i)$
///
/// $M(i) = O(i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// Although the time and space complexities are worst-case linear, the worst case is very rare. If
/// we exclude the cases where the the least-significant limb of the absolute value of the
/// previously-generated value is `Limb::MAX`, the worst case space and time complexities are
/// constant.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::exhaustive_integer_range;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     exhaustive_integer_range(Integer::from(-4), Integer::from(4))
///         .collect_vec()
///         .to_debug_string(),
///     "[0, 1, -1, 2, -2, 3, -3, -4]"
/// )
/// ```
pub fn exhaustive_integer_range(a: Integer, b: Integer) -> ExhaustiveIntegerRange {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
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

/// Generates all [`Integer`]s in the closed interval $[a, b]$, in order of increasing absolute
/// value.
///
/// When two [`Integer`]s have the same absolute value, the positive one comes first. $a$ must be
/// less than or equal to $b$. If $a$ and $b$ are equal, the range contains a single element.
///
/// The output satisfies $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|,
/// \operatorname{sgn}(-x_j))$ whenever $i, j \\in [0, b - a]$ and $i < j$.
///
/// The output length is $b - a + 1$.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(i)$
///
/// $M(i) = O(i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// Although the time and space complexities are worst-case linear, the worst case is very rare. If
/// we exclude the cases where the the least-significant limb of the absolute value of the
/// previously-generated value is `Limb::MAX`, the worst case space and time complexities are
/// constant.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::exhaustive::exhaustive_integer_inclusive_range;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     exhaustive_integer_inclusive_range(Integer::from(-4), Integer::from(4))
///         .collect_vec()
///         .to_debug_string(),
///     "[0, 1, -1, 2, -2, 3, -3, 4, -4]"
/// )
/// ```
pub fn exhaustive_integer_inclusive_range(a: Integer, b: Integer) -> ExhaustiveIntegerRange {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    if a >= 0 {
        ExhaustiveIntegerRange::NonNegative(integer_increasing_inclusive_range(a, b))
    } else if b <= 0 {
        ExhaustiveIntegerRange::NonPositive(integer_increasing_inclusive_range(a, b).rev())
    } else {
        ExhaustiveIntegerRange::BothSigns(
            once(Integer::ZERO).chain(
                integer_increasing_inclusive_range(Integer::ONE, b)
                    .interleave(integer_increasing_inclusive_range(a, Integer::NEGATIVE_ONE).rev()),
            ),
        )
    }
}

/// Generates all [`Integer`]s greater than or equal to some number $a$, in order of increasing
/// absolute value.
///
/// When two [`Integer`]s have the same absolute value, the positive one comes first.
///
/// The output satisfies $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|,
/// \operatorname{sgn}(-x_j))$ whenever $i < j$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(i)$
///
/// $M(i) = O(i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// Although the time and space complexities are worst-case linear, the worst case is very rare. If
/// we exclude the cases where the the previously-generated value is positive and its
/// least-significant limb is `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::integer::exhaustive::exhaustive_integer_range_to_infinity;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_integer_range_to_infinity(Integer::from(-2)), 10),
///     "[0, 1, -1, 2, -2, 3, 4, 5, 6, 7, ...]"
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

/// Generates all [`Integer`]s less than or equal to some number $a$, in order of increasing
/// absolute value.
///
/// When two [`Integer`]s have the same absolute value, the positive one comes first.
///
/// The output satisfies $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|,
/// \operatorname{sgn}(-x_j))$ whenever $i < j$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(i)$
///
/// $M(i) = O(i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// Although the time and space complexities are worst-case linear, the worst case is very rare. If
/// we exclude the cases where the the previously-generated value is positive and its
/// least-significant limb is `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::integer::exhaustive::exhaustive_integer_range_to_negative_infinity;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_integer_range_to_negative_infinity(Integer::from(2)),
///         10
///     ),
///     "[0, 1, -1, 2, -2, -3, -4, -5, -6, -7, ...]"
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
