// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::bools::random::{
    get_weighted_random_bool, random_bools, weighted_random_bools, RandomBools, WeightedRandomBools,
};
use crate::num::arithmetic::traits::Gcd;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactInto;
use crate::random::Seed;
use std::fmt::Debug;

use super::VariableRangeGenerator;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SimpleRational {
    pub(crate) n: u64,
    pub(crate) d: u64,
}

impl SimpleRational {
    pub(crate) fn new(n: u64, d: u64) -> SimpleRational {
        assert_ne!(d, 0);
        let gcd = n.gcd(d);
        SimpleRational {
            n: n / gcd,
            d: d / gcd,
        }
    }

    fn inverse(self) -> SimpleRational {
        assert_ne!(self.n, 0);
        SimpleRational {
            n: self.d,
            d: self.n,
        }
    }

    // unwrap not const yet
    #[allow(clippy::missing_const_for_fn)]
    fn sub_u64(self, x: u64) -> SimpleRational {
        SimpleRational {
            n: self.n.checked_sub(x.checked_mul(self.d).unwrap()).unwrap(),
            d: self.d,
        }
    }
}

pub(crate) fn mean_to_p_with_min<T: PrimitiveInt>(
    min: T,
    um_numerator: u64,
    um_denominator: u64,
) -> (u64, u64) {
    let um = SimpleRational::new(um_numerator, um_denominator);
    let p = um.sub_u64(ExactInto::<u64>::exact_into(min)).inverse();
    (p.n, p.d)
}

/// Generates random unsigned integers from a truncated geometric distribution.
#[derive(Clone, Debug)]
pub struct GeometricRandomNaturalValues<T: PrimitiveInt> {
    xs: WeightedRandomBools,
    min: T,
    max: T,
}

impl<T: PrimitiveInt> Iterator for GeometricRandomNaturalValues<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut failures = self.min;
        loop {
            if self.xs.next().unwrap() {
                return Some(failures);
            }
            // Wrapping to min is equivalent to restarting this function.
            if failures == self.max {
                failures = self.min;
            } else {
                failures += T::ONE;
            }
        }
    }
}

fn geometric_random_natural_values_inclusive_range<T: PrimitiveInt>(
    seed: Seed,
    min: T,
    max: T,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    assert!(min <= max);
    assert_ne!(um_denominator, 0);
    let (numerator, denominator) = mean_to_p_with_min(min, um_numerator, um_denominator);
    GeometricRandomNaturalValues {
        xs: weighted_random_bools(seed, numerator, numerator.checked_add(denominator).unwrap()),
        min,
        max,
    }
}

fn get_geometric_random_natural_value_from_inclusive_range<T: PrimitiveInt>(
    range_generator: &mut VariableRangeGenerator,
    min: T,
    max: T,
    um_numerator: u64,
    um_denominator: u64,
) -> T {
    assert!(min <= max);
    assert_ne!(um_denominator, 0);
    let (n, denominator) = mean_to_p_with_min(min, um_numerator, um_denominator);
    let d = n.checked_add(denominator).unwrap();
    let mut failures = min;
    loop {
        if get_weighted_random_bool(range_generator, n, d) {
            return failures;
        }
        // Wrapping to min is equivalent to restarting this function.
        if failures == max {
            failures = min;
        } else {
            failures += T::ONE;
        }
    }
}

/// Generates random negative signed integers from a modified geometric distribution.
#[derive(Clone, Debug)]
pub struct GeometricRandomNegativeSigneds<T: PrimitiveSigned> {
    xs: WeightedRandomBools,
    abs_min: T,
    abs_max: T,
}

impl<T: PrimitiveSigned> Iterator for GeometricRandomNegativeSigneds<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut result = self.abs_min;
        loop {
            if self.xs.next().unwrap() {
                return Some(result);
            }
            // Wrapping to min is equivalent to restarting this function.
            if result == self.abs_max {
                result = self.abs_min;
            } else {
                result -= T::ONE;
            }
        }
    }
}

fn geometric_random_negative_signeds_inclusive_range<T: PrimitiveSigned>(
    seed: Seed,
    abs_min: T,
    abs_max: T,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> GeometricRandomNegativeSigneds<T> {
    assert!(abs_min >= abs_max);
    assert_ne!(abs_um_denominator, 0);
    let (numerator, denominator) = mean_to_p_with_min(
        abs_min.checked_neg().unwrap(),
        abs_um_numerator,
        abs_um_denominator,
    );
    GeometricRandomNegativeSigneds {
        xs: weighted_random_bools(seed, numerator, numerator.checked_add(denominator).unwrap()),
        abs_min,
        abs_max,
    }
}

fn get_geometric_random_negative_signed_from_inclusive_range<T: PrimitiveSigned>(
    range_generator: &mut VariableRangeGenerator,
    abs_min: T,
    abs_max: T,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> T {
    assert!(abs_min >= abs_max);
    assert_ne!(abs_um_denominator, 0);
    let (n, denominator) = mean_to_p_with_min(
        abs_min.checked_neg().unwrap(),
        abs_um_numerator,
        abs_um_denominator,
    );
    let d = n.checked_add(denominator).unwrap();
    let mut result = abs_min;
    loop {
        if get_weighted_random_bool(range_generator, n, d) {
            return result;
        }
        // Wrapping to min is equivalent to restarting this function.
        if result == abs_max {
            result = abs_min;
        } else {
            result -= T::ONE;
        }
    }
}

/// Generates random nonzero signed integers from a modified geometric distribution.
#[derive(Clone, Debug)]
pub struct GeometricRandomNonzeroSigneds<T: PrimitiveSigned> {
    bs: RandomBools,
    xs: WeightedRandomBools,
    min: T,
    max: T,
}

impl<T: PrimitiveSigned> Iterator for GeometricRandomNonzeroSigneds<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            if self.bs.next().unwrap() {
                let mut result = T::ONE;
                loop {
                    if self.xs.next().unwrap() {
                        return Some(result);
                    } else if result == self.max {
                        break;
                    }
                    result += T::ONE;
                }
            } else {
                let mut result = T::NEGATIVE_ONE;
                loop {
                    if self.xs.next().unwrap() {
                        return Some(result);
                    } else if result == self.min {
                        break;
                    }
                    result -= T::ONE;
                }
            }
        }
    }
}

