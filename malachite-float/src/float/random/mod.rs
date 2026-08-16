// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use malachite_base::bools::random::{
    RandomBools, WeightedRandomBools, random_bools, weighted_random_bools,
};
use malachite_base::iterators::{WithSpecialValues, with_special_values};
use malachite_base::num::arithmetic::traits::{
    DivRound, IsPowerOf2, ModPowerOf2, NegAssign, NegModPowerOf2, PowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, LowMask, NotAssign, SignificantBits};
use malachite_base::num::random::geometric::{
    GeometricRandomNaturalValues, GeometricRandomSignedRange,
    geometric_random_signed_inclusive_range,
};
use malachite_base::num::random::{RandomPrimitiveInts, random_primitive_ints};
use malachite_base::random::Seed;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::natural::random::{
    RandomNaturals, StripedRandomNaturalInclusiveRange, StripedRandomNaturals,
    UniformRandomNaturalRange, get_random_natural_with_up_to_bits, random_positive_naturals,
    striped_random_natural_inclusive_range, striped_random_positive_naturals,
    uniform_random_natural_inclusive_range,
};
use malachite_nz::platform::Limb;

/// Generates random positive finite [`Float`]s.
///
/// This `struct` is created by [`random_positive_finite_floats`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomPositiveFiniteFloats<I: Iterator<Item = Natural>> {
    exponents: GeometricRandomSignedRange<i32>,
    xs: I,
}

impl<I: Iterator<Item = Natural>> Iterator for RandomPositiveFiniteFloats<I> {
    type Item = Float;

    fn next(&mut self) -> Option<Float> {
        let x = self.xs.next().unwrap();
        let precision = x.significant_bits();
        assert_ne!(precision, 0);
        Some(Float(Finite {
            sign: true,
            exponent: self.exponents.next().unwrap() + 1,
            precision,
            significand: x << precision.neg_mod_power_of_2(Limb::LOG_WIDTH),
        }))
    }
}

/// Generates random positive finite [`Float`]s.
///
/// Simpler [`Float`]s (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the mean absolute sci-exponent and precision by passing the numerators
/// and denominators of their means.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// Neither positive nor negative zero is generated. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n / m + 1)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::random_positive_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_positive_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "0.88#3",
///         "1.31e-6#6",
///         "0.0078#1",
///         "0.50#1",
///         "82144.0#13",
///         "0.01558827446#29",
///         "0.016#1",
///         "3.406#7",
///         "4.5981711652#33",
///         "0.000033432058#23",
///         "0.3392996773764#37",
///         "2.662e4#7",
///         "3.3e4#1",
///         "1.398#8",
///         "37.38#9",
///         "0.25#1",
///         "0.0011108#13",
///         "1066.0#10",
///         "0.1836#7",
///         "0.001332305612#28"
///     ]
/// );
/// ```
pub fn random_positive_finite_floats(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
) -> RandomPositiveFiniteFloats<RandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomPositiveFiniteFloats {
        exponents: geometric_random_signed_inclusive_range(
            seed.fork("exponents"),
            Float::MIN_EXPONENT,
            Float::MAX_EXPONENT,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
        ),
        xs: random_positive_naturals(
            seed.fork("significands"),
            mean_precision_numerator,
            mean_precision_denominator,
        ),
    }
}

/// Generates random positive finite [`Float`]s with a specified precision.
///
/// Simpler [`Float`]s (those with a lower absolute sci-exponent) are more likely to be chosen. You
/// can specify the mean absolute sci-exponent by passing the numerators and denominators of its
/// means.
///
/// But note that the specified mean is only approximate, since the distribution we are sampling is
/// truncated geometric, and its exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual mean is slightly lower than the specified mean.
/// - However, increasing the specified mean increases the actual mean, so this still works as a
///   mechanism for controlling the sci-exponent.
/// - The specified sci-exponent mean must be greater than 0, but it may be as high as you like.
///
/// Neither positive nor negative zero is generated. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
///
/// # Panics
/// Panics if `prec` is zero.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::random_positive_floats_with_precision;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_positive_floats_with_precision(EXAMPLE_SEED, 10, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "0.95898#10",
///         "1.8887e-6#10",
///         "0.012909#10",
///         "0.70996#10",
///         "1.0202e5#10",
///         "0.011810#10",
///         "0.019531#10",
///         "3.0820#10",
///         "7.2422#10",
///         "0.000055969#10",
///         "0.38770#10",
///         "21440.0#10",
///         "58560.0#10",
///         "1.4297#10",
///         "62.188#10",
///         "0.46582#10",
///         "0.0016594#10",
///         "1914.0#10",
///         "0.13599#10",
///         "0.0011444#10"
///     ]
/// );
/// ```
pub fn random_positive_floats_with_precision(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    prec: u64,
) -> RandomPositiveFiniteFloats<UniformRandomNaturalRange> {
    assert_ne!(prec, 0);
    RandomPositiveFiniteFloats {
        exponents: geometric_random_signed_inclusive_range(
            seed.fork("exponents"),
            Float::MIN_EXPONENT,
            Float::MAX_EXPONENT,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
        ),
        xs: uniform_random_natural_inclusive_range(
            seed.fork("significands"),
            Natural::power_of_2(prec - 1),
            Natural::low_mask(prec),
        ),
    }
}

/// Generates random negative finite [`Float`]s.
///
/// This `struct` is created by [`random_negative_finite_floats`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomNegativeFiniteFloats<I: Iterator<Item = Natural>>(RandomPositiveFiniteFloats<I>);

impl<I: Iterator<Item = Natural>> Iterator for RandomNegativeFiniteFloats<I> {
    type Item = Float;

    #[inline]
    fn next(&mut self) -> Option<Float> {
        self.0.next().map(|f| -f)
    }
}

/// Generates random negative finite [`Float`]s.
///
/// Simpler [`Float`]s (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the mean absolute sci-exponent and precision by passing the numerators
/// and denominators of their means.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// Neither positive nor negative zero is generated. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n / m + 1)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::random_negative_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_negative_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "-0.88#3",
///         "-1.31e-6#6",
///         "-0.0078#1",
///         "-0.50#1",
///         "-82144.0#13",
///         "-0.01558827446#29",
///         "-0.016#1",
///         "-3.406#7",
///         "-4.5981711652#33",
///         "-0.000033432058#23",
///         "-0.3392996773764#37",
///         "-2.662e4#7",
///         "-3.3e4#1",
///         "-1.398#8",
///         "-37.38#9",
///         "-0.25#1",
///         "-0.0011108#13",
///         "-1066.0#10",
///         "-0.1836#7",
///         "-0.001332305612#28"
///     ]
/// );
/// ```
#[inline]
pub fn random_negative_finite_floats(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
) -> RandomNegativeFiniteFloats<RandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomNegativeFiniteFloats(random_positive_finite_floats(
        seed,
        mean_sci_exponent_abs_numerator,
        mean_sci_exponent_abs_denominator,
        mean_precision_numerator,
        mean_precision_denominator,
    ))
}

/// Generates random non-negative finite [`Float`]s.
///
/// This `struct` is created by [`random_non_negative_finite_floats`]; see its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct RandomNonNegativeFiniteFloats<I: Iterator<Item = Natural>> {
    bs: WeightedRandomBools,
    xs: RandomPositiveFiniteFloats<I>,
}

impl<I: Iterator<Item = Natural>> Iterator for RandomNonNegativeFiniteFloats<I> {
    type Item = Float;

    #[inline]
    fn next(&mut self) -> Option<Float> {
        if self.bs.next().unwrap() {
            Some(Float::ZERO)
        } else {
            self.xs.next()
        }
    }
}

