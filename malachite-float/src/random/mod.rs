// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use malachite_base::bools::random::{
    random_bools, weighted_random_bools, RandomBools, WeightedRandomBools,
};
use malachite_base::iterators::{with_special_values, WithSpecialValues};
use malachite_base::num::arithmetic::traits::{NegModPowerOf2, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::logic::traits::{LowMask, SignificantBits};
use malachite_base::num::random::geometric::{
    geometric_random_signeds, GeometricRandomNaturalValues, GeometricRandomSigneds,
};
use malachite_base::random::Seed;
use malachite_nz::natural::random::{
    random_positive_naturals, striped_random_natural_inclusive_range,
    striped_random_positive_naturals, uniform_random_natural_inclusive_range, RandomNaturals,
    StripedRandomNaturalInclusiveRange, StripedRandomNaturals, UniformRandomNaturalRange,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

/// Generates random positive finite [`Float`]s.
///
/// This `struct` is created by [`random_positive_finite_floats`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomPositiveFiniteFloats<I: Iterator<Item = Natural>> {
    exponents: GeometricRandomSigneds<i32>,
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
///     &[
///         "0.9#3",
///         "1.31e-6#6",
///         "0.008#1",
///         "0.5#1",
///         "8.214e4#13",
///         "0.01558827446#29",
///         "0.02#1",
///         "3.41#7",
///         "4.598171165#33",
///         "0.000033432058#23",
///         "0.339299677376#37",
///         "2.66e4#7",
///         "3.0e4#1",
///         "1.4#8",
///         "37.4#9",
///         "0.2#1",
///         "0.0011108#13",
///         "1066.0#10",
///         "0.184#7",
///         "0.00133230561#28"
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
        exponents: geometric_random_signeds(
            seed.fork("exponents"),
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
///     &[
///         "0.959#10",
///         "1.889e-6#10",
///         "0.01291#10",
///         "0.71#10",
///         "1.02e5#10",
///         "0.01181#10",
///         "0.01953#10",
///         "3.082#10",
///         "7.24#10",
///         "0.00005597#10",
///         "0.3877#10",
///         "2.144e4#10",
///         "5.856e4#10",
///         "1.43#10",
///         "62.19#10",
///         "0.4658#10",
///         "0.001659#10",
///         "1914.0#10",
///         "0.136#10",
///         "0.001144#10"
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
        exponents: geometric_random_signeds(
            seed.fork("exponents"),
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
///     &[
///         "0.9#3",
///         "1.31e-6#6",
///         "0.008#1",
///         "0.5#1",
///         "8.214e4#13",
///         "0.01558827446#29",
///         "0.02#1",
///         "3.41#7",
///         "4.598171165#33",
///         "0.000033432058#23",
///         "0.339299677376#37",
///         "2.66e4#7",
///         "3.0e4#1",
///         "1.4#8",
///         "37.4#9",
///         "0.2#1",
///         "0.0011108#13",
///         "1066.0#10",
///         "0.184#7",
///         "0.00133230561#28"
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
///     &[
///         "1.11e5#5",
///         "0.0310805#17",
///         "9.594e6#14",
///         "0.0",
///         "0.0127#5",
///         "0.01843#11",
///         "2.0#5",
///         "3.082#10",
///         "0.87495#16",
///         "10288.29527676#38",
///         "9.22#10",
///         "0.030048549#23",
///         "311.452#19",
///         "0.0",
///         "1.07e3#7",
///         "0.000965#9",
///         "59159.522#27",
///         "0.0",
///         "0.000035#6",
///         "2.0e1#1"
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
///     &[
///         "-1.11e5#5",
///         "-0.0310805#17",
///         "-9.594e6#14",
///         "-0.0",
///         "-0.0127#5",
///         "-0.01843#11",
///         "-2.0#5",
///         "-3.082#10",
///         "-0.87495#16",
///         "-10288.29527676#38",
///         "-9.22#10",
///         "-0.030048549#23",
///         "-311.452#19",
///         "-0.0",
///         "-1.07e3#7",
///         "-0.000965#9",
///         "-59159.522#27",
///         "-0.0",
///         "-0.000035#6",
///         "-2.0e1#1"
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
///     &[
///         "-1.11e5#5",
///         "-0.0310805#17",
///         "-9.594e6#14",
///         "0.0127#5",
///         "-0.01843#11",
///         "2.0#5",
///         "-3.082#10",
///         "-0.87495#16",
///         "-10288.29527676#38",
///         "9.22#10",
///         "0.030048549#23",
///         "311.452#19",
///         "-1.07e3#7",
///         "-0.000965#9",
///         "59159.522#27",
///         "-0.000035#6",
///         "-2.0e1#1",
///         "-120.0#5",
///         "-9.6e2#5",
///         "-358.2402#20"
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
///     &[
///         "-2.44#7",
///         "-2.323395887e-8#30",
///         "-0.086#6",
///         "1009.377#20",
///         "-0.00082#6",
///         "1.98#10",
///         "-1.9e-6#3",
///         "-2.6819e5#14",
///         "-0.0003386#10",
///         "6.0#2",
///         "0.0",
///         "0.1#5",
///         "-1.3665#13",
///         "-3.0e9#2",
///         "0.117#4",
///         "-0.19#2",
///         "-0.03#7",
///         "-4.0e-6#2",
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
///     &[
///         "7.203#10",
///         "39.2#8",
///         "0.0",
///         "NaN",
///         "-0.00003#2",
///         "-5.0e2#1",
///         "-0.0879#8",
///         "-95.12#17",
///         "0.38077#14",
///         "0.000138037#15",
///         "-0.109#7",
///         "-10.312#12",
///         "-13.68396900512259#51",
///         "Infinity",
///         "-0.34#4",
///         "-7.3e-12#5",
///         "-394584.0#16",
///         "NaN",
///         "13.5#5",
///         "-0.0",
///         "-0.0063#5",
///         "0.06#1",
///         "0.18933#12",
///         "0.00004#6",
///         "-4.819e-8#13",
///         "1.15e3#6",
///         "-1.91e7#7",
///         "475.734#17",
///         "1.1e-6#7",
///         "Infinity",
///         "-24.0#3",
///         "-4.0e-15#1",
///         "-Infinity",
///         "0.5039#11",
///         "-1.0e3#3",
///         "-0.0000281#6",
///         "-2.0e5#2",
///         "6.43178e-6#20",
///         "-0.00019#5",
///         "-0.0",
///         "-30.0#4",
///         "0.2#1",
///         "-0.00629938#18",
///         "4.58278771862e-6#38",
///         "-0.0002707085#19",
///         "0.00001313#10",
///         "NaN",
///         "-0.0",
///         "7.0e7#1",
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
///     &[
///         "0.94#4",
///         "1.906e-6#11",
///         "0.008#2",
///         "0.5#3",
///         "98332.0#21",
///         "0.01416063310170989651#60",
///         "0.023#2",
///         "2.11#8",
///         "4.00003028288443785#57",
///         "0.000057221276833275#43",
///         "0.25000005983747242139#63",
///         "24576.0#12",
///         "3.0e4#1",
///         "1.98431#16",
///         "33.5#12",
///         "0.2#1",
///         "0.0009768007#23",
///         "1279.5#25",
///         "0.125#7",
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
        exponents: geometric_random_signeds(
            seed.fork("exponents"),
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
///     &[
///         "0.94#4",
///         "1.906e-6#11",
///         "0.008#2",
///         "0.5#3",
///         "98332.0#21",
///         "0.01416063310170989651#60",
///         "0.023#2",
///         "2.11#8",
///         "4.00003028288443785#57",
///         "0.000057221276833275#43",
///         "0.25000005983747242139#63",
///         "24576.0#12",
///         "3.0e4#1",
///         "1.98431#16",
///         "33.5#12",
///         "0.2#1",
///         "0.0009768007#23",
///         "1279.5#25",
///         "0.125#7",
///         "0.0014648735386622#42"
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
        exponents: geometric_random_signeds(
            seed.fork("exponents"),
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
///     &[
///         "-0.94#4",
///         "-1.906e-6#11",
///         "-0.008#2",
///         "-0.5#3",
///         "-98332.0#21",
///         "-0.01416063310170989651#60",
///         "-0.023#2",
///         "-2.11#8",
///         "-4.00003028288443785#57",
///         "-0.000057221276833275#43",
///         "-0.25000005983747242139#63",
///         "-24576.0#12",
///         "-3.0e4#1",
///         "-1.98431#16",
///         "-33.5#12",
///         "-0.2#1",
///         "-0.0009768007#23",
///         "-1279.5#25",
///         "-0.125#7",
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
///     &[
///         "6.6e4#7",
///         "0.021484375#26",
///         "8.40496e6#19",
///         "0.0",
///         "0.0155065#16",
///         "0.03121951#20",
///         "3.94#6",
///         "2.0038#15",
///         "0.6171265#21",
///         "16383.978515147231406#61",
///         "12.0#14",
///         "0.0195310124#31",
///         "380.00023#25",
///         "0.0",
///         "1511.5#12",
///         "0.00091559#14",
///         "32799.999752045#46",
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
///     &[
///         "-6.6e4#7",
///         "-0.021484375#26",
///         "-8.40496e6#19",
///         "-0.0",
///         "-0.0155065#16",
///         "-0.03121951#20",
///         "-3.94#6",
///         "-2.0038#15",
///         "-0.6171265#21",
///         "-16383.978515147231406#61",
///         "-12.0#14",
///         "-0.0195310124#31",
///         "-380.00023#25",
///         "-0.0",
///         "-1511.5#12",
///         "-0.00091559#14",
///         "-32799.999752045#46",
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
///     &[
///         "-6.6e4#7",
///         "-0.021484375#26",
///         "-8.40496e6#19",
///         "0.0155065#16",
///         "-0.03121951#20",
///         "3.94#6",
///         "-2.0038#15",
///         "-0.6171265#21",
///         "-16383.978515147231406#61",
///         "12.0#14",
///         "0.0195310124#31",
///         "380.00023#25",
///         "-1511.5#12",
///         "-0.00091559#14",
///         "32799.999752045#46",
///         "-0.0000305#6",
///         "-24.0#2",
///         "-64.0#9",
///         "-7.6e2#7",
///         "-287.76562497#34"
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
///     &[
///         "-3.8921#14",
///         "-2.60770320922795e-8#47",
///         "-0.09375#11",
///         "527.999999754#38",
///         "-0.00051#7",
///         "1.00385#17",
///         "-1.9e-6#3",
///         "-5.2427e5#16",
///         "-0.000440707#18",
///         "7.8#5",
///         "0.0",
///         "0.12451#12",
///         "-1.992187#21",
///         "-3.0e9#2",
///         "0.0625#8",
///         "-0.22#3",
///         "-0.01562#11",
///         "-3.8e-6#4",
///         "-64.0#13",
///         "-4064.0#19"
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
///     &[
///         "7.9998#15",
///         "32.8#9",
///         "0.0",
///         "NaN",
///         "-0.00005#2",
///         "-5.0e2#1",
///         "-0.1249#10",
///         "-127.4999852#28",
///         "0.4999999#22",
///         "0.000243902209#28",
///         "-0.11719#11",
///         "-9.96875#23",
///         "-15.9844663292160586998132#75",
///         "Infinity",
///         "-0.48#5",
///         "-1.41e-11#5",
///         "-262144.0#21",
///         "NaN",
///         "8.875#12",
///         "-0.0",
///         "-0.00586#7",
///         "0.06#1",
///         "0.12695307#22",
///         "0.00006098#10",
///         "-3.073363e-8#22",
///         "1024.0#9",
///         "-3.1519e7#13",
///         "483.9384763#31",
///         "9.8324e-7#17",
///         "Infinity",
///         "-24.0#6",
///         "-4.0e-15#1",
///         "-Infinity",
///         "0.6083984445#31",
///         "-1.0e3#4",
///         "-0.0000153#7",
///         "-1.0e5#2",
///         "3.8297144e-6#24",
///         "-0.0001235#10",
///         "-0.0",
///         "-23.94#9",
///         "0.2#1",
///         "-0.007326125891#31",
///         "3.818422533722416844e-6#61",
///         "-0.00048828123727#34",
///         "0.0000151538#16",
///         "NaN",
///         "-0.0",
///         "7.0e7#1",
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