fn geometric_random_nonzero_signeds_inclusive_range<T: PrimitiveSigned>(
    seed: Seed,
    min: T,
    max: T,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> GeometricRandomNonzeroSigneds<T> {
    assert!(min <= max);
    assert_ne!(abs_um_denominator, 0);
    let (numerator, denominator) = mean_to_p_with_min(T::ONE, abs_um_numerator, abs_um_denominator);
    GeometricRandomNonzeroSigneds {
        bs: random_bools(seed.fork("bs")),
        xs: weighted_random_bools(
            seed.fork("xs"),
            numerator,
            numerator.checked_add(denominator).unwrap(),
        ),
        min,
        max,
    }
}

/// Generates random signed integers from a modified geometric distribution.
#[derive(Clone, Debug)]
pub struct GeometricRandomSigneds<T: PrimitiveSigned> {
    bs: RandomBools,
    xs: WeightedRandomBools,
    min: T,
    max: T,
}

impl<T: PrimitiveSigned> Iterator for GeometricRandomSigneds<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            let mut result = T::ZERO;
            if self.bs.next().unwrap() {
                loop {
                    if self.xs.next().unwrap() {
                        if result == T::ZERO && self.bs.next().unwrap() {
                            break;
                        }
                        return Some(result);
                    } else if result == self.max {
                        break;
                    }
                    result += T::ONE;
                }
            } else {
                loop {
                    if self.xs.next().unwrap() {
                        if result == T::ZERO && self.bs.next().unwrap() {
                            break;
                        }
                        return Some(result);
                    } else if result == self.min {
                        break;
                    }
                    result -= T::ONE;
                }
            }
        }
    }
}

fn geometric_random_signed_inclusive_range_helper<T: PrimitiveSigned>(
    seed: Seed,
    min: T,
    max: T,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> GeometricRandomSigneds<T> {
    assert!(min <= max);
    assert_ne!(abs_um_denominator, 0);
    let (numerator, denominator) =
        mean_to_p_with_min(T::ZERO, abs_um_numerator, abs_um_denominator);
    GeometricRandomSigneds {
        bs: random_bools(seed.fork("bs")),
        xs: weighted_random_bools(
            seed.fork("xs"),
            numerator,
            numerator.checked_add(denominator).unwrap(),
        ),
        min,
        max,
    }
}

fn get_geometric_random_signed_from_inclusive_range_helper<T: PrimitiveSigned>(
    range_generator: &mut VariableRangeGenerator,
    min: T,
    max: T,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> T {
    assert!(min <= max);
    assert_ne!(abs_um_denominator, 0);
    let (n, denominator) = mean_to_p_with_min(T::ZERO, abs_um_numerator, abs_um_denominator);
    let d = n.checked_add(denominator).unwrap();
    loop {
        let mut result = T::ZERO;
        if range_generator.next_bool() {
            loop {
                if get_weighted_random_bool(range_generator, n, d) {
                    if result == T::ZERO && range_generator.next_bool() {
                        break;
                    }
                    return result;
                } else if result == max {
                    break;
                }
                result += T::ONE;
            }
        } else {
            loop {
                if get_weighted_random_bool(range_generator, n, d) {
                    if result == T::ZERO && range_generator.next_bool() {
                        break;
                    }
                    return result;
                } else if result == min {
                    break;
                }
                result -= T::ONE;
            }
        }
    }
}

/// Generates random negative signed integers in a range from a modified geometric distribution.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum GeometricRandomSignedRange<T: PrimitiveSigned> {
    NonNegative(GeometricRandomNaturalValues<T>),
    NonPositive(GeometricRandomNegativeSigneds<T>),
    BothSigns(GeometricRandomSigneds<T>),
}

impl<T: PrimitiveSigned> Iterator for GeometricRandomSignedRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            GeometricRandomSignedRange::NonNegative(ref mut xs) => xs.next(),
            GeometricRandomSignedRange::NonPositive(ref mut xs) => xs.next(),
            GeometricRandomSignedRange::BothSigns(ref mut xs) => xs.next(),
        }
    }
}