/// Generates random non-negative finite [`Float`]s.
///
/// Simpler [`Float`]s (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the numerator and denominator of the probability that a zero will be
/// generated. You can also specify the mean absolute sci-exponent and precision by passing the
/// numerators and denominators of their means of the nonzero [`Float`]s.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// Positive zero is generated, but negative zero is not. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n / m + 1)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::random_non_negative_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_non_negative_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "1.11e5#5",
///         "0.03108048#17",
///         "9.59386e6#14",
///         "0.0",
///         "0.0127#5",
///         "0.018433#11",
///         "2.00#5",
///         "3.0820#10",
///         "0.874954#16",
///         "10288.29527676#38",
///         "9.2188#10",
///         "0.030048549#23",
///         "311.4521#19",
///         "0.0",
///         "1072.0#7",
///         "0.0009651#9",
///         "59159.52197#27",
///         "0.0",
///         "0.0000353#6",
///         "16.0#1"
///     ]
/// );
/// ```
#[inline]
pub fn random_non_negative_finite_floats(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    zero_p_numerator: u64,
    zero_p_denominator: u64,
) -> RandomNonNegativeFiniteFloats<RandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomNonNegativeFiniteFloats {
        bs: weighted_random_bools(seed.fork("bs"), zero_p_numerator, zero_p_denominator),
        xs: random_positive_finite_floats(
            seed.fork("xs"),
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
        ),
    }
}

/// Generates random non-positive finite [`Float`]s.
///
/// This `struct` is created by [`random_non_positive_finite_floats`]; see its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct RandomNonPositiveFiniteFloats<I: Iterator<Item = Natural>> {
    bs: WeightedRandomBools,
    xs: RandomNegativeFiniteFloats<I>,
}

impl<I: Iterator<Item = Natural>> Iterator for RandomNonPositiveFiniteFloats<I> {
    type Item = Float;

    #[inline]
    fn next(&mut self) -> Option<Float> {
        if self.bs.next().unwrap() {
            Some(Float::NEGATIVE_ZERO)
        } else {
            self.xs.next()
        }
    }
}

/// Generates random non-positive finite [`Float`]s.
///
/// Simpler [`Float`]s (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the numerator and denominator of the probability that a zero will be
/// generated. You can also specify the mean absolute sci-exponent and precision by passing the
/// numerators and denominators of their means of the nonzero [`Float`]s.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// Negative zero is generated, but positive zero is not. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n / m + 1)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::random_non_positive_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_non_positive_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "-1.11e5#5",
///         "-0.03108048#17",
///         "-9.59386e6#14",
///         "-0.0",
///         "-0.0127#5",
///         "-0.018433#11",
///         "-2.00#5",
///         "-3.0820#10",
///         "-0.874954#16",
///         "-10288.29527676#38",
///         "-9.2188#10",
///         "-0.030048549#23",
///         "-311.4521#19",
///         "-0.0",
///         "-1072.0#7",
///         "-0.0009651#9",
///         "-59159.52197#27",
///         "-0.0",
///         "-0.0000353#6",
///         "-16.0#1"
///     ]
/// );
/// ```
#[inline]
pub fn random_non_positive_finite_floats(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    zero_p_numerator: u64,
    zero_p_denominator: u64,
) -> RandomNonPositiveFiniteFloats<RandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomNonPositiveFiniteFloats {
        bs: weighted_random_bools(seed.fork("bs"), zero_p_numerator, zero_p_denominator),
        xs: random_negative_finite_floats(
            seed.fork("xs"),
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
        ),
    }
}

/// Generates random nonzero finite [`Float`]s.
///
/// This `struct` is created by [`random_nonzero_finite_floats`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomNonzeroFiniteFloats<I: Iterator<Item = Natural>> {
    bs: RandomBools,
    xs: RandomPositiveFiniteFloats<I>,
}

impl<I: Iterator<Item = Natural>> Iterator for RandomNonzeroFiniteFloats<I> {
    type Item = Float;

    #[inline]
    fn next(&mut self) -> Option<Float> {
        let x = self.xs.next().unwrap();
        Some(if self.bs.next().unwrap() { x } else { -x })
    }
}

/// Generates random nonzero finite [`Float`]s.
///
/// Simpler [`Float`]s (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the mean absolute sci-exponent and precision by passing the numerators
/// and denominators of their means.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// Neither positive nor negative zero is generated. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n / m + 1)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::random_nonzero_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_nonzero_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "-1.11e5#5",
///         "-0.03108048#17",
///         "-9.59386e6#14",
///         "0.0127#5",
///         "-0.018433#11",
///         "2.00#5",
///         "-3.0820#10",
///         "-0.874954#16",
///         "-10288.29527676#38",
///         "9.2188#10",
///         "0.030048549#23",
///         "311.4521#19",
///         "-1072.0#7",
///         "-0.0009651#9",
///         "59159.52197#27",
///         "-0.0000353#6",
///         "-16.0#1",
///         "-120.0#5",
///         "-960.0#5",
///         "-358.24023#20"
///     ]
/// );
/// ```
#[inline]
pub fn random_nonzero_finite_floats(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
) -> RandomNonzeroFiniteFloats<RandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomNonzeroFiniteFloats {
        bs: random_bools(seed.fork("bs")),
        xs: random_positive_finite_floats(
            seed.fork("xs"),
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
        ),
    }
}

/// Generates random finite [`Float`]s.
///
/// This `struct` is created by [`random_finite_floats`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomFiniteFloats<I: Iterator<Item = Natural>> {
    bs: RandomBools,
    xs: RandomNonNegativeFiniteFloats<I>,
}

impl<I: Iterator<Item = Natural>> Iterator for RandomFiniteFloats<I> {
    type Item = Float;

    #[inline]
    fn next(&mut self) -> Option<Float> {
        let x = self.xs.next().unwrap();
        Some(if self.bs.next().unwrap() { x } else { -x })
    }
}

/// Generates random finite [`Float`]s.
///
/// Simpler [`Float`]s (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the numerator and denominator of the probability that a zero will be
/// generated. You can also specify the mean absolute sci-exponent and precision by passing the
/// numerators and denominators of their means of the nonzero [`Float`]s.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// Positive zero and negative zero are both generated. `NaN` is not.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n / m + 1)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::random_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "-2.438#7",
///         "-2.3233958868e-8#30",
///         "-0.0859#6",
///         "1009.3770#20",
///         "-0.000824#6",
///         "1.9805#10",
///         "-1.9e-6#3",
///         "-268192.0#14",
///         "-0.00033855#10",
///         "6.0#2",
///         "0.0",
///         "0.102#5",
///         "-1.3665#13",
///         "-3.2e9#2",
///         "0.117#4",
///         "-0.19#2",
///         "-0.03003#7",
///         "-3.8e-6#2",
///         "-114.0#6",
///         "-4002.0#13"
///     ]
/// );
/// ```
#[inline]
pub fn random_finite_floats(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    zero_p_numerator: u64,
    zero_p_denominator: u64,
) -> RandomFiniteFloats<RandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomFiniteFloats {
        bs: random_bools(seed.fork("bs")),
        xs: random_non_negative_finite_floats(
            seed.fork("xs"),
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
            zero_p_numerator,
            zero_p_denominator,
        ),
    }
}

/// Generates random [`Float`]s.
///
/// Simpler [`Float`]s (those with a lower absolute sci-exponent or precision) are more likely to be
/// chosen. You can specify the numerator and denominator of the probability that a zero, an
/// infinity, or a NaN will be generated. You can also specify the mean absolute sci-exponent and
/// precision by passing the numerators and denominators of their means of the nonzero [`Float`]s.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The practical
/// implications are that
/// - The actual means are slightly lower than the specified means.
/// - However, increasing the specified means increases the actual means, so this still works as a
///   mechanism for controlling the sci-exponent and precision.
/// - The specified sci-exponent mean must be greater than 0 and the precision mean greater than 2,
///   but they may be as high as you like.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n / m + 1)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::random_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_floats(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///         .take(50)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "7.2031#10",
///         "39.25#8",
///         "0.0",
///         "NaN",
///         "-0.000031#2",
///         "-5.1e2#1",
///         "-0.08789#8",
///         "-95.12012#17",
///         "0.380768#14",
///         "0.000138037#15",
///         "-0.1094#7",
///         "-10.312#12",
///         "-13.683969005122592#51",
///         "Infinity",
///         "-0.344#4",
///         "-7.28e-12#5",
///         "-394584.0#16",
///         "NaN",
///         "13.5#5",
///         "-0.0",
///         "-0.00635#5",
///         "0.062#1",
///         "0.18933#12",
///         "0.0000401#6",
///         "-4.8189e-8#13",
///         "1.15e3#6",
///         "-1.914e7#7",
///         "475.7344#17",
///         "1.103e-6#7",
///         "Infinity",
///         "-24.0#3",
///         "-3.6e-15#1",
///         "-Infinity",
///         "0.50391#11",
///         "-1.0e3#3",
///         "-0.0000281#6",
///         "-2.0e5#2",
///         "6.4317792e-6#20",
///         "-0.000191#5",
///         "-0.0",
///         "-30.0#4",
///         "0.25#1",
///         "-0.006299376#18",
///         "4.582787718616e-6#38",
///         "-0.0002707085#19",
///         "0.000013128#10",
///         "NaN",
///         "-0.0",
///         "6.7e7#1",
///         "20263.5#16"
///     ]
/// );
/// ```
#[inline]
pub fn random_floats(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
) -> WithSpecialValues<RandomFiniteFloats<RandomNaturals<GeometricRandomNaturalValues<u64>>>> {
    with_special_values(
        seed,
        vec![Float::INFINITY, Float::NEGATIVE_INFINITY, Float::NAN],
        mean_special_p_numerator,
        mean_special_p_denominator,
        &|seed_2| {
            random_finite_floats(
                seed_2,
                mean_sci_exponent_abs_numerator,
                mean_sci_exponent_abs_denominator,
                mean_precision_numerator,
                mean_precision_denominator,
                mean_special_p_numerator,
                mean_special_p_denominator,
            )
        },
    )
}

