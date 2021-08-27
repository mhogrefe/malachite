use malachite_base::num::basic::traits::{One, Zero};
use natural::Natural;

/// Generates all `Natural`s in a finite interval.
///
/// This `struct` is created by the `exhaustive_natural_range` and
/// `exhaustive_natural_inclusive_range` functions. See their documentation for more.
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

/// Generates all `Natural`s greater than or equal to some `Natural`, in ascending order.
///
/// This `struct` is created by the `exhaustive_natural_range_to_infinity` function. See its
/// documentation for more.
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

/// Generates all `Natural`s in ascending order.
///
/// The output is $(k)_{k=0}^{\infty}$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = \mathcal{O}(i)$
///
/// $M(i) = \mathcal{O}(i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// Although the time and space complexities are worst-case linear, the worst case is very rare. If
/// we exclude the cases where the least-significant limb of the previously-generated value is
/// `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::exhaustive::exhaustive_naturals;
///
/// assert_eq!(
///     exhaustive_naturals()
///         .take(10)
///         .collect_vec()
///         .to_debug_string(),
///     "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]"
/// )
/// ```
#[inline]
pub const fn exhaustive_naturals() -> ExhaustiveNaturalRangeToInfinity {
    exhaustive_natural_range_to_infinity(Natural::ZERO)
}

/// Generates all positive `Natural`s in ascending order.
///
/// The output is $(k)_{k=1}^{\infty}$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = \mathcal{O}(i)$
///
/// $M(i) = \mathcal{O}(i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// Although the time and space complexities are worst-case linear, the worst case is very rare. If
/// we exclude the cases where the least-significant limb of the previously-generated value is
/// `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::exhaustive::exhaustive_positive_naturals;
///
/// assert_eq!(
///     exhaustive_positive_naturals()
///         .take(10)
///         .collect_vec()
///         .to_debug_string(),
///     "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"
/// )
/// ```
#[inline]
pub const fn exhaustive_positive_naturals() -> ExhaustiveNaturalRangeToInfinity {
    exhaustive_natural_range_to_infinity(Natural::ONE)
}

/// Generates all `Natural`s in the half-open interval $[a, b)$, in ascending order.
///
/// `a` must be less than or equal to `b`. If `a` and `b` are equal, the range is empty. To generate
/// all `Natural`s in an infinite interval, use `exhaustive_natural_range_to_infinity`.
///
/// The output is $(k)_{k=a}^{b-1}$.
///
/// The output length is $b - a$.
///
/// # Worst-case complexity per iteration
/// $T(i) = \mathcal{O}(i)$
///
/// $M(i) = \mathcal{O}(i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// Although the time and space complexities are worst-case linear, the worst case is very rare. If
/// we exclude the cases where the least-significant limb of the previously-generated value is
/// `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
/// use malachite_base::num::conversion::traits::ExactFrom;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::exhaustive::exhaustive_natural_range;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     exhaustive_natural_range(Natural::exact_from(5), Natural::exact_from(10))
///         .collect_vec()
///         .to_debug_string(),
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

/// Generates all `Natural`s in the closed interval $[a, b]$, in ascending order.
///
/// `a` must be less than or equal to `b`. If `a` and `b` are equal, the range contains a single
/// element. To generate all `Natural`s in an infinite interval, use
/// `exhaustive_natural_range_to_infinity`.
///
/// The output is $(k)_{k=a}^{b}$.
///
/// The output length is $b - a + 1$.
///
/// # Worst-case complexity per iteration
/// $T(i) = \mathcal{O}(i)$
///
/// $M(i) = \mathcal{O}(i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// Although the time and space complexities are worst-case linear, the worst case is very rare. If
/// we exclude the cases where the least-significant limb of the previously-generated value is
/// `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
/// use malachite_base::num::conversion::traits::ExactFrom;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::exhaustive::exhaustive_natural_inclusive_range;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     exhaustive_natural_inclusive_range(Natural::exact_from(5), Natural::exact_from(10))
///         .collect_vec()
///         .to_debug_string(),
///     "[5, 6, 7, 8, 9, 10]"
/// )
/// ```
#[inline]
pub fn exhaustive_natural_inclusive_range(a: Natural, b: Natural) -> ExhaustiveNaturalRange {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    ExhaustiveNaturalRange {
        a,
        b: b + Natural::ONE,
    }
}

/// Generates all `Natural`s greater than or equal to `a`, in ascending order.
///
/// The output is $(k)_{k=a}^{\infty}$.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = \mathcal{O}(i)$
///
/// $M(i) = \mathcal{O}(i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// Although the time and space complexities are worst-case linear, the worst case is very rare. If
/// we exclude the cases where the least-significant limb of the previously-generated value is
/// `Limb::MAX`, the worst case space and time complexities are constant.
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
/// use malachite_base::num::conversion::traits::ExactFrom;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::exhaustive::exhaustive_natural_range_to_infinity;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     exhaustive_natural_range_to_infinity(Natural::exact_from(5))
///         .take(10)
///         .collect_vec()
///         .to_debug_string(),
///     "[5, 6, 7, 8, 9, 10, 11, 12, 13, 14]"
/// )
/// ```
#[inline]
pub const fn exhaustive_natural_range_to_infinity(a: Natural) -> ExhaustiveNaturalRangeToInfinity {
    ExhaustiveNaturalRangeToInfinity { a }
}