/// Generates random unsigned integers from a truncated geometric distribution.
///
/// With this distribution, the probability of a value being generated decreases as the value
/// increases. The probabilities $P(0), P(1), P(2), \ldots$ decrease in a geometric sequence; that's
/// where the "geometric" comes from. Unlike a true geometric distribution, this distribution is
/// truncated, meaning that values above `T::MAX` are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `um_numerator / um_denominator`. The unadjusted mean is what
/// the mean generated value would be if the distribution were not truncated. If $m_u$ is
/// significantly lower than `T::MAX`, which is usually the case, then it is very close to the
/// actual mean. The higher $m_u$ is, the more gently the probabilities drop; the lower it is, the
/// more quickly they drop. $m_u$ must be greater than zero. It may be arbitrarily high, but note
/// that the iteration time increases linearly with `um_numerator + um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[0, 2^W)$, where $W$ is the width of the type. Then we have
/// $$
/// P(n) \neq 0 \leftrightarrow n \in S,
/// $$
/// and whenever $n, n + 1 \in S$,
/// $$
/// \frac{P(n)}{P(n+1)} = \frac{m_u + 1}{m_u}.
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `um_numerator + um_denominator`.
///
/// # Panics
/// Panics if `um_numerator` or `um_denominator` are zero, or, if after being reduced to lowest
/// terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::geometric::geometric_random_unsigneds;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(geometric_random_unsigneds::<u64>(EXAMPLE_SEED, 1, 1), 10),
///     "[1, 0, 0, 3, 4, 4, 1, 0, 0, 1, ...]"
/// )
/// ```
///
/// # Further details
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p} - 1$, or $p = \frac{1}{m_u + 1}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^np}{1-(1-p)^{2^W}} & \text{if} \\quad 0 \\leq n < 2^W, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $W$ is the width of the type.
///
/// It's also useful to note that
/// $$
///     \lim_{W \to \infty} P(n) = (1-p)^np.
/// $$
pub fn geometric_random_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    assert_ne!(um_numerator, 0);
    geometric_random_natural_values_inclusive_range(
        seed,
        T::ZERO,
        T::MAX,
        um_numerator,
        um_denominator,
    )
}

/// Generates random positive unsigned integers from a truncated geometric distribution.
///
/// With this distribution, the probability of a value being generated decreases as the value
/// increases. The probabilities $P(1), P(2), P(3), \ldots$ decrease in a geometric sequence; that's
/// where the "geometric" comes from. Unlike a true geometric distribution, this distribution is
/// truncated, meaning that values above `T::MAX` are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `um_numerator / um_denominator`. The unadjusted mean is what
/// the mean generated value would be if the distribution were not truncated. If $m_u$ is
/// significantly lower than `T::MAX`, which is usually the case, then it is very close to the
/// actual mean. The higher $m_u$ is, the more gently the probabilities drop; the lower it is, the
/// more quickly they drop. $m_u$ must be greater than one. It may be arbitrarily high, but note
/// that the iteration time increases linearly with `um_numerator + um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[1, 2^W)$, where $W$ is the width of the type. Then we have
/// $$
/// P(n) \neq 0 \leftrightarrow n \in S
/// $$
/// and whenever $n, n + 1 \in S$,
/// $$
/// \frac{P(n)}{P(n+1)} = \frac{m_u}{m_u - 1}.
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `um_numerator + um_denominator`.
///
/// # Panics
/// Panics if `um_denominator` is zero or if `um_numerator <= um_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::geometric::geometric_random_positive_unsigneds;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         geometric_random_positive_unsigneds::<u64>(EXAMPLE_SEED, 2, 1),
///         10
///     ),
///     "[2, 1, 1, 4, 5, 5, 2, 1, 1, 2, ...]"
/// )
/// ```
///
/// # Further details
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p}$, or $p = \frac{1}{m_u}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^{n-1}p}{1-(1-p)^{2^W-1}} & \text{if} \\quad 0 < n < 2^W, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $W$ is the width of the type.
///
/// It's also useful to note that
/// $$
///     \lim_{W \to \infty} P(n) = (1-p)^{n-1}p.
/// $$
pub fn geometric_random_positive_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    assert!(um_numerator > um_denominator);
    geometric_random_natural_values_inclusive_range(
        seed,
        T::ONE,
        T::MAX,
        um_numerator,
        um_denominator,
    )
}

/// Generates random signed integers from a modified geometric distribution.
///
/// This distribution can be derived from a truncated geometric distribution by mirroring it,
/// producing a truncated double geometric distribution. Zero is included.
///
/// With this distribution, the probability of a value being generated decreases as its absolute
/// value increases. The probabilities $P(0), P(\pm 1), P(\pm 2), \ldots$ decrease in a geometric
/// sequence; that's where the "geometric" comes from. Values below `T::MIN` or above `T::MAX` are
/// never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `abs_um_numerator / abs_um_denominator`. The unadjusted mean
/// is what the mean generated value would be if the distribution were not truncated, and were
/// restricted to non-negative values. If $m_u$ is significantly lower than `T::MAX`, which is
/// usually the case, then it is very close to the actual mean of the distribution restricted to
/// positive values. The higher $m_u$ is, the more gently the probabilities drop; the lower it is,
/// the more quickly they drop. $m_u$ must be greater than zero. It may be arbitrarily high, but
/// note that the iteration time increases linearly with `abs_um_numerator + abs_um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[-2^{W-1}, 2^{W-1})$, where $W$ is the width of the type. Then we have
/// $$
/// P(n) \neq 0 \leftrightarrow n \in S
/// $$
/// Whenever $n \geq 0$ and $n, n + 1 \in S$,
/// $$
/// \frac{P(n)}{P(n+1)} = \frac{m_u}{m_u - 1},
/// $$
/// and whenever $n \leq 0$ and $n, n - 1 \in S$,
/// $$
/// \frac{P(n)}{P(n-1)} = \frac{m_u}{m_u - 1}.
/// $$
///
/// As a corollary, $P(n) = P(-n)$ whenever $n, -n \in S$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `abs_um_numerator + abs_um_denominator`.
///
/// # Panics
/// Panics if `abs_um_numerator` or `abs_um_denominator` are zero, or, if after being reduced to
/// lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::geometric::geometric_random_signeds;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(geometric_random_signeds::<i64>(EXAMPLE_SEED, 1, 1), 10),
///     "[-1, -1, -1, 1, -2, 1, 0, 0, 0, 0, ...]"
/// )
/// ```
///
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p} - 1$, or $p = \frac{1}{m_u + 1}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^{|n|}p}{((1-p)^{2^{W-1}}-1)(p-2)} &
///         \text{if} \\quad -2^{W-1} \leq n < 2^{W-1}, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $W$ is the width of the type.
///
/// It's also useful to note that
/// $$
/// \lim_{W \to \infty} P(n) = \frac{(1-p)^{|n|}p}{2-p}.
/// $$
pub fn geometric_random_signeds<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> GeometricRandomSigneds<T> {
    assert_ne!(abs_um_numerator, 0);
    geometric_random_signed_inclusive_range_helper(
        seed,
        T::MIN,
        T::MAX,
        abs_um_numerator,
        abs_um_denominator,
    )
}