/// Generates striped random positive finite [`Float`]s.
///
/// The actual precision is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_sci_exponent_abs_numerator / mean_sci_exponent_abs_denominator`; $m$ must be greater than
/// 0. A striped bit sequence with the given stripe parameter is generated and truncated at the bit
/// length. The highest bit is forced to be 1, and the [`Float`] is generated from the sequence and
/// a random sci-exponent.
///
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for information
/// about generating striped random numbers.
///
/// Neither positive nor negative zero is generated. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n / m + 1)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_precision_numerator` or `mean_precision_denominator` are
/// zero, or, if after being reduced to lowest terms, their sum is greater than or equal to
/// $2^{64}$.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::striped_random_positive_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_positive_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "0.938#4",
///         "1.9064e-6#11",
///         "0.0078#2",
///         "0.50#3",
///         "98332.000#21",
///         "0.014160633101709896512#60",
///         "0.023#2",
///         "2.109#8",
///         "4.000030282884437849#57",
///         "0.000057221276833275#43",
///         "0.25000005983747242139#63",
///         "24576.0#12",
///         "3.3e4#1",
///         "1.98431#16",
///         "33.500#12",
///         "0.25#1",
///         "0.00097680069#23",
///         "1279.50000#25",
///         "0.1250#7",
///         "0.0014648735386622#42"
///     ]
/// );
/// ```
pub fn striped_random_positive_finite_floats(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
) -> RandomPositiveFiniteFloats<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomPositiveFiniteFloats {
        exponents: geometric_random_signed_inclusive_range(
            seed.fork("exponents"),
            Float::MIN_EXPONENT,
            Float::MAX_EXPONENT,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
        ),
        xs: striped_random_positive_naturals(
            seed.fork("significands"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
        ),
    }
}

/// Generates striped random positive finite [`Float`]s with a specified precision.
///
/// A striped bit sequence with the given stripe parameter is generated and truncated at the bit
/// length. The highest bit is forced to be 1, and the [`Float`] is generated from the sequence and
/// a random sci-exponent.
///
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for information
/// about generating striped random numbers.
///
/// Neither positive nor negative zero is generated. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, or if `prec` is zero.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::striped_random_positive_floats_with_precision;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_positive_floats_with_precision(EXAMPLE_SEED, 10, 1, 8, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "0.81152#10",
///         "9.5367e-7#10",
///         "0.015610#10",
///         "0.50000#10",
///         "65536.0#10",
///         "0.015381#10",
///         "0.017548#10",
///         "3.9961#10",
///         "7.9922#10",
///         "0.000060976#10",
///         "0.44092#10",
///         "32736.0#10",
///         "64960.0#10",
///         "1.1250#10",
///         "63.938#10",
///         "0.29688#10",
///         "0.0019512#10",
///         "1920.0#10",
///         "0.12573#10",
///         "0.0014629#10"
///     ]
/// );
/// ```
pub fn striped_random_positive_floats_with_precision(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    prec: u64,
) -> RandomPositiveFiniteFloats<StripedRandomNaturalInclusiveRange> {
    assert_ne!(prec, 0);
    RandomPositiveFiniteFloats {
        exponents: geometric_random_signed_inclusive_range(
            seed.fork("exponents"),
            Float::MIN_EXPONENT,
            Float::MAX_EXPONENT,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
        ),
        xs: striped_random_natural_inclusive_range(
            seed.fork("significands"),
            Natural::power_of_2(prec - 1),
            Natural::low_mask(prec),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
    }
}

/// Generates striped random negative finite [`Float`]s.
///
/// The actual precision is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_stripe_numerator / mean_stripe_denominator`; $m$ must be greater than 0. A striped bit
/// sequence with the given stripe parameter is generated and truncated at the bit length. The
/// highest bit is forced to be 1, and the [`Float`] is generated from the sequence and a random
/// sci-exponent.
///
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for information
/// about generating striped random numbers.
///
/// Neither positive nor negative zero is generated. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n / m + 1)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_precision_numerator` or `mean_precision_denominator` are
/// zero, or, if after being reduced to lowest terms, their sum is greater than or equal to
/// $2^{64}$.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::striped_random_negative_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_negative_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "-0.938#4",
///         "-1.9064e-6#11",
///         "-0.0078#2",
///         "-0.50#3",
///         "-98332.000#21",
///         "-0.014160633101709896512#60",
///         "-0.023#2",
///         "-2.109#8",
///         "-4.000030282884437849#57",
///         "-0.000057221276833275#43",
///         "-0.25000005983747242139#63",
///         "-24576.0#12",
///         "-3.3e4#1",
///         "-1.98431#16",
///         "-33.500#12",
///         "-0.25#1",
///         "-0.00097680069#23",
///         "-1279.50000#25",
///         "-0.1250#7",
///         "-0.0014648735386622#42"
///     ]
/// );
/// ```
pub fn striped_random_negative_finite_floats(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
) -> RandomNegativeFiniteFloats<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomNegativeFiniteFloats(striped_random_positive_finite_floats(
        seed,
        mean_sci_exponent_abs_numerator,
        mean_sci_exponent_abs_denominator,
        mean_stripe_numerator,
        mean_stripe_denominator,
        mean_precision_numerator,
        mean_precision_denominator,
    ))
}

/// Generates striped random non-negative finite [`Float`]s.
///
/// Positive zero is generated with the specified probability. If the [`Float`] to be generated is
/// nonzero, then the actual precision is chosen from a geometric distribution with mean $m$, where
/// $m$ is `mean_stripe_numerator / mean_stripe_denominator`; $m$ must be greater than 0. A striped
/// bit sequence with the given stripe parameter is generated and truncated at the bit length. The
/// highest bit is forced to be 1, and the [`Float`] is generated from the sequence and a random
/// sci-exponent.
///
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for information
/// about generating striped random numbers.
///
/// Positive zero is generated, but negative zero is not. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n / m + 1)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_precision_numerator` or `mean_precision_denominator` are
/// zero, or, if after being reduced to lowest terms, their sum is greater than or equal to
/// $2^{64}$.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::striped_random_non_negative_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_non_negative_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "6.554e4#7",
///         "0.0214843750#26",
///         "8404960.0#19",
///         "0.0",
///         "0.0155065#16",
///         "0.031219512#20",
///         "3.94#6",
///         "2.00378#15",
///         "0.61712646#21",
///         "16383.978515147231406#61",
///         "12.0000#14",
///         "0.019531012396#31",
///         "380.000229#25",
///         "0.0",
///         "1511.5#12",
///         "0.000915587#14",
///         "32799.9997520447#46",
///         "0.0",
///         "0.0000305#6",
///         "24.0#2"
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_non_negative_finite_floats(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    zero_p_numerator: u64,
    zero_p_denominator: u64,
) -> RandomNonNegativeFiniteFloats<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomNonNegativeFiniteFloats {
        bs: weighted_random_bools(seed.fork("bs"), zero_p_numerator, zero_p_denominator),
        xs: striped_random_positive_finite_floats(
            seed.fork("xs"),
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
        ),
    }
}

