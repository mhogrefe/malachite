// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::basic::traits::{One, Zero};

/// Generates all [`Natural`]s in a finite interval.
///
/// This `struct` is created by [`exhaustive_natural_range`] and
/// [`exhaustive_natural_inclusive_range`]; see their documentation for more.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExhaustiveNaturalRange {
    a: Natural,
    b: Natural,
}

impl Iterator for ExhaustiveNaturalRange {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.a == self.b {
            None
        } else {
            let result = self.a.clone();
            self.a += Natural::ONE;
            Some(result)
        }
    }
}

impl DoubleEndedIterator for ExhaustiveNaturalRange {
    fn next_back(&mut self) -> Option<Natural> {
        if self.a == self.b {
            None
        } else {
            self.b -= Natural::ONE;
            Some(self.b.clone())
        }
    }
}

/// Generates all [`Natural`]s greater than or equal to some [`Natural`], in ascending order.
///
/// This `struct` is created by [`exhaustive_natural_range_to_infinity`]; see its documentation for
/// more.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExhaustiveNaturalRangeToInfinity {
    a: Natural,
}

impl Iterator for ExhaustiveNaturalRangeToInfinity {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        let result = self.a.clone();
        self.a += Natural::ONE;
        Some(result)
    }
}

/// Generates all [`Natural`]s in ascending order.
///
/// The output is $(k)_{k=0}^{\infty}$.
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
/// we exclude the cases where the least-significant limb of the previously-generated value is
/// `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::natural::exhaustive::exhaustive_naturals;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_naturals(), 10),
///     "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, ...]"
/// );
/// ```
#[inline]
pub const fn exhaustive_naturals() -> ExhaustiveNaturalRangeToInfinity {
    exhaustive_natural_range_to_infinity(Natural::ZERO)
}

/// Generates all positive [`Natural`]s in ascending order.
///
/// The output is $(k)_{k=1}^{\infty}$.
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
/// we exclude the cases where the least-significant limb of the previously-generated value is
/// `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::natural::exhaustive::exhaustive_positive_naturals;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_positive_naturals(), 10),
///     "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, ...]"
/// )
/// ```
#[inline]
pub const fn exhaustive_positive_naturals() -> ExhaustiveNaturalRangeToInfinity {
    exhaustive_natural_range_to_infinity(Natural::ONE)
}

/// Generates all [`Natural`]s in the half-open interval $[a, b)$, in ascending order.
///
/// `a` must be less than or equal to `b`. If `a` and `b` are equal, the range is empty. To generate
/// all [`Natural`]s in an infinite interval, use [`exhaustive_natural_range_to_infinity`].
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
/// we exclude the cases where the least-significant limb of the previously-generated value is
/// `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::exhaustive::exhaustive_natural_range;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     exhaustive_natural_range(Natural::from(5u32), Natural::from(10u32))
///         .collect_vec()
///         .to_debug_string(),
///     "[5, 6, 7, 8, 9]"
/// )
/// ```
#[inline]
pub fn exhaustive_natural_range(a: Natural, b: Natural) -> ExhaustiveNaturalRange {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    ExhaustiveNaturalRange { a, b }
}

/// Generates all [`Natural`]s in the closed interval $[a, b]$, in ascending order.
///
/// `a` must be less than or equal to `b`. If `a` and `b` are equal, the range contains a single
/// element. To generate all [`Natural`]s in an infinite interval, use
/// [`exhaustive_natural_range_to_infinity`].
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
/// we exclude the cases where the least-significant limb of the previously-generated value is
/// `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Panics
/// Panics if $a>b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::exhaustive::exhaustive_natural_inclusive_range;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     exhaustive_natural_inclusive_range(Natural::from(5u32), Natural::from(10u32))
///         .collect_vec()
///         .to_debug_string(),
///     "[5, 6, 7, 8, 9, 10]"
/// )
/// ```
#[inline]
pub fn exhaustive_natural_inclusive_range(a: Natural, b: Natural) -> ExhaustiveNaturalRange {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    ExhaustiveNaturalRange {
        a,
        b: b + Natural::ONE,
    }
}

/// Generates all [`Natural`]s greater than or equal to some number $a$, in ascending order.
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
/// we exclude the cases where the least-significant limb of the previously-generated value is
/// `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_nz::natural::exhaustive::exhaustive_natural_range_to_infinity;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_natural_range_to_infinity(Natural::from(5u32)),
///         10
///     ),
///     "[5, 6, 7, 8, 9, 10, 11, 12, 13, 14, ...]"
/// )
/// ```
#[inline]
pub const fn exhaustive_natural_range_to_infinity(a: Natural) -> ExhaustiveNaturalRangeToInfinity {
    ExhaustiveNaturalRangeToInfinity { a }
}