/// Generates random natural (non-negative) signed integers from a truncated geometric distribution.
///
/// With this distribution, the probability of a value being generated decreases as the value
/// increases. The probabilities $P(0), P(1), P(2), \ldots$ decrease in a geometric sequence; that's
/// where the "geometric" comes from. Unlike a true geometric distribution, this distribution is
/// truncated, meaning that values above `T::MAX` are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `um_numerator / um_denominator`. The unadjusted mean is what
/// the mean generated value would be if the distribution were not truncated. If $m_u$ is
/// significantly lower than `T::MAX`, which is usually the case, then it is very close to the
/// actual mean. The higher $m_u$ is, the more gently the probabilities drop; the lower it is, the
/// more quickly they drop. $m_u$ must be greater than zero. It may be arbitrarily high, but note
/// that the iteration time increases linearly with `um_numerator + um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[0, 2^{W-1})$, where $W$ is the width of the type. Then we have
/// $$
///     P(n) \neq 0 \leftrightarrow n \in S
/// $$
/// and whenever $n, n + 1 \in S$,
/// $$
/// \frac{P(n)}{P(n+1)} = \frac{m_u + 1}{m_u}.
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `um_numerator + um_denominator`.
///
/// # Panics
/// Panics if `um_numerator` or `um_denominator` are zero, or, if after being reduced to lowest
/// terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::geometric::geometric_random_natural_signeds;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         geometric_random_natural_signeds::<i64>(EXAMPLE_SEED, 1, 1),
///         10
///     ),
///     "[1, 0, 0, 3, 4, 4, 1, 0, 0, 1, ...]"
/// )
/// ```
///
/// # Further details
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p} - 1$, or $p = \frac{1}{m_u + 1}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^np}{1-(1-p)^{2^{W-1}}} & \text{if} \\quad 0 \\leq n < 2^{W-1}, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $W$ is the width of the type.
///
/// It's also useful to note that
/// $$
/// \lim_{W \to \infty} P(n) = \\begin{cases}
///     (1-p)^np & \text{if} \\quad n \geq 0, \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
pub fn geometric_random_natural_signeds<T: PrimitiveSigned>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    assert_ne!(um_numerator, 0);
    geometric_random_natural_values_inclusive_range(
        seed,
        T::ZERO,
        T::MAX,
        um_numerator,
        um_denominator,
    )
}

/// Generates random positive signed integers from a truncated geometric distribution.
///
/// With this distribution, the probability of a value being generated decreases as the value
/// increases. The probabilities $P(1), P(2), P(3), \ldots$ decrease in a geometric sequence; that's
/// where the "geometric" comes from. Unlike a true geometric distribution, this distribution is
/// truncated, meaning that values above `T::MAX` are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `um_numerator / um_denominator`. The unadjusted mean is what
/// the mean generated value would be if the distribution were not truncated. If $m_u$ is
/// significantly lower than `T::MAX`, which is usually the case, then it is very close to the
/// actual mean. The higher $m_u$ is, the more gently the probabilities drop; the lower it is, the
/// more quickly they drop. $m_u$ must be greater than one. It may be arbitrarily high, but note
/// that the iteration time increases linearly with `um_numerator + um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[1, 2^{W-1})$, where $W$ is the width of the type. Then we have
/// $$
///     P(n) \neq 0 \leftrightarrow n \in S
/// $$
///
/// and whenever $n, n + 1 \in S$,
/// $$
/// \frac{P(n)}{P(n+1)} = \frac{m_u}{m_u - 1}.
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `um_numerator + um_denominator`.
///
/// # Panics
/// Panics if `um_denominator` is zero or if `um_numerator <= um_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::geometric::geometric_random_positive_signeds;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         geometric_random_positive_signeds::<i64>(EXAMPLE_SEED, 2, 1),
///         10
///     ),
///     "[2, 1, 1, 4, 5, 5, 2, 1, 1, 2, ...]"
/// )
/// ```
///
/// # Further details
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p}$, or $p = \frac{1}{m_u}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^{n-1}p}{1-(1-p)^{2^{W-1}-1}} & \text{if} \\quad 0 < n < 2^{W-1}, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $W$ is the width of the type.
///
/// It's also useful to note that
/// $$
/// \lim_{W \to \infty} P(n) = \\begin{cases}
///     (1-p)^{n-1}p & \text{if} \\quad n > 0, \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
pub fn geometric_random_positive_signeds<T: PrimitiveSigned>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    geometric_random_natural_values_inclusive_range(
        seed,
        T::ONE,
        T::MAX,
        um_numerator,
        um_denominator,
    )
}