/// Generates striped random non-positive finite [`Float`]s.
///
/// Negative zero is generated with the specified probability. If the [`Float`] to be generated is
/// nonzero, then the actual precision is chosen from a geometric distribution with mean $m$, where
/// $m$ is `mean_stripe_numerator / mean_stripe_denominator`; $m$ must be greater than 0. A striped
/// bit sequence with the given stripe parameter is generated and truncated at the bit length. The
/// highest bit is forced to be 1, and the [`Float`] is generated from the sequence and a random
/// sci-exponent.
///
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for information
/// about generating striped random numbers.
///
/// Negative zero is generated, but positive zero is not. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n / m + 1)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_precision_numerator` or `mean_precision_denominator` are
/// zero, or, if after being reduced to lowest terms, their sum is greater than or equal to
/// $2^{64}$.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::striped_random_non_positive_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_non_positive_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "-6.554e4#7",
///         "-0.0214843750#26",
///         "-8404960.0#19",
///         "-0.0",
///         "-0.0155065#16",
///         "-0.031219512#20",
///         "-3.94#6",
///         "-2.00378#15",
///         "-0.61712646#21",
///         "-16383.978515147231406#61",
///         "-12.0000#14",
///         "-0.019531012396#31",
///         "-380.000229#25",
///         "-0.0",
///         "-1511.5#12",
///         "-0.000915587#14",
///         "-32799.9997520447#46",
///         "-0.0",
///         "-0.0000305#6",
///         "-24.0#2"
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_non_positive_finite_floats(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    zero_p_numerator: u64,
    zero_p_denominator: u64,
) -> RandomNonPositiveFiniteFloats<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomNonPositiveFiniteFloats {
        bs: weighted_random_bools(seed.fork("bs"), zero_p_numerator, zero_p_denominator),
        xs: striped_random_negative_finite_floats(
            seed.fork("xs"),
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
        ),
    }
}

/// Generates striped random nonzero finite [`Float`]s.
///
/// The actual precision is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_stripe_numerator / mean_stripe_denominator`; $m$ must be greater than 0. A striped bit
/// sequence with the given stripe parameter is generated and truncated at the bit length. The
/// highest bit is forced to be 1, and the [`Float`] is generated from the sequence and a random
/// sci-exponent.
///
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for information
/// about generating striped random numbers.
///
/// Neither positive nor negative zero is generated. `NaN` is not generated either.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n / m + 1)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_precision_numerator` or `mean_precision_denominator` are
/// zero, or, if after being reduced to lowest terms, their sum is greater than or equal to
/// $2^{64}$.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::striped_random_nonzero_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_nonzero_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "-6.554e4#7",
///         "-0.0214843750#26",
///         "-8404960.0#19",
///         "0.0155065#16",
///         "-0.031219512#20",
///         "3.94#6",
///         "-2.00378#15",
///         "-0.61712646#21",
///         "-16383.978515147231406#61",
///         "12.0000#14",
///         "0.019531012396#31",
///         "380.000229#25",
///         "-1511.5#12",
///         "-0.000915587#14",
///         "32799.9997520447#46",
///         "-0.0000305#6",
///         "-24.0#2",
///         "-64.00#9",
///         "-760.0#7",
///         "-287.765624970#34"
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_nonzero_finite_floats(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
) -> RandomNonzeroFiniteFloats<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomNonzeroFiniteFloats {
        bs: random_bools(seed.fork("bs")),
        xs: striped_random_positive_finite_floats(
            seed.fork("xs"),
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
        ),
    }
}

/// Generates striped random finite [`Float`]s.
///
/// Zero is generated with the specified probability. If the [`Float`] to be generated is nonzero,
/// then the actual precision is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_stripe_numerator / mean_stripe_denominator`; $m$ must be greater than 0. A striped bit
/// sequence with the given stripe parameter is generated and truncated at the bit length. The
/// highest bit is forced to be 1, and the [`Float`] is generated from the sequence and a random
/// sci-exponent.
///
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for information
/// about generating striped random numbers.
///
/// Both positive and negative zero are generated. `NaN` is not.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n / m + 1)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_precision_numerator` or `mean_precision_denominator` are
/// zero, or, if after being reduced to lowest terms, their sum is greater than or equal to
/// $2^{64}$.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::striped_random_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "-3.89209#14",
///         "-2.607703209227954e-8#47",
///         "-0.093750#11",
///         "527.9999997541#38",
///         "-0.0005112#7",
///         "1.003845#17",
///         "-1.9e-6#3",
///         "-524272.0#16",
///         "-0.0004407074#18",
///         "7.75#5",
///         "0.0",
///         "0.12451#12",
///         "-1.9921865#21",
///         "-3.2e9#2",
///         "0.06250#8",
///         "-0.22#3",
///         "-0.015625#11",
///         "-3.81e-6#4",
///         "-64.000#13",
///         "-4064.000#19"
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_finite_floats(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    zero_p_numerator: u64,
    zero_p_denominator: u64,
) -> RandomFiniteFloats<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomFiniteFloats {
        bs: random_bools(seed.fork("bs")),
        xs: striped_random_non_negative_finite_floats(
            seed.fork("xs"),
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
            zero_p_numerator,
            zero_p_denominator,
        ),
    }
}

/// Generates striped random finite [`Float`]s.
///
/// Special values (NaN, infinities, and zeros) are generated with the specified probability. If the
/// [`Float`] to be generated is finite and nonzero, then the actual precision is chosen from a
/// geometric distribution with mean $m$, where $m$ is `mean_stripe_numerator /
/// mean_stripe_denominator`; $m$ must be greater than 0. A striped bit sequence with the given
/// stripe parameter is generated and truncated at the bit length. The highest bit is forced to be
/// 1, and the [`Float`] is generated from the sequence and a random sci-exponent.
///
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for information
/// about generating striped random numbers.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n, m) = O(n / m + 1)$
///
/// $M(n, m) = O(n / m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `mean_precision_numerator`, and $m$ is
/// `mean_precision_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_precision_numerator` or `mean_precision_denominator` are
/// zero, or, if after being reduced to lowest terms, their sum is greater than or equal to
/// $2^{64}$.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::striped_random_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1, 1, 10)
///         .take(50)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "7.99976#15",
///         "32.75#9",
///         "0.0",
///         "NaN",
///         "-0.000046#2",
///         "-5.1e2#1",
///         "-0.12488#10",
///         "-127.4999852#28",
///         "0.49999988#22",
///         "0.0002439022091#28",
///         "-0.11719#11",
///         "-9.9687500#23",
///         "-15.9844663292160586998132#75",
///         "Infinity",
///         "-0.484#5",
///         "-1.41e-11#5",
///         "-262144.00#21",
///         "NaN",
///         "8.8750#12",
///         "-0.0",
///         "-0.005859#7",
///         "0.062#1",
///         "0.12695307#22",
///         "0.000060976#10",
///         "-3.0733631e-8#22",
///         "1024.0#9",
///         "-3.1519e7#13",
///         "483.93847632#31",
///         "9.832438e-7#17",
///         "Infinity",
///         "-24.0#6",
///         "-3.6e-15#1",
///         "-Infinity",
///         "0.60839844448#31",
///         "-1.02e3#4",
///         "-0.00001526#7",
///         "-1.3e5#2",
///         "3.82971439e-6#24",
///         "-0.00012350#10",
///         "-0.0",
///         "-23.94#9",
///         "0.25#1",
///         "-0.0073261258913#31",
///         "3.8184225337224168437e-6#61",
///         "-0.000488281237267#34",
///         "0.0000151538#16",
///         "NaN",
///         "-0.0",
///         "6.7e7#1",
///         "20423.984375#33"
///     ]
/// );
/// ```
#[inline]
pub fn striped_random_floats(
    seed: Seed,
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
) -> WithSpecialValues<RandomFiniteFloats<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>>>
{
    with_special_values(
        seed,
        vec![Float::INFINITY, Float::NEGATIVE_INFINITY, Float::NAN],
        mean_special_p_numerator,
        mean_special_p_denominator,
        &|seed_2| {
            striped_random_finite_floats(
                seed_2,
                mean_sci_exponent_abs_numerator,
                mean_sci_exponent_abs_denominator,
                mean_stripe_numerator,
                mean_stripe_denominator,
                mean_precision_numerator,
                mean_precision_denominator,
                mean_special_p_numerator,
                mean_special_p_denominator,
            )
        },
    )
}

