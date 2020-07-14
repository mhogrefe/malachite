use bools::random::{random_bools, RandomBools};
use num::arithmetic::traits::Parity;
use num::basic::integers::PrimitiveInteger;
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::CheckedInto;
use num::random::random_unsigneds_less_than;
use num::random::random_unsigneds_less_than::RandomUnsignedsLessThan;
use random::seed::Seed;

#[derive(Clone, Debug)]
pub struct GeometricRandomNaturalValues<T: PrimitiveInteger> {
    xs: RandomUnsignedsLessThan<u64>,
    numerator: u64,
    min: T,
    max: T,
}

impl<T: PrimitiveInteger> Iterator for GeometricRandomNaturalValues<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut failures = self.min;
        loop {
            if self.xs.next().unwrap() < self.numerator {
                return Some(failures);
            } else {
                // Wrapping to min is equivalent to restarting this function.
                if failures == self.max {
                    failures = self.min;
                } else {
                    failures += T::ONE;
                }
            }
        }
    }
}

//TODO use actual gcd
fn gcd(u: u64, v: u64) -> u64 {
    if u == v {
        u
    } else if u == 0 {
        v
    } else if v == 0 {
        u
    } else if u.even() {
        if v.odd() {
            gcd(u >> 1, v)
        } else {
            gcd(u >> 1, v >> 1) << 1
        }
    } else if v.even() {
        gcd(u, v >> 1)
    } else if u > v {
        gcd((u - v) >> 1, v)
    } else {
        gcd((v - u) >> 1, u)
    }
}

fn unadjusted_mean_to_p<T: PrimitiveInteger>(
    um_numerator: u64,
    um_denominator: u64,
    min: T,
) -> (u64, u64) {
    let gcd = gcd(um_numerator, um_denominator);
    let (um_numerator, um_denominator) = (um_numerator / gcd, um_denominator / gcd);
    let um_numerator = um_numerator
        .checked_sub(
            um_denominator
                .checked_mul(CheckedInto::<u64>::checked_into(min).unwrap())
                .unwrap(),
        )
        .unwrap();
    (
        um_denominator,
        um_numerator.checked_add(um_denominator).unwrap(),
    )
}

fn geometric_random_natural_values_range<T: PrimitiveInteger>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
    min: T,
    max: T,
) -> GeometricRandomNaturalValues<T> {
    assert!(min < max);
    assert_ne!(um_numerator, 0);
    assert_ne!(um_denominator, 0);
    let (numerator, denominator) = unadjusted_mean_to_p(um_numerator, um_denominator, min);
    GeometricRandomNaturalValues {
        xs: random_unsigneds_less_than(seed, denominator),
        numerator,
        min,
        max,
    }
}

#[derive(Clone, Debug)]
pub struct GeometricRandomNegativeSigneds<T: PrimitiveSigned> {
    xs: RandomUnsignedsLessThan<u64>,
    abs_numerator: u64,
    abs_min: T,
    abs_max: T,
}

impl<T: PrimitiveSigned> Iterator for GeometricRandomNegativeSigneds<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut result = self.abs_min;
        loop {
            if self.xs.next().unwrap() < self.abs_numerator {
                return Some(result);
            } else {
                // Wrapping to min is equivalent to restarting this function.
                if result == self.abs_max {
                    result = self.abs_min;
                } else {
                    result -= T::ONE;
                }
            }
        }
    }
}

fn geometric_random_negative_signeds_range<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
    abs_min: T,
    abs_max: T,
) -> GeometricRandomNegativeSigneds<T> {
    assert!(abs_min > abs_max);
    assert_ne!(abs_um_numerator, 0);
    assert_ne!(abs_um_denominator, 0);
    let (numerator, denominator) = unadjusted_mean_to_p(
        abs_um_numerator,
        abs_um_denominator,
        abs_min.checked_neg().unwrap(),
    );
    GeometricRandomNegativeSigneds {
        xs: random_unsigneds_less_than(seed, denominator),
        abs_numerator: numerator,
        abs_min,
        abs_max,
    }
}

#[derive(Clone, Debug)]
pub struct GeometricRandomNonzeroSigneds<T: PrimitiveSigned> {
    bs: RandomBools,
    xs: RandomUnsignedsLessThan<u64>,
    abs_numerator: u64,
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
                    if self.xs.next().unwrap() < self.abs_numerator {
                        return Some(result);
                    } else if result == self.max {
                        break;
                    } else {
                        result += T::ONE;
                    }
                }
            } else {
                let mut result = T::NEGATIVE_ONE;
                loop {
                    if self.xs.next().unwrap() < self.abs_numerator {
                        return Some(result);
                    } else if result == self.min {
                        break;
                    } else {
                        result -= T::ONE;
                    }
                }
            }
        }
    }
}