/// Generates random negative signed integers from a modified geometric distribution.
///
/// This distribution can be derived from a truncated geometric distribution by negating its domain.
/// The distribution is truncated at `T::MIN`.
///
/// With this distribution, the probability of a value being generated decreases as its absolute
/// value increases. The probabilities $P(-1), P(-2), P(-3), \ldots$ decrease in a geometric
/// sequence; that's where the "geometric" comes from. Values below `T::MIN` are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `abs_um_numerator / abs_um_denominator`. The unadjusted mean
/// is what the mean of the absolute values of the generated values would be if the distribution
/// were not truncated. If $m_u$ is significantly lower than `-T::MIN`, which is usually the case,
/// then it is very close to the actual mean of the absolute values. The higher $m_u$ is, the more
/// gently the probabilities drop; the lower it is, the more quickly they drop. $m_u$ must be
/// greater than one. It may be arbitrarily high, but note that the iteration time increases
/// linearly with `abs_um_numerator + abs_um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[-2^{W-1}, 0)$, where $W$ is the width of the type. Then we have
/// $$
///     P(n) \neq 0 \leftrightarrow n \in S
/// $$
///
/// and whenever $n, n - 1 \in S$,
/// $$
/// \frac{P(n)}{P(n-1)} = \frac{m_u}{m_u - 1}.
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `abs_um_numerator + abs_um_denominator`.
///
/// # Panics
/// Panics if `abs_um_denominator` is zero or if `abs_um_numerator <= abs_um_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::geometric::geometric_random_negative_signeds;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         geometric_random_negative_signeds::<i64>(EXAMPLE_SEED, 2, 1),
///         10
///     ),
///     "[-2, -1, -1, -4, -5, -5, -2, -1, -1, -2, ...]"
/// )
/// ```
///
/// # Further details
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p}$, or $p = \frac{1}{m_u}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^{-n-1}p}{1-(1-p)^{2^{W-1}}} & \text{if} \\quad -2^{W-1} \leq n < 0, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $W$ is the width of the type.
///
/// It's also useful to note that
/// $$
/// \lim_{W \to \infty} P(n) = \\begin{cases}
///     (1-p)^{-n-1}p & \text{if} \\quad n < 0, \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
pub fn geometric_random_negative_signeds<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> GeometricRandomNegativeSigneds<T> {
    assert!(abs_um_numerator > abs_um_denominator);
    geometric_random_negative_signeds_inclusive_range(
        seed,
        T::NEGATIVE_ONE,
        T::MIN,
        abs_um_numerator,
        abs_um_denominator,
    )
}

/// Generates random nonzero signed integers from a modified geometric distribution.
///
/// This distribution can be derived from a truncated geometric distribution by mirroring it,
/// producing a truncated double geometric distribution. Zero is excluded.
///
/// With this distribution, the probability of a value being generated decreases as its absolute
/// value increases. The probabilities $P(\pm 1), P(\pm 2), P(\pm 3), \ldots$ decrease in a
/// geometric sequence; that's where the "geometric" comes from. Values below `T::MIN` or above
/// `T::MAX` are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `abs_um_numerator / abs_um_denominator`. The unadjusted mean
/// is what the mean of the absolute values of the generated values would be if the distribution
/// were not truncated. If $m_u$ is significantly lower than `T::MAX`, which is usually the case,
/// then it is very close to the actual mean of the absolute values. The higher $m_u$ is, the more
/// gently the probabilities drop; the lower it is, the more quickly they drop. $m_u$ must be
/// greater than one. It may be arbitrarily high, but note that the iteration time increases
/// linearly with `abs_um_numerator + abs_um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[-2^{W-1}, 2^{W-1}) \setminus \\{0\\}$, where $W$ is the width of the type. Then we have
/// $$
/// P(n) \neq 0 \leftrightarrow n \in S
/// $$
/// $$
/// P(1) = P(-1)
/// $$
/// Whenever $n > 0$ and $n, n + 1 \in S$,
/// $$
/// \frac{P(n)}{P(n+1)} = \frac{m_u}{m_u - 1},
/// $$
/// and whenever $n < 0$ and $n, n - 1 \in S$,
/// $$
/// \frac{P(n)}{P(n-1)} = \frac{m_u}{m_u - 1}.
/// $$
///
/// As a corollary, $P(n) = P(-n)$ whenever $n, -n \in S$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `abs_um_numerator + abs_um_denominator`.
///
/// # Panics
/// Panics if `abs_um_denominator` is zero or if `abs_um_numerator <= abs_um_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::geometric::geometric_random_nonzero_signeds;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         geometric_random_nonzero_signeds::<i64>(EXAMPLE_SEED, 2, 1),
///         10
///     ),
///     "[-2, -2, -2, 2, -3, 2, -1, -1, -1, 1, ...]"
/// )
/// ```
///
/// # Further details
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p}$, or $p = \frac{1}{m_u}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^{|n|}p}{(1-p)^{2^{W-1}}(p-2)-2p+2} &
///         \text{if} \\quad -2^{W-1} \leq n < 0 \\ \mathrm{or} \\ 0 < n < -2^{W-1}, \\\\
///     0 & \\text{otherwise},
/// \\end{cases}
/// $$
/// where $W$ is the width of the type.
///
/// It's also useful to note that
/// $$
/// \lim_{W \to \infty} P(n) = \\begin{cases}
///     \frac{(1-p)^{|n|}p}{2-2p} & \text{if} \\quad n \neq 0, \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
pub fn geometric_random_nonzero_signeds<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> GeometricRandomNonzeroSigneds<T> {
    assert!(abs_um_numerator > abs_um_denominator);
    geometric_random_nonzero_signeds_inclusive_range(
        seed,
        T::MIN,
        T::MAX,
        abs_um_numerator,
        abs_um_denominator,
    )
}