// This is a translation of mpfr_urandomb from urandomb.c, MPFR 4.2.2, using Malachite's seeded
// random streams in place of a GMP randstate.
/// Generates uniform random [`Float`]s in the interval $[0, 1)$, with a fixed precision.
///
/// This `struct` is created by [`uniform_random_non_negative_floats_less_than_one`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct UniformRandomNonNegativeFloatsLessThanOne {
    xs: RandomPrimitiveInts<u64>,
    prec: u64,
}

impl Iterator for UniformRandomNonNegativeFloatsLessThanOne {
    type Item = Float;

    fn next(&mut self) -> Option<Float> {
        // Draws exactly prec bits (in u64 chunks on every platform), mirroring mpfr_rand_raw's
        // guarantee that the stream position is independent of the machine word size.
        let k = get_random_natural_with_up_to_bits(&mut self.xs, self.prec);
        if k == 0u32 {
            // all drawn bits are zero
            Some(Float::ZERO)
        } else {
            let bits = k.significant_bits();
            // The value is k / 2^prec, so the raw exponent is bits - prec.
            let exponent = i64::exact_from(bits) - i64::exact_from(self.prec);
            if exponent < Float::MIN_EXPONENT_I64 {
                // Mirrors mpfr_urandomb: if the exponent is out of range (possible only when the
                // precision is on the order of 2^30), a NaN is returned as this is probably a user
                // error. This branch cannot be exercised by sampling: it requires a draw whose top
                // 2^30 or so bits are all zero, with probability around 2^(-2^30). It is not
                // limb-width-dependent (the stream is u64-based on every platform).
                Some(Float::NAN)
            } else {
                Some(Float(Finite {
                    sign: true,
                    exponent: i32::exact_from(exponent),
                    precision: self.prec,
                    significand: k
                        << (self.prec.neg_mod_power_of_2(Limb::LOG_WIDTH) + self.prec - bits),
                }))
            }
        }
    }
}

/// Generates uniform random [`Float`]s in the interval $[0, 1)$, with a fixed precision.
///
/// Each output is $k/2^p$, where $p$ is `prec` and $k$ is chosen uniformly from $[0, 2^p)$, so
/// every value is a dyadic rational whose denominator divides $2^p$, and each of the $2^p$ possible
/// values is equally likely. Every nonzero output has precision `prec`. Zero is drawn with
/// probability $2^{-p}$, and is a positive zero.
///
/// This function samples the same distribution as `mpfr_urandomb`. Like that function, it draws
/// exactly `prec` bits from the underlying stream per output, independently of the machine word
/// size, and returns `NaN` in the (practically unobservable) case that the scientific exponent of
/// the drawn value falls below [`Float::MIN_EXPONENT`], which can only happen when `prec` is on the
/// order of $2^{30}$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
///
/// # Panics
/// Panics if `prec` is zero.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::float::random::uniform_random_non_negative_floats_less_than_one;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     uniform_random_non_negative_floats_less_than_one(EXAMPLE_SEED, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "0.86035#10",
///         "0.084961#10",
///         "0.090820#10",
///         "0.61426#10",
///         "0.50684#10",
///         "0.97754#10",
///         "0.61133#10",
///         "0.35156#10",
///         "0.23633#10",
///         "0.47949#10",
///         "0.082031#10",
///         "0.15137#10",
///         "0.91992#10",
///         "0.34082#10",
///         "0.021484#10",
///         "0.20898#10",
///         "0.72949#10",
///         "0.62598#10",
///         "0.11230#10",
///         "0.13184#10"
///     ]
/// );
/// ```
pub fn uniform_random_non_negative_floats_less_than_one(
    seed: Seed,
    prec: u64,
) -> UniformRandomNonNegativeFloatsLessThanOne {
    assert_ne!(prec, 0);
    UniformRandomNonNegativeFloatsLessThanOne {
        xs: random_primitive_ints(seed),
        prec,
    }
}

// A bit source that consumes u32s from a u64 stream (low half of each word first) and serves n-bit
// requests by assembling full 64-bit chunks (two u32s, low first) and masking the low bits of one
// final u32 (or u32 pair) for the partial chunk. This is exactly the consumption pattern of MPFR
// driven by the same u64 stream through a custom GMP randstate whose partial requests take low
// bits, so a Float sampler built on this source can be compared against MPFR output-for-output over
// a whole stream, not just on first draws.
#[derive(Clone, Debug)]
struct U32BitSource<I: Iterator<Item = u64>> {
    xs: I,
    hi: Option<u32>,
}

impl<I: Iterator<Item = u64>> U32BitSource<I> {
    fn next_u32(&mut self) -> u32 {
        if let Some(h) = self.hi.take() {
            h
        } else {
            let x = self.xs.next().unwrap();
            self.hi = Some((x >> 32) as u32);
            x as u32
        }
    }

    // Draws n bits for n <= 32, low-aligned in the low bits of one u32.
    fn u32_bits(&mut self, n: u64) -> u32 {
        self.next_u32().mod_power_of_2(n)
    }

    // Draws a uniform random value in [0, n), replicating gmp_urandomm_ui: each attempt draws
    // exactly enough bits for n (one fewer when n is a power of 2, in which case no rejection can
    // occur), rejecting values that are too large. The iteration cap and the final fallback,
    // reachable only with a degenerate stream, are GMP's.
    fn uniform_mod(&mut self, n: u64) -> u64 {
        assert_ne!(n, 0);
        let bits = n.significant_bits() - u64::from(n.is_power_of_2());
        let mut r = 0;
        for _ in 0..80 {
            r = u64::exact_from(&self.bits(bits));
            if r < n {
                return r;
            }
        }
        r - n
    }

    // Draws n bits, low-aligned.
    fn bits(&mut self, n: u64) -> Natural {
        let mut result = Natural::ZERO;
        let mut shift = 0;
        for _ in 0..(n >> 6) {
            let lo = u64::from(self.next_u32());
            let hi = u64::from(self.next_u32());
            result |= Natural::from(lo | (hi << 32)) << shift;
            shift += 64;
        }
        let rest = n.mod_power_of_2(6);
        if rest >= 32 {
            let mut chunk = u64::from(self.next_u32());
            if rest > 32 {
                chunk |= u64::from(self.u32_bits(rest - 32)) << 32;
            }
            result |= Natural::from(chunk) << shift;
        } else if rest != 0 {
            result |= Natural::from(self.u32_bits(rest)) << shift;
        }
        result
    }
}

// This is a translation of mpfr_urandom from urandom.c, MPFR 4.2.2, including the underflow
// handling of mpfr_check_range from exceptions.c.
/// Generates random [`Float`]s in $[0, 1]$, as if a uniform random real number were drawn from the
/// unit interval and rounded to a fixed precision with a fixed rounding mode.
///
/// This `struct` is created by [`uniform_random_non_negative_floats_at_most_one`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct UniformRandomNonNegativeFloatsAtMostOne<I: Iterator<Item = u64>> {
    bits: U32BitSource<I>,
    prec: u64,
    rm: RoundingMode,
}

impl<I: Iterator<Item = u64>> Iterator for UniformRandomNonNegativeFloatsAtMostOne<I> {
    type Item = Float;