fn geometric_random_nonzero_signeds_range<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
    min: T,
    max: T,
) -> GeometricRandomNonzeroSigneds<T> {
    let (numerator, denominator) =
        unadjusted_mean_to_p(abs_um_numerator, abs_um_denominator, T::ONE);
    GeometricRandomNonzeroSigneds {
        bs: random_bools(seed.fork("bs")),
        xs: random_unsigneds_less_than(seed.fork("xs"), denominator),
        abs_numerator: numerator,
        min,
        max,
    }
}

#[derive(Clone, Debug)]
pub struct GeometricRandomSigneds<T: PrimitiveSigned> {
    bs: RandomBools,
    xs: RandomUnsignedsLessThan<u64>,
    abs_numerator: u64,
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
                    if self.xs.next().unwrap() < self.abs_numerator {
                        if result == T::ZERO && self.bs.next().unwrap() {
                            break;
                        } else {
                            return Some(result);
                        }
                    } else if result == self.max {
                        break;
                    } else {
                        result += T::ONE;
                    }
                }
            } else {
                loop {
                    if self.xs.next().unwrap() < self.abs_numerator {
                        if result == T::ZERO && self.bs.next().unwrap() {
                            break;
                        } else {
                            return Some(result);
                        }
                    } else if result == self.min {
                        break;
                    } else {
                        result -= T::ONE;
                    }
                }
            }
        }
    }
}

fn geometric_random_signeds_range<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
    min: T,
    max: T,
) -> GeometricRandomSigneds<T> {
    let (numerator, denominator) =
        unadjusted_mean_to_p(abs_um_numerator, abs_um_denominator, T::ZERO);
    GeometricRandomSigneds {
        bs: random_bools(seed.fork("bs")),
        xs: random_unsigneds_less_than(seed.fork("xs"), denominator),
        abs_numerator: numerator,
        min,
        max,
    }
}

/// Generates random unsigned integers from a truncated geometric distribution. The maximum value
/// is `T::MAX`. A geometric distribution is typically parametrized by a value p, such that the
/// probability P(n) of generating n is (1 - p)<sup>n</sup>p. Instead, this function accepts a value
/// called the "unadjusted mean"; the numerator and denominator of this value are accepted.
///
/// The unadjusted mean is what the mean of the distribution would be if the distribution weren't
/// truncated. If it is significantly lower than `T::MAX`, then it is very close to the actual mean.
/// It is related to the parameter p by m = 1 / p - 1, or p = 1 / (m + 1).
///
/// One way to characterize this distribution is that it is the unique distribution on [0, `T::MAX`]
/// such that P(n) / P(n + 1) = (m + 1) / m, where m = `um_numerator` / `um_denominator`.
///
/// Length is infinite.
///
/// Time per iteration: O(m)
///
/// Additional memory per iteration: O(1)
///
/// where m = `um_numerator` / `um_denominator`.
///
/// # Panics
/// Panics if `um_numerator` or `um_denominator` are zero, or if, after being reduced to lowest
/// terms, their sum is greater than or equal to 2<sup>64</sup>.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::geometric::geometric_random_unsigneds;
///
/// assert_eq!(
///     geometric_random_unsigneds::<u64>(EXAMPLE_SEED, 1, 1).take(10).collect::<Vec<u64>>(),
///     &[1, 0, 0, 3, 4, 4, 1, 0, 0, 1]
/// )
/// ```
pub fn geometric_random_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    geometric_random_natural_values_range(seed, um_numerator, um_denominator, T::ZERO, T::MAX)
}

pub fn geometric_random_positive_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    geometric_random_natural_values_range(seed, um_numerator, um_denominator, T::ONE, T::MAX)
}

pub fn geometric_random_signeds<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> GeometricRandomSigneds<T> {
    geometric_random_signeds_range(seed, abs_um_numerator, abs_um_denominator, T::MIN, T::MAX)
}

pub fn geometric_random_natural_signeds<T: PrimitiveSigned>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    geometric_random_natural_values_range(seed, um_numerator, um_denominator, T::ZERO, T::MAX)
}

pub fn geometric_random_positive_signeds<T: PrimitiveSigned>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    geometric_random_natural_values_range(seed, um_numerator, um_denominator, T::ONE, T::MAX)
}

pub fn geometric_random_negative_signeds<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> GeometricRandomNegativeSigneds<T> {
    geometric_random_negative_signeds_range(
        seed,
        abs_um_numerator,
        abs_um_denominator,
        T::NEGATIVE_ONE,
        T::MIN,
    )
}

pub fn geometric_random_nonzero_signeds<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> GeometricRandomNonzeroSigneds<T> {
    geometric_random_nonzero_signeds_range(
        seed,
        abs_um_numerator,
        abs_um_denominator,
        T::MIN,
        T::MAX,
    )
}