/// Generates random unsigned integers from a truncated geometric distribution over the half-open
/// interval $[a, b)$.
///
/// With this distribution, the probability of a value being generated decreases as the value
/// increases. The probabilities $P(a), P(a + 1), P(a + 2), \ldots$ decrease in a geometric
/// sequence; that's where the "geometric" comes from. Unlike a true geometric distribution, this
/// distribution is truncated, meaning that values above $b$ are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `um_numerator / um_denominator`. The unadjusted mean is what
/// the mean generated value would be if the distribution were not truncated. If $m_u$ is
/// significantly lower than $b$, then it is very close to the actual mean. The higher $m_u$ is, the
/// more gently the probabilities drop; the lower it is, the more quickly they drop. $m_u$ must be
/// greater than $a$. It may be arbitrarily high, but note that the iteration time increases
/// linearly with `um_numerator + um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[a, b)$. Then we have
/// $$
/// P(n) \neq 0 \leftrightarrow n \in S
/// $$
///
/// and whenever $n, n + 1 \in S$,
/// $$
/// \frac{P(n)}{P(n+1)} = \frac{m_u + 1}{m_u}.
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `um_numerator + um_denominator`.
///
/// # Panics
/// Panics if $a \geq b$, if `um_numerator` or `um_denominator` are zero, if their ratio is less
/// than or equal to $a$, or if they are too large and manipulating them leads to arithmetic
/// overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::geometric::geometric_random_unsigned_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         geometric_random_unsigned_range::<u16>(EXAMPLE_SEED, 1, 7, 3, 1),
///         10
///     ),
///     "[2, 5, 2, 3, 4, 2, 5, 6, 1, 2, ...]"
/// )
/// ```
///
/// # Further details
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p} + a - 1$, or $p = \frac{1}{m_u - a + 1}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^np}{(1-p)^a-(1-p)^b} & \text{if} \\quad a \\leq n < b, \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
#[inline]
pub fn geometric_random_unsigned_range<T: PrimitiveUnsigned>(
    seed: Seed,
    a: T,
    b: T,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    assert!(a < b, "a must be less than b. a: {a}, b: {b}");
    geometric_random_natural_values_inclusive_range(
        seed,
        a,
        b - T::ONE,
        um_numerator,
        um_denominator,
    )
}

/// Generates random unsigned integers from a truncated geometric distribution over the closed
/// interval $[a, b]$.
///
/// With this distribution, the probability of a value being generated decreases as the value
/// increases. The probabilities $P(a), P(a + 1), P(a + 2), \ldots$ decrease in a geometric
/// sequence; that's where the "geometric" comes from. Unlike a true geometric distribution, this
/// distribution is truncated, meaning that values above $b$ are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `um_numerator / um_denominator`. The unadjusted mean is what
/// the mean generated value would be if the distribution were not truncated. If $m_u$ is
/// significantly lower than $b$, then it is very close to the actual mean. The higher $m_u$ is, the
/// more gently the probabilities drop; the lower it is, the more quickly they drop. $m_u$ must be
/// greater than $a$. It may be arbitrarily high, but note that the iteration time increases
/// linearly with `um_numerator + um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[a, b]$. Then we have
/// $$
/// P(n) \neq 0 \leftrightarrow n \in S
/// $$
///
/// and whenever $n, n + 1 \in S$,
/// $$
/// \frac{P(n)}{P(n+1)} = \frac{m_u + 1}{m_u}.
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `um_numerator + um_denominator`.
///
/// # Panics
/// Panics if $a > b$, if `um_numerator` or `um_denominator` are zero, if their ratio is less than
/// or equal to $a$, or if they are too large and manipulating them leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::geometric::geometric_random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         geometric_random_unsigned_inclusive_range::<u16>(EXAMPLE_SEED, 1, 6, 3, 1),
///         10
///     ),
///     "[2, 5, 2, 3, 4, 2, 5, 6, 1, 2, ...]"
/// )
/// ```
///
/// # Further details
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p} + a - 1$, or $p = \frac{1}{m_u - a + 1}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^np}{(1-p)^a-(1-p)^{b+1}} & \text{if} \\quad a \\leq n \\leq b, \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
#[inline]
pub fn geometric_random_unsigned_inclusive_range<T: PrimitiveUnsigned>(
    seed: Seed,
    a: T,
    b: T,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    geometric_random_natural_values_inclusive_range(seed, a, b, um_numerator, um_denominator)
}