    fn next(&mut self) -> Option<Float> {
        // Step 1 (exponent): 0 with probability 1/2, -1 with probability 1/4, and so on, determined
        // by leading-zero counts of 8-bit blocks.
        let mut exponent: i64 = 0;
        loop {
            let block = self.u32_bits_block();
            let cnt = 8 - block.significant_bits();
            // Any exponent below MIN_EXPONENT - 1 behaves identically, so clamping here cannot
            // change the result, and it prevents any theoretical overflow. The generator is still
            // advanced, so the stream position does not depend on the clamp.
            if exponent >= const { Float::MIN_EXPONENT_I64 - 2 } {
                exponent -= i64::exact_from(cnt);
            }
            if cnt != 8 {
                break;
            }
        }
        // Step 2 (significand): prec - 1 drawn bits under an implicit leading 1, so the
        // pre-rounding value is in [1/2, 1) at the raw exponent 0 drawn above.
        let mut mantissa = if self.prec == 1 {
            Natural::ONE
        } else {
            self.bits.bits(self.prec - 1) | Natural::power_of_2(self.prec - 1)
        };
        // The rounding bit, which decides between the two neighboring representable values when
        // rounding to nearest: the exact value lies in an open 1-ulp interval, and the two halves
        // of that interval have equal measure.
        let rbit = self.bits.u32_bits(1);
        let up = match self.rm {
            Ceiling | Up => true,
            Floor | Down => false,
            Nearest => rbit != 0,
            // the constructor rejects Exact
            Exact => unreachable!(),
        };
        if up {
            mantissa += Natural::ONE;
            if mantissa.significant_bits() > self.prec {
                // the significand was all ones, so rounding up reaches the next binade
                mantissa >>= 1u64;
                exponent += 1;
            }
        }
        // Underflow handling, as in mpfr_check_range: unreachable by sampling, since reaching it
        // requires on the order of 2^27 consecutive all-zero 8-bit blocks.
        if exponent < Float::MIN_EXPONENT_I64 {
            // In the Nearest mode, round toward zero if the value is below half of the minimum
            // positive Float, or equal to that half with the exact value below it.
            let down = match self.rm {
                Floor | Down => true,
                Ceiling | Up => false,
                Nearest => {
                    exponent < Float::MIN_EXPONENT_MINUS_1_I64 || up && mantissa.is_power_of_2()
                }
                Exact => unreachable!(),
            };
            return Some(if down {
                Float::ZERO
            } else {
                {}
                Float(Finite {
                    sign: true,
                    exponent: Float::MIN_EXPONENT,
                    precision: self.prec,
                    significand: Natural::power_of_2(
                        self.prec.neg_mod_power_of_2(Limb::LOG_WIDTH) + self.prec - 1,
                    ),
                })
            });
        }
        Some(Float(Finite {
            sign: true,
            exponent: i32::exact_from(exponent),
            precision: self.prec,
            significand: mantissa << self.prec.neg_mod_power_of_2(Limb::LOG_WIDTH),
        }))
    }
}

impl<I: Iterator<Item = u64>> UniformRandomNonNegativeFloatsAtMostOne<I> {
    // Draws the 8-bit block used by the exponent loop.
    fn u32_bits_block(&mut self) -> u32 {
        self.bits.u32_bits(8)
    }
}

crate_test_fn! {
    // Like [`uniform_random_non_negative_floats_at_most_one`], but takes an arbitrary stream of
    // u64s instead of a seed, allowing tests to inject a rigged stream (for example, one that
    // reaches the underflow branches, which no seed can reach by sampling).
    uniform_random_non_negative_floats_at_most_one_from_u64s<I: Iterator<Item = u64>>(
        xs: I,
        prec: u64,
        rm: RoundingMode,
    ) -> UniformRandomNonNegativeFloatsAtMostOne<I> {
        assert_ne!(prec, 0);
        assert_ne!(rm, Exact);
        UniformRandomNonNegativeFloatsAtMostOne {
            bits: U32BitSource { xs, hi: None },
            prec,
            rm,
        }
    }
}

/// Generates random [`Float`]s in $[0, 1]$, as if a uniform random real number were drawn from the
/// unit interval and rounded to precision `prec` with rounding mode `rm`.
///
/// The distribution is that of a continuous uniform random variable on the unit interval, correctly
/// rounded: each output is a precision-`prec` [`Float`], and the probability of any output equals
/// the measure of the set of real numbers that round to it. Every output has precision `prec`. The
/// rounded value can be $1$ (under `Ceiling`, `Up`, or `Nearest`), but under `Floor` or `Down` it
/// is always less than $1$. It can be $0$ only via underflow, which requires the drawn exponent to
/// fall below the minimum exponent; this is unreachable in practice, since its probability is
/// roughly $2^{-2^{30}}$.
///
/// This function samples the same distribution as `mpfr_urandom`, and consumes randomness from the
/// underlying stream in the same pattern, including the fact that the amount consumed depends on
/// `prec` but not on `rm`. The result is never exact, so `Exact` is not a valid rounding mode.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
///
/// # Panics
/// Panics if `prec` is zero or if `rm` is `Exact`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::float::random::uniform_random_non_negative_floats_at_most_one;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     uniform_random_non_negative_floats_at_most_one(EXAMPLE_SEED, 10, Nearest)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "0.36182#10",
///         "0.59180#10",
///         "0.44922#10",
///         "0.48877#10",
///         "0.26904#10",
///         "0.73730#10",
///         "0.69531#10",
///         "0.65234#10",
///         "0.85059#10",
///         "0.52148#10",
///         "0.85547#10",
///         "0.039124#10",
///         "0.30127#10",
///         "0.38965#10",
///         "0.94336#10",
///         "0.48535#10",
///         "0.21631#10",
///         "0.078979#10",
///         "0.12866#10",
///         "0.36182#10"
///     ]
/// );
/// ```
#[inline]
pub fn uniform_random_non_negative_floats_at_most_one(
    seed: Seed,
    prec: u64,
    rm: RoundingMode,
) -> UniformRandomNonNegativeFloatsAtMostOne<RandomPrimitiveInts<u64>> {
    uniform_random_non_negative_floats_at_most_one_from_u64s(random_primitive_ints(seed), prec, rm)
}

// The random-deviate machinery of random_deviate.c, MPFR 4.2.2 (contributed to MPFR by Charles
// Karney): a lazily-extended random real, uniform in (0, 1), of which only the leading `e` bits
// have been decided. The first 32 bits (the "high fraction") live in `h`, and the rest in `f`. `h`
// is meaningless if e == 0, and `f` is meaningless if e <= 32. MPFR standardizes the chunk size at
// 32 bits for cross-platform reproducibility; every request below is one u32 from the stream, which
// keeps this machinery sequence-comparable with MPFR through the test harness.
#[derive(Clone, Debug)]
struct RandomDeviate {
    e: u64,
    h: u32,
    f: Natural,
}

const CHUNK: u64 = 32;
const CHUNK_PLUS_1: u64 = CHUNK + 1;
const TWICE_CHUNK: u64 = CHUNK << 1;
// A bound so large that exceeding it indicates a defective random stream.
const SANITY_BOUND: u64 = u64::MAX >> 1;

impl RandomDeviate {
    const fn new() -> Self {
        Self {
            e: 0,
            h: 0,
            f: Natural::ZERO,
        }
    }

    const fn reset(&mut self) {
        self.e = 0;
    }

    // Ensures that at least k bits of the fraction have been decided, drawing 32 bits at a time.
    // This is random_deviate_generate with a null mpz_t argument.
    fn generate<I: Iterator<Item = u64>>(&mut self, k: u64, src: &mut U32BitSource<I>) {
        if self.e >= k {
            return;
        }
        if self.e == 0 {
            self.h = src.next_u32();
            self.e = CHUNK;
            if self.e >= k {
                return;
            }
        }
        while self.e < k {
            let w = Natural::from(src.next_u32());
            self.f = if self.e == CHUNK {
                w
            } else {
                (&self.f << CHUNK) + w
            };
            self.e += CHUNK;
        }
    }

    // Like `generate`, but requests all needed bits at once, as random_deviate_generate does when
    // passed an mpz_t temporary. The batched request assembles its bits in the opposite order from
    // the chunked path, exactly as mpz_urandomb does relative to repeated gmp_urandomb_ui calls, so
    // the two paths must be kept distinct for MPFR parity.
    fn generate_batch<I: Iterator<Item = u64>>(&mut self, k: u64, src: &mut U32BitSource<I>) {
        if self.e >= k {
            return;
        }
        if self.e == 0 {
            self.h = src.next_u32();
            self.e = CHUNK;
            if self.e >= k {
                return;
            }
        }
        let k = k.div_round(CHUNK, Ceiling).0 * CHUNK - self.e;
        let t = src.bits(k);
        self.f = if self.e == CHUNK {
            t
        } else {
            (&self.f << k) + t
        };
        self.e += k;
    }

