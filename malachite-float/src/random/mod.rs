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
use malachite_base::num::arithmetic::traits::{NegModPowerOf2, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::logic::traits::{LowMask, SignificantBits};
use malachite_base::num::random::geometric::{
    GeometricRandomNaturalValues, GeometricRandomSignedRange,
    geometric_random_signed_inclusive_range,
};
use malachite_base::random::Seed;
use malachite_nz::natural::Natural;
use malachite_nz::natural::random::{
    RandomNaturals, StripedRandomNaturalInclusiveRange, StripedRandomNaturals,
    UniformRandomNaturalRange, random_positive_naturals, striped_random_natural_inclusive_range,
    striped_random_positive_naturals, uniform_random_natural_inclusive_range,
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
/// $T(n, m) = O(n + m)$
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
/// use malachite_float::random::random_positive_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_positive_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["0.88#3", "1.31e-6#6", "0.0078#1", "0.50#1", "82144.0#13", "0.01558827446#29", "0.016#1", "3.406#7", "4.5981711652#33", "0.000033432058#23", "0.3392996773764#37", "2.662e4#7", "3.3e4#1", "1.398#8", "37.38#9", "0.25#1", "0.0011108#13", "1066.0#10", "0.1836#7", "0.001332305612#28"]
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
/// $M(n,) = O(n)$
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
/// use malachite_float::random::random_positive_floats_with_precision;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_positive_floats_with_precision(EXAMPLE_SEED, 10, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["0.95898#10", "1.8887e-6#10", "0.012909#10", "0.70996#10", "1.0202e5#10", "0.011810#10", "0.019531#10", "3.0820#10", "7.2422#10", "0.000055969#10", "0.38770#10", "21440.0#10", "58560.0#10", "1.4297#10", "62.188#10", "0.46582#10", "0.0016594#10", "1914.0#10", "0.13599#10", "0.0011444#10"]
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
/// $T(n, m) = O(n + m)$
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
/// use malachite_float::random::random_negative_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_negative_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["-0.88#3", "-1.31e-6#6", "-0.0078#1", "-0.50#1", "-82144.0#13", "-0.01558827446#29", "-0.016#1", "-3.406#7", "-4.5981711652#33", "-0.000033432058#23", "-0.3392996773764#37", "-2.662e4#7", "-3.3e4#1", "-1.398#8", "-37.38#9", "-0.25#1", "-0.0011108#13", "-1066.0#10", "-0.1836#7", "-0.001332305612#28"]
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
/// $T(n, m) = O(n + m)$
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
/// use malachite_float::random::random_non_negative_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_non_negative_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["1.11e5#5", "0.03108048#17", "9.59386e6#14", "0.0", "0.0127#5", "0.018433#11", "2.00#5", "3.0820#10", "0.874954#16", "10288.29527676#38", "9.2188#10", "0.030048549#23", "311.4521#19", "0.0", "1072.0#7", "0.0009651#9", "59159.52197#27", "0.0", "0.0000353#6", "16.0#1"]
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
/// $T(n, m) = O(n + m)$
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
/// use malachite_float::random::random_non_positive_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_non_positive_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["-1.11e5#5", "-0.03108048#17", "-9.59386e6#14", "-0.0", "-0.0127#5", "-0.018433#11", "-2.00#5", "-3.0820#10", "-0.874954#16", "-10288.29527676#38", "-9.2188#10", "-0.030048549#23", "-311.4521#19", "-0.0", "-1072.0#7", "-0.0009651#9", "-59159.52197#27", "-0.0", "-0.0000353#6", "-16.0#1"]
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
/// $T(n, m) = O(n + m)$
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
/// use malachite_float::random::random_nonzero_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_nonzero_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["-1.11e5#5", "-0.03108048#17", "-9.59386e6#14", "0.0127#5", "-0.018433#11", "2.00#5", "-3.0820#10", "-0.874954#16", "-10288.29527676#38", "9.2188#10", "0.030048549#23", "311.4521#19", "-1072.0#7", "-0.0009651#9", "59159.52197#27", "-0.0000353#6", "-16.0#1", "-120.0#5", "-960.0#5", "-358.24023#20"]
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
/// $T(n, m) = O(n + m)$
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
/// use malachite_float::random::random_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["-2.438#7", "-2.3233958868e-8#30", "-0.0859#6", "1009.3770#20", "-0.000824#6", "1.9805#10", "-1.9e-6#3", "-268192.0#14", "-0.00033855#10", "6.0#2", "0.0", "0.102#5", "-1.3665#13", "-3.2e9#2", "0.117#4", "-0.19#2", "-0.03003#7", "-3.8e-6#2", "-114.0#6", "-4002.0#13"]
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
/// $T(n, m) = O(n + m)$
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
/// use malachite_float::random::random_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_floats(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///         .take(50)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["7.2031#10", "39.25#8", "0.0", "NaN", "-0.000031#2", "-5.1e2#1", "-0.08789#8", "-95.12012#17", "0.380768#14", "0.000138037#15", "-0.1094#7", "-10.312#12", "-13.683969005122592#51", "Infinity", "-0.344#4", "-7.28e-12#5", "-394584.0#16", "NaN", "13.5#5", "-0.0", "-0.00635#5", "0.062#1", "0.18933#12", "0.0000401#6", "-4.8189e-8#13", "1.15e3#6", "-1.914e7#7", "475.7344#17", "1.103e-6#7", "Infinity", "-24.0#3", "-3.6e-15#1", "-Infinity", "0.50391#11", "-1.0e3#3", "-0.0000281#6", "-2.0e5#2", "6.4317792e-6#20", "-0.000191#5", "-0.0", "-30.0#4", "0.25#1", "-0.006299376#18", "4.582787718616e-6#38", "-0.0002707085#19", "0.000013128#10", "NaN", "-0.0", "6.7e7#1", "20263.5#16"]
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
/// $T(n, m) = O(n + m)$
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
/// use malachite_float::random::striped_random_positive_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_positive_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["0.938#4", "1.9064e-6#11", "0.0078#2", "0.50#3", "98332.000#21", "0.014160633101709896512#60", "0.023#2", "2.109#8", "4.000030282884437849#57", "0.000057221276833275#43", "0.25000005983747242139#63", "24576.0#12", "3.3e4#1", "1.98431#16", "33.500#12", "0.25#1", "0.00097680069#23", "1279.50000#25", "0.1250#7", "0.0014648735386622#42"]
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
/// use malachite_float::random::striped_random_positive_floats_with_precision;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_positive_floats_with_precision(EXAMPLE_SEED, 10, 1, 8, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["0.81152#10", "9.5367e-7#10", "0.015610#10", "0.50000#10", "65536.0#10", "0.015381#10", "0.017548#10", "3.9961#10", "7.9922#10", "0.000060976#10", "0.44092#10", "32736.0#10", "64960.0#10", "1.1250#10", "63.938#10", "0.29688#10", "0.0019512#10", "1920.0#10", "0.12573#10", "0.0014629#10"]
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
/// $T(n, m) = O(n + m)$
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
/// use malachite_float::random::striped_random_negative_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_negative_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["-0.938#4", "-1.9064e-6#11", "-0.0078#2", "-0.50#3", "-98332.000#21", "-0.014160633101709896512#60", "-0.023#2", "-2.109#8", "-4.000030282884437849#57", "-0.000057221276833275#43", "-0.25000005983747242139#63", "-24576.0#12", "-3.3e4#1", "-1.98431#16", "-33.500#12", "-0.25#1", "-0.00097680069#23", "-1279.50000#25", "-0.1250#7", "-0.0014648735386622#42"]
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
/// $T(n, m) = O(n + m)$
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
/// use malachite_float::random::striped_random_non_negative_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_non_negative_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["6.554e4#7", "0.0214843750#26", "8404960.0#19", "0.0", "0.0155065#16", "0.031219512#20", "3.94#6", "2.00378#15", "0.61712646#21", "16383.978515147231406#61", "12.0000#14", "0.019531012396#31", "380.000229#25", "0.0", "1511.5#12", "0.000915587#14", "32799.9997520447#46", "0.0", "0.0000305#6", "24.0#2"]
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
/// $T(n, m) = O(n + m)$
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
/// use malachite_float::random::striped_random_non_positive_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_non_positive_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["-6.554e4#7", "-0.0214843750#26", "-8404960.0#19", "-0.0", "-0.0155065#16", "-0.031219512#20", "-3.94#6", "-2.00378#15", "-0.61712646#21", "-16383.978515147231406#61", "-12.0000#14", "-0.019531012396#31", "-380.000229#25", "-0.0", "-1511.5#12", "-0.000915587#14", "-32799.9997520447#46", "-0.0", "-0.0000305#6", "-24.0#2"]
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
/// $T(n, m) = O(n + m)$
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
/// use malachite_float::random::striped_random_nonzero_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_nonzero_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["-6.554e4#7", "-0.0214843750#26", "-8404960.0#19", "0.0155065#16", "-0.031219512#20", "3.94#6", "-2.00378#15", "-0.61712646#21", "-16383.978515147231406#61", "12.0000#14", "0.019531012396#31", "380.000229#25", "-1511.5#12", "-0.000915587#14", "32799.9997520447#46", "-0.0000305#6", "-24.0#2", "-64.00#9", "-760.0#7", "-287.765624970#34"]
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
/// $T(n, m) = O(n + m)$
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
/// use malachite_float::random::striped_random_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1, 1, 10)
///         .take(20)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["-3.89209#14", "-2.607703209227954e-8#47", "-0.093750#11", "527.9999997541#38", "-0.0005112#7", "1.003845#17", "-1.9e-6#3", "-524272.0#16", "-0.0004407074#18", "7.75#5", "0.0", "0.12451#12", "-1.9921865#21", "-3.2e9#2", "0.06250#8", "-0.22#3", "-0.015625#11", "-3.81e-6#4", "-64.000#13", "-4064.000#19"]
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
/// $T(n, m) = O(n + m)$
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
/// use malachite_float::random::striped_random_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1, 1, 10)
///         .take(50)
///         .map(|f| ComparableFloat(f).to_string())
///         .collect_vec()
///         .as_slice(),
///     &["7.99976#15", "32.75#9", "0.0", "NaN", "-0.000046#2", "-5.1e2#1", "-0.12488#10", "-127.4999852#28", "0.49999988#22", "0.0002439022091#28", "-0.11719#11", "-9.9687500#23", "-15.9844663292160586998132#75", "Infinity", "-0.484#5", "-1.41e-11#5", "-262144.00#21", "NaN", "8.8750#12", "-0.0", "-0.005859#7", "0.062#1", "0.12695307#22", "0.000060976#10", "-3.0733631e-8#22", "1024.0#9", "-3.1519e7#13", "483.93847632#31", "9.832438e-7#17", "Infinity", "-24.0#6", "-3.6e-15#1", "-Infinity", "0.60839844448#31", "-1.02e3#4", "-0.00001526#7", "-1.3e5#2", "3.82971439e-6#24", "-0.00012350#10", "-0.0", "-23.94#9", "0.25#1", "-0.0073261258913#31", "3.8184225337224168437e-6#61", "-0.000488281237267#34", "0.0000151538#16", "NaN", "-0.0", "6.7e7#1", "20423.984375#33"]
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