/// Generates random signed integers from a modified geometric distribution over the half-open
/// interval $[a, b)$.
///
/// With this distribution, the probability of a value being generated decreases as its absolute
/// value increases. The probabilities $P(n), P(n + \operatorname{sgn}(n)), P(n +
/// 2\operatorname{sgn}(n)), \ldots$, where $n, n + \operatorname{sgn}(n), n +
/// 2\operatorname{sgn}(n), \ldots \in [a, b) \\setminus \\{0\\}$, decrease in a geometric sequence;
/// that's where the "geometric" comes from.
///
/// The form of the distribution depends on the range. If $a \geq 0$, the distribution is highest at
/// $a$ and is truncated at $b$. If $b \leq 1$, the distribution is reflected: it is highest at $b -
/// 1$ and is truncated at $a$. Otherwise, the interval includes both positive and negative values.
/// In that case the distribution is doubled: it is highest at zero and is truncated at $a$ and $b$.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `abs_um_numerator / abs_um_denominator`. The unadjusted mean
/// is what the mean of the absolute values of the generated values would be if the distribution
/// were not truncated. If $m_u$ is significantly lower than $b$, then it is very close to the
/// actual mean of the absolute values. The higher $m_u$ is, the more gently the probabilities drop;
/// the lower it is, the more quickly they drop. $m_u$ must be greater than $a$. It may be
/// arbitrarily high, but note that the iteration time increases linearly with `abs_um_numerator +
/// abs_um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[a, b)$. Let $c = \min_{n\in S}|n|$. Geometric distributions are typically parametrized
/// by a parameter $p$. The relationship between $p$ and $m_u$ is $m_u = \frac{1}{p} + c - 1$, or $p
/// = \frac{1}{m_u - c + 1}$. Then we have
/// $$
/// P(n) \neq 0 \leftrightarrow n \in S
/// $$
/// If $0, 1 \in S$, then
/// $$
/// \frac{P(0)}{P(1)} = \frac{m_u + 1}{m_u}.
/// $$
/// If $-1, 0 \in S$, then
/// $$
/// \frac{P(0)}{P(-1)} = \frac{m_u + 1}{m_u}.
/// $$
/// and whenever $n, n + \operatorname{sgn}(n) \in S \setminus \\{0\\}$,
/// $$
/// \frac{P(n)}{P(n+\operatorname{sgn}(n))} = \frac{m_u + 1}{m_u}.
/// $$
///
/// As a corollary, $P(n) = P(-n)$ whenever $n, -n \in S$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `um_numerator + um_denominator`.
///
/// # Panics
/// Panics if $a \geq b$, if `um_numerator` or `um_denominator` are zero, if their ratio is less
/// than or equal to $a$, or if they are too large and manipulating them leads to arithmetic
/// overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::geometric::geometric_random_signed_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         geometric_random_signed_range::<i8>(EXAMPLE_SEED, -100, 100, 30, 1),
///         10
///     ),
///     "[-32, -31, -88, 52, -40, 64, -36, -1, -7, 46, ...]"
/// )
/// ```
///
/// # Further details
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^np}{(1-p)^a-(1-p)^b} & \text{if} \\quad 0 \\leq a \\leq n < b, \\\\
///     \frac{(1-p)^{-n}p}{(1-p)^{1-b}-(1-p)^{1-a}} & \text{if} \\quad a \\leq n < b \\leq 1, \\\\
///     \frac{(1-p)^{|n|}p}{2-p-(1-p)^{1-a}-(1-p)^b} &
///         \text{if} \\quad a < 0 < 1 < b \\ \mathrm{and} \\ a \\leq n < b, \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
#[inline]
pub fn geometric_random_signed_range<T: PrimitiveSigned>(
    seed: Seed,
    a: T,
    b: T,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> GeometricRandomSignedRange<T> {
    assert!(a < b, "a must be less than b. a: {a}, b: {b}");
    if a >= T::ZERO {
        GeometricRandomSignedRange::NonNegative(geometric_random_natural_values_inclusive_range(
            seed,
            a,
            b - T::ONE,
            abs_um_numerator,
            abs_um_denominator,
        ))
    } else if b <= T::ONE {
        GeometricRandomSignedRange::NonPositive(geometric_random_negative_signeds_inclusive_range(
            seed,
            b - T::ONE,
            a,
            abs_um_numerator,
            abs_um_denominator,
        ))
    } else {
        GeometricRandomSignedRange::BothSigns(geometric_random_signed_inclusive_range_helper(
            seed,
            a,
            b - T::ONE,
            abs_um_numerator,
            abs_um_denominator,
        ))
    }
}

/// Generates random signed integers from a modified geometric distribution over the closed interval
/// $[a, b]$.
///
/// With this distribution, the probability of a value being generated decreases as its absolute
/// value increases. The probabilities $P(n), P(n + \operatorname{sgn}(n)), P(n +
/// 2\operatorname{sgn}(n)), \ldots$, where $n, n + \operatorname{sgn}(n), n +
/// 2\operatorname{sgn}(n), \ldots \in [a, b] \\setminus \\{0\\}$, decrease in a geometric sequence;
/// that's where the "geometric" comes from.
///
/// The form of the distribution depends on the range. If $a \geq 0$, the distribution is highest at
/// $a$ and is truncated at $b$. If $b \leq 0$, the distribution is reflected: it is highest at $b$
/// and is truncated at $a$. Otherwise, the interval includes both positive and negative values. In
/// that case the distribution is doubled: it is highest at zero and is truncated at $a$ and $b$.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `abs_um_numerator / abs_um_denominator`. The unadjusted mean
/// is what the mean of the absolute values of the generated values would be if the distribution
/// were not truncated. If $m_u$ is significantly lower than $b$, then it is very close to the
/// actual mean of the absolute values. The higher $m_u$ is, the more gently the probabilities drop;
/// the lower it is, the more quickly they drop. $m_u$ must be greater than $a$. It may be
/// arbitrarily high, but note that the iteration time increases linearly with `abs_um_numerator +
/// abs_um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[a, b]$. Let $c = \min_{n\in S}|n|$. Geometric distributions are typically parametrized
/// by a parameter $p$. The relationship between $p$ and $m_u$ is $m_u = \frac{1}{p} + c - 1$, or $p
/// = \frac{1}{m_u - c + 1}$. Then we have
/// $$
/// P(n) \neq 0 \leftrightarrow n \in S
/// $$
/// If $0, 1 \in S$, then
/// $$
/// \frac{P(0)}{P(1)} = \frac{m_u + 1}{m_u}.
/// $$
/// If $-1, 0 \in S$, then
/// $$
/// \frac{P(0)}{P(-1)} = \frac{m_u + 1}{m_u}.
/// $$
/// and whenever $n, n + \operatorname{sgn}(n) \in S \setminus \\{0\\}$,
/// $$
/// \frac{P(n)}{P(n+\operatorname{sgn}(n))} = \frac{m_u + 1}{m_u}.
/// $$
///
/// As a corollary, $P(n) = P(-n)$ whenever $n, -n \in S$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `um_numerator + um_denominator`.
///
/// # Panics
/// Panics if $a > b$, if `um_numerator` or `um_denominator` are zero, if their ratio is less than
/// or equal to $a$, or if they are too large and manipulating them leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::geometric::geometric_random_signed_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     prefix_to_string(
///         geometric_random_signed_inclusive_range::<i8>(EXAMPLE_SEED, -100, 99, 30, 1),
///         10
///     ),
///     "[-32, -31, -88, 52, -40, 64, -36, -1, -7, 46, ...]"
/// )
/// ```
///
/// # Further details
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^np}{(1-p)^a-(1-p)^{b+1}} & \text{if} \\quad 0 \\leq a \\leq n \\leq b, \\\\
///     \frac{(1-p)^{-n}p}{(1-p)^{-b}-(1-p)^{1-a}}
///         & \text{if} \\quad a \\leq n \\leq b \\leq 0, \\\\
///     \frac{(1-p)^{|n|}p}{2-p-(1-p)^{1-a}-(1-p)^{b+1}}
///         & \text{if} \\quad a < 0 < b \\ \mathrm{and} \\ a \\leq n \\leq b, \\\\
///     0 & \\text{otherwise}.
/// \\end{cases}
/// $$
#[inline]
pub fn geometric_random_signed_inclusive_range<T: PrimitiveSigned>(
    seed: Seed,
    a: T,
    b: T,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> GeometricRandomSignedRange<T> {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    if a >= T::ZERO {
        GeometricRandomSignedRange::NonNegative(geometric_random_natural_values_inclusive_range(
            seed,
            a,
            b,
            abs_um_numerator,
            abs_um_denominator,
        ))
    } else if b <= T::ZERO {
        GeometricRandomSignedRange::NonPositive(geometric_random_negative_signeds_inclusive_range(
            seed,
            b,
            a,
            abs_um_numerator,
            abs_um_denominator,
        ))
    } else {
        GeometricRandomSignedRange::BothSigns(geometric_random_signed_inclusive_range_helper(
            seed,
            a,
            b,
            abs_um_numerator,
            abs_um_denominator,
        ))
    }
}

/// Generates a random signed integers from a modified geometric distribution over the closed
/// interval $[a, b]$.
///
/// See [`geometric_random_signed_inclusive_range`] for a detailed description of the distribution.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(1)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ = `um_numerator + um_denominator`.
///
/// # Panics
/// Panics if $a > b$, if `um_numerator` or `um_denominator` are zero, if their ratio is less than
/// or equal to $a$, or if they are too large and manipulating them leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::num::random::geometric::get_geometric_random_signed_from_inclusive_range;
/// use malachite_base::num::random::VariableRangeGenerator;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     get_geometric_random_signed_from_inclusive_range::<i8>(
///         &mut VariableRangeGenerator::new(EXAMPLE_SEED),
///         -100,
///         99,
///         30,
///         1
///     ),
///     8
/// )
/// ```
pub fn get_geometric_random_signed_from_inclusive_range<T: PrimitiveSigned>(
    range_generator: &mut VariableRangeGenerator,
    a: T,
    b: T,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> T {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    if a >= T::ZERO {
        get_geometric_random_natural_value_from_inclusive_range(
            range_generator,
            a,
            b,
            abs_um_numerator,
            abs_um_denominator,
        )
    } else if b <= T::ZERO {
        get_geometric_random_negative_signed_from_inclusive_range(
            range_generator,
            b,
            a,
            abs_um_numerator,
            abs_um_denominator,
        )
    } else {
        get_geometric_random_signed_from_inclusive_range_helper(
            range_generator,
            a,
            b,
            abs_um_numerator,
            abs_um_denominator,
        )
    }
}