    // The position of the leading bit of the fraction, counting from 1: the leading bit represents
    // 2^(-l).
    fn leading_bit<I: Iterator<Item = u64>>(&mut self, src: &mut U32BitSource<I>) -> u64 {
        self.generate(CHUNK, src);
        if self.h != 0 {
            return CHUNK_PLUS_1 - self.h.significant_bits();
        }
        self.generate(TWICE_CHUNK, src);
        while self.f == 0u32 {
            self.generate(self.e + 1, src);
        }
        let l = self.e + 1 - self.f.significant_bits();
        // A ridiculously long string of leading zeros (probability on the order of 2^(-2^31)) would
        // indicate a defective random stream.
        assert!(l < SANITY_BOUND);
        l
    }

    // The kth bit of the fraction, representing 2^(-k). The k == 0 and k <= 32 arms are not
    // reachable from the exponential sampler, whose only caller of this function is the comparison
    // loop, which starts at k = 33; they are used by mpfr_nrandom's algorithms, which test fraction
    // bits from position 1.
    fn tstbit<I: Iterator<Item = u64>>(&mut self, k: u64, src: &mut U32BitSource<I>) -> bool {
        if k == 0 {
            return false;
        }
        self.generate(k, src);
        if k <= CHUNK {
            self.h.get_bit(CHUNK - k)
        } else {
            self.f.get_bit(self.e - k)
        }
    }
}

// Compares two random deviates, deciding more of their bits as needed to break ties. Since the
// deviates are (conceptually) uniform random reals, this terminates with probability 1.
fn random_deviate_less<I: Iterator<Item = u64>>(
    x: &mut RandomDeviate,
    y: &mut RandomDeviate,
    src: &mut U32BitSource<I>,
) -> bool {
    x.generate(CHUNK, src);
    y.generate(CHUNK, src);
    if x.h != y.h {
        return x.h < y.h;
    }
    let mut k = CHUNK_PLUS_1;
    loop {
        let a = x.tstbit(k, src);
        let b = y.tstbit(k, src);
        if a != b {
            return b;
        }
        k += 1;
    }
}

// Converts n + x, where x is a random deviate, to a Float rounded to `prec` with `rm`, deciding as
// many more bits of x as the precision requires. This is mpfr_random_deviate_value, with the sign
// applied before the rounding, since the directed rounding modes do not commute with negation. The
// trailing bit of the assembled integer is set, so the result is always inexact, and there are
// never ties to break in the Nearest mode.
fn random_deviate_value<I: Iterator<Item = u64>>(
    neg: bool,
    n: u64,
    x: &mut RandomDeviate,
    prec: u64,
    rm: RoundingMode,
    src: &mut U32BitSource<I>,
) -> Float {
    let (s_positive, l) = if n == 0 {
        (false, x.leading_bit(src))
    } else {
        (true, n.significant_bits() - 1)
    };
    if s_positive && prec + 1 > l || !s_positive {
        let k = if s_positive {
            prec + 1 - l
        } else {
            prec + 1 + l
        };
        x.generate_batch(k, src);
    }
    let mut t = if n == 0 {
        // the minimum precision is 1, so the high fraction has been generated
        Natural::from(x.h)
    } else {
        let mut t = Natural::from(n);
        if x.e > 0 {
            t <<= CHUNK;
            t += Natural::from(x.h);
        }
        t
    };
    if x.e > CHUNK {
        t <<= x.e - CHUNK;
        t += &x.f;
    }
    t.set_bit(0);
    // The exact value is +/- t * 2^(-e). Negating exactly and then shifting with rounding rounds
    // once, as mpfr_set_z_2exp does, and handles the (unreachable-by-sampling) underflow at extreme
    // e.
    let mut exact = Float::exact_from(t);
    if neg {
        exact.neg_assign();
    }
    exact.shr_prec_round(x.e, prec, rm).0
}

// This is a translation of mpfr_erandom from erandom.c, MPFR 4.2.2, which uses von Neumann's
// rejection algorithm: the integer part of the deviate is the number of leading rejections, and
// each accept/reject test is a Bernoulli trial with success probability exp(-x), realized as
// comparisons of lazily-decided uniform deviates, with no transcendental evaluations.
/// Generates random [`Float`]s sampled, with rounding, from the exponential distribution with mean
/// 1.
///
/// This `struct` is created by [`exponential_random_floats`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct ExponentialRandomFloats<I: Iterator<Item = u64>> {
    bits: U32BitSource<I>,
    prec: u64,
    rm: RoundingMode,
}

// True with probability exp(-x): von Neumann's test, using p and q as scratch deviates.
fn exp_bernoulli<I: Iterator<Item = u64>>(
    x: &mut RandomDeviate,
    p: &mut RandomDeviate,
    q: &mut RandomDeviate,
    src: &mut U32BitSource<I>,
) -> bool {
    p.reset();
    if !random_deviate_less(p, x, src) {
        return true;
    }
    loop {
        q.reset();
        if !random_deviate_less(q, p, src) {
            return false;
        }
        p.reset();
        if !random_deviate_less(p, q, src) {
            return true;
        }
    }
}

impl<I: Iterator<Item = u64>> Iterator for ExponentialRandomFloats<I> {
    type Item = Float;

    fn next(&mut self) -> Option<Float> {
        let mut x = RandomDeviate::new();
        let mut p = RandomDeviate::new();
        let mut q = RandomDeviate::new();
        let mut k: u64 = 0;
        while !exp_bernoulli(&mut x, &mut p, &mut q, &mut self.bits) {
            k += 1;
            // A wraparound of k (probability on the order of exp(-2^64)) would indicate a defective
            // random stream.
            assert_ne!(k, 0);
            x.reset();
        }
        Some(random_deviate_value(
            false,
            k,
            &mut x,
            self.prec,
            self.rm,
            &mut self.bits,
        ))
    }
}

crate_test_fn! {
    // Like [`exponential_random_floats`], but takes an arbitrary stream of u64s instead of a seed,
    // allowing tests to inject a rigged stream.
    exponential_random_floats_from_u64s<I: Iterator<Item = u64>>(
        xs: I,
        prec: u64,
        rm: RoundingMode,
    ) -> ExponentialRandomFloats<I> {
        assert_ne!(prec, 0);
        assert_ne!(rm, Exact);
        ExponentialRandomFloats {
            bits: U32BitSource { xs, hi: None },
            prec,
            rm,
        }
    }
}

/// Generates random [`Float`]s sampled, with rounding, from the exponential distribution with mean
/// 1.
///
/// The result is a correctly-rounded sample: each output is a precision-`prec` [`Float`], and the
/// probability of any output equals the probability that an exponentially-distributed real number
/// rounds to it under `rm`. The sampler is von Neumann's rejection algorithm as used by
/// `mpfr_erandom`, which draws no transcendental function evaluations; the number of random bits
/// consumed is finite with probability 1 but not bounded. Every output is positive: a zero would
/// require underflow, whose probability is on the order of $2^{-2^{30}}$. The result is never
/// exact, so `Exact` is not a valid rounding mode.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
///
/// # Panics
/// Panics if `prec` is zero or if `rm` is `Exact`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::float::random::exponential_random_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     exponential_random_floats(EXAMPLE_SEED, 10, Nearest)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "0.63184#10",
///         "1.4648#10",
///         "0.96582#10",
///         "2.6836#10",
///         "3.6719#10",
///         "2.5703#10",
///         "0.097046#10",
///         "1.6602#10",
///         "0.69629#10",
///         "0.052429#10",
///         "0.58398#10",
///         "0.23486#10",
///         "0.88965#10",
///         "2.1992#10",
///         "1.7480#10",
///         "0.16748#10",
///         "0.35693#10",
///         "1.0996#10",
///         "0.44238#10",
///         "0.51172#10"
///     ]
/// );
/// ```
#[inline]
pub fn exponential_random_floats(
    seed: Seed,
    prec: u64,
    rm: RoundingMode,
) -> ExponentialRandomFloats<RandomPrimitiveInts<u64>> {
    exponential_random_floats_from_u64s(random_primitive_ints(seed), prec, rm)
}

