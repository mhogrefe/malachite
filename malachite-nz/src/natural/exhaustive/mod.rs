use malachite_base::num::basic::traits::{One, Zero};

use natural::Natural;

/// Generates all `Natural`s in a finite interval, in ascending order.
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

/// Generates all `Natural`s in the half-open interval [`a`, `b`), in ascending order. `a` must be
/// less than or equal to `b`. If `a` and `b` are equal, the range is empty. To generate all
/// `Natural`s in an infinite interval, use `exhaustive_natural_range_to_infinity`.
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
/// use malachite_base::num::conversion::traits::ExactFrom;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::exhaustive::exhaustive_natural_range;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     exhaustive_natural_range(Natural::exact_from(5), Natural::exact_from(10))
///         .collect::<Vec<_>>().to_debug_string(),
///     "[5, 6, 7, 8, 9]"
/// )
/// ```
#[inline]
pub fn exhaustive_natural_range(a: Natural, b: Natural) -> ExhaustiveNaturalRange {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    ExhaustiveNaturalRange { a, b }
}

/// Generates all `Natural`s greater than or equal to some `Natural`, in ascending order.
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

/// Generates all `Natural`s greater than or equal to `a`, in ascending order.
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
/// use malachite_base::num::conversion::traits::ExactFrom;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::exhaustive::exhaustive_natural_range_to_infinity;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     exhaustive_natural_range_to_infinity(Natural::exact_from(5)).take(10)
///         .collect::<Vec<_>>().to_debug_string(),
///     "[5, 6, 7, 8, 9, 10, 11, 12, 13, 14]"
/// )
/// ```
#[inline]
pub const fn exhaustive_natural_range_to_infinity(a: Natural) -> ExhaustiveNaturalRangeToInfinity {
    ExhaustiveNaturalRangeToInfinity { a }
}

/// Generates all `Natural`s in ascending order.
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
/// use malachite_nz::natural::exhaustive::exhaustive_naturals;
///
/// assert_eq!(
///     exhaustive_naturals().take(10).collect::<Vec<_>>().to_debug_string(),
///     "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]"
/// )
/// ```
#[inline]
pub const fn exhaustive_naturals() -> ExhaustiveNaturalRangeToInfinity {
    exhaustive_natural_range_to_infinity(Natural::ZERO)
}

/// Generates all positive `Natural`s in ascending order.
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
/// use malachite_nz::natural::exhaustive::exhaustive_positive_naturals;
///
/// assert_eq!(
///     exhaustive_positive_naturals().take(10).collect::<Vec<_>>().to_debug_string(),
///     "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"
/// )
/// ```
#[inline]
pub const fn exhaustive_positive_naturals() -> ExhaustiveNaturalRangeToInfinity {
    exhaustive_natural_range_to_infinity(Natural::ONE)
}
