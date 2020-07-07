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

fn geometric_random_natural_values_range<T: PrimitiveInteger>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
    min: T,
    max: T,
) -> GeometricRandomNaturalValues<T> {
    let gcd = gcd(um_numerator, um_denominator);
    let (um_numerator, um_denominator) = (um_numerator / gcd, um_denominator / gcd);
    let um_numerator = um_numerator
        .checked_sub(
            um_denominator
                .checked_mul(CheckedInto::<u64>::checked_into(min).unwrap())
                .unwrap(),
        )
        .unwrap();
    GeometricRandomNaturalValues {
        xs: random_unsigneds_less_than(seed, um_numerator.checked_add(um_denominator).unwrap()),
        numerator: um_denominator,
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
    let gcd = gcd(abs_um_numerator, abs_um_denominator);
    let (abs_um_numerator, abs_um_denominator) = (abs_um_numerator / gcd, abs_um_denominator / gcd);
    let abs_um_numerator = abs_um_numerator
        .checked_sub(
            abs_um_denominator
                .checked_mul(
                    CheckedInto::<u64>::checked_into(abs_min.checked_neg().unwrap()).unwrap(),
                )
                .unwrap(),
        )
        .unwrap();
    GeometricRandomNegativeSigneds {
        xs: random_unsigneds_less_than(
            seed,
            abs_um_numerator.checked_add(abs_um_denominator).unwrap(),
        ),
        abs_numerator: abs_um_denominator,
        abs_min,
        abs_max,
    }
}

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

pub fn geometric_random_natural_signeds<T: PrimitiveSigned>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    geometric_random_natural_values_range(seed, um_numerator, um_denominator, T::ZERO, T::MAX)
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