// True with probability exp(-1/2): algorithm H of mpfr_nrandom, whose initial rejection step just
// tests the leading fraction bit.
fn half_exp_bernoulli<I: Iterator<Item = u64>>(
    p: &mut RandomDeviate,
    q: &mut RandomDeviate,
    src: &mut U32BitSource<I>,
) -> bool {
    p.reset();
    if p.tstbit(1, src) {
        return true;
    }
    loop {
        q.reset();
        if !random_deviate_less(q, p, src) {
            return false;
        }
        p.reset();
        if !random_deviate_less(p, q, src) {
            return true;
        }
    }
}

// Returns n >= 0 with probability exp(-n/2) * (1 - exp(-1/2)): step N1 of mpfr_nrandom.
fn truncated_exponential<I: Iterator<Item = u64>>(
    p: &mut RandomDeviate,
    q: &mut RandomDeviate,
    src: &mut U32BitSource<I>,
) -> u64 {
    let mut n = 0;
    while half_exp_bernoulli(p, q, src) {
        n += 1;
        // A wraparound of n (probability on the order of exp(-2^64)) would indicate a defective
        // random stream.
        assert_ne!(n, 0);
    }
    n
}

// True with probability exp(-m * n / 2): step N2 of mpfr_nrandom. The product m * n is passed as
// two separate factors because the caller passes m = k and n = k - 1 with a wrapping subtraction:
// for k = 0, m = 0 and the wrapped n is never used, so the result is true.
fn exp_half_product_bernoulli<I: Iterator<Item = u64>>(
    m: u64,
    n: u64,
    p: &mut RandomDeviate,
    q: &mut RandomDeviate,
    src: &mut U32BitSource<I>,
) -> bool {
    for _ in 0..m {
        for _ in 0..n {
            if !half_exp_bernoulli(p, q, src) {
                return false;
            }
        }
    }
    true
}

// Returns -1, 0, or 1 with probabilities 1/m, 1/m, and 1 - 2/m: algorithm C of mpfr_nrandom.
fn choice<I: Iterator<Item = u64>>(m: u64, src: &mut U32BitSource<I>) -> i8 {
    match src.uniform_mod(m) {
        0 => -1,
        1 => 0,
        _ => 1,
    }
}

// True with probability exp(-x * (2 * k + x) / (2 * k + 2)): algorithm B of mpfr_nrandom. The loop
// unpacks the short-circuit condition chain of the C original, preserving the order of the draws;
// the result is whether the number of completed iterations is even.
fn tail_bernoulli<I: Iterator<Item = u64>>(
    k: u64,
    x: &mut RandomDeviate,
    p: &mut RandomDeviate,
    q: &mut RandomDeviate,
    src: &mut U32BitSource<I>,
) -> bool {
    // 2 * k + 2 would overflow; a k this large (probability on the order of exp(-2^63)) would
    // indicate a defective random stream.
    assert!(k < SANITY_BOUND);
    let m = (k << 1) + 2;
    let mut parity_even = true;
    let mut first = true;
    loop {
        let mut f = if k == 0 { choice(m, src) } else { 0 };
        if f < 0 {
            break;
        }
        q.reset();
        if first {
            random_deviate_less(q, x, src);
        } else {
            random_deviate_less(q, p, src);
            break;
        };
        if k != 0 {
            f = choice(m, src);
        }
        if f < 0 {
            break;
        }
        if f == 0 {
            p.reset();
            if !random_deviate_less(p, x, src) {
                break;
            }
        }
        core::mem::swap(p, q);
        parity_even.not_assign();
        first = false;
    }
    parity_even
}

// This is a translation of mpfr_nrandom from nrandom.c, MPFR 4.2.2: algorithm N of Karney,
// "Sampling exactly from the normal distribution", ACM Transactions on Mathematical Software 42(1)
// (2016). Everything is built from Bernoulli trials on lazily-decided uniform deviates, with no
// transcendental evaluations.
/// Generates random [`Float`]s sampled, with rounding, from the normal distribution with mean 0 and
/// variance 1.
///
/// This `struct` is created by [`normal_random_floats`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct NormalRandomFloats<I: Iterator<Item = u64>> {
    bits: U32BitSource<I>,
    prec: u64,
    rm: RoundingMode,
}

impl<I: Iterator<Item = u64>> Iterator for NormalRandomFloats<I> {
    type Item = Float;

    fn next(&mut self) -> Option<Float> {
        let mut x = RandomDeviate::new();
        let mut p = RandomDeviate::new();
        let mut q = RandomDeviate::new();
        let k;
        loop {
            // step 1: k with probability exp(-k/2) * (1 - exp(-1/2))
            let kk = truncated_exponential(&mut p, &mut q, &mut self.bits);
            // step 2: accept with probability exp(-k * (k - 1) / 2), so that k now follows the
            // normal tail weights
            if !exp_half_product_bernoulli(kk, kk.wrapping_sub(1), &mut p, &mut q, &mut self.bits) {
                continue;
            }
            // steps 3 and 4: accept the fraction x with probability exp(-x * (2 * k + x) / 2), via
            // k + 1 successes of the tail test
            x.reset();
            let mut j = 0;
            while j <= kk && tail_bernoulli(kk, &mut x, &mut p, &mut q, &mut self.bits) {
                j += 1;
            }
            if j > kk {
                k = kk;
                break;
            }
        }
        // steps 5 to 7: attach a random sign to k + x and round
        let neg = self.bits.u32_bits(1) != 0;
        Some(random_deviate_value(
            neg,
            k,
            &mut x,
            self.prec,
            self.rm,
            &mut self.bits,
        ))
    }
}

crate_test_fn! {
    // Like [`normal_random_floats`], but takes an arbitrary stream of u64s instead of a seed,
    // allowing tests to inject a rigged stream.
    normal_random_floats_from_u64s<I: Iterator<Item = u64>>(
        xs: I,
        prec: u64,
        rm: RoundingMode,
    ) -> NormalRandomFloats<I> {
        assert_ne!(prec, 0);
        assert_ne!(rm, Exact);
        NormalRandomFloats {
            bits: U32BitSource { xs, hi: None },
            prec,
            rm,
        }
    }
}

crate_test_fn! {
    // Direct access to the gmp_urandomm_ui replica, for differential testing against GMP.
    uniform_mod_from_u64s<I: Iterator<Item = u64>>(xs: I, n: u64) -> u64 {
        let mut src = U32BitSource { xs, hi: None };
        src.uniform_mod(n)
    }
}

/// Generates random [`Float`]s sampled, with rounding, from the normal distribution with mean 0 and
/// variance 1.
///
/// The result is a correctly-rounded sample: each output is a precision-`prec` [`Float`], and the
/// probability of any output equals the probability that a normally-distributed real number rounds
/// to it under `rm`. The sampler is algorithm N of Karney, "Sampling exactly from the normal
/// distribution", as used by `mpfr_nrandom`; it is built entirely from Bernoulli trials on
/// lazily-decided uniform deviates and draws no transcendental function evaluations. The number of
/// random bits consumed is finite with probability 1 but not bounded. Every output is nonzero: a
/// zero would require underflow, whose probability is on the order of $2^{-2^{30}}$. The result is
/// never exact, so `Exact` is not a valid rounding mode.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
///
/// # Panics
/// Panics if `prec` is zero or if `rm` is `Exact`.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_float::float::random::normal_random_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     normal_random_floats(EXAMPLE_SEED, 10, Nearest)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &[
///         "-0.45166#10",
///         "-2.2695#10",
///         "-2.1602#10",
///         "-0.78516#10",
///         "0.23486#10",
///         "-0.61230#10",
///         "-0.91797#10",
///         "-0.13672#10",
///         "1.2891#10",
///         "-0.045227#10",
///         "-0.77051#10",
///         "-0.21143#10",
///         "0.61621#10",
///         "-0.58594#10",
///         "0.57520#10",
///         "1.0117#10",
///         "0.58008#10",
///         "1.0195#10",
///         "0.89453#10",
///         "-0.069092#10"
///     ]
/// );
/// ```
#[inline]
pub fn normal_random_floats(
    seed: Seed,
    prec: u64,
    rm: RoundingMode,
) -> NormalRandomFloats<RandomPrimitiveInts<u64>> {
    normal_random_floats_from_u64s(random_primitive_ints(seed), prec, rm)
}
