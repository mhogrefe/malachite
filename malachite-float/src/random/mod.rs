use crate::Float;
use crate::InnerFloat::Finite;
use malachite_base::bools::random::{
    random_bools, weighted_random_bools, RandomBools, WeightedRandomBools,
};
use malachite_base::iterators::{with_special_values, WithSpecialValues};
use malachite_base::num::arithmetic::traits::NegModPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::random::geometric::{
    geometric_random_signeds, GeometricRandomNaturalValues, GeometricRandomSigneds,
};
use malachite_base::random::Seed;
use malachite_nz::natural::random::{
    random_positive_naturals, striped_random_positive_naturals, RandomNaturals,
    StripedRandomNaturals,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

/// Generates random positive finite [`Float`]s.
///
/// This `struct` is created by [`random_positive_finite_floats`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomPositiveFiniteFloats<I: Iterator<Item = Natural>> {
    exponents: GeometricRandomSigneds<i64>,
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
            exponent: self.exponents.next().unwrap(),
            precision,
            significand: x << precision.neg_mod_power_of_2(Limb::LOG_WIDTH),
        }))
    }
}

/// Generates random positive finite [`Float`]s.
///
/// Simpler [`Float`]s (those with a lower absolute sci-exponent or precision) are more likely to
/// be chosen. You can specify the mean absolute sci-exponent and precision by passing the
/// numerators and denominators of their means.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The
/// practical implications are that
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
/// use malachite_float::ComparableFloat;
/// use malachite_float::random::random_negative_finite_floats;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_negative_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1)
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "-0.44#3", "-6.6e-7#6", "-0.004#1", "-0.2#1", "-4.107e4#13", "-0.00779413723#29",
///         "-0.008#1", "-1.7#7", "-2.2990855826#33", "-0.000016716029#23", "-0.169649838688#37",
///         "-1.33e4#7", "-2.0e4#1", "-0.699#8", "-18.7#9", "-0.1#1", "-0.0005554#13", "-533.0#10",
///         "-0.092#7", "-0.000666152806#28"
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
/// Simpler [`Float`]s (those with a lower absolute sci-exponent or precision) are more likely to
/// be chosen. You can specify the mean absolute sci-exponent and precision by passing the
/// numerators and denominators of their means.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The
/// practical implications are that
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
/// use malachite_float::ComparableFloat;
/// use malachite_float::random::random_positive_finite_floats;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_positive_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1)
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "0.44#3", "6.6e-7#6", "0.004#1", "0.2#1", "4.107e4#13", "0.00779413723#29", "0.008#1",
///         "1.7#7", "2.2990855826#33", "0.000016716029#23", "0.169649838688#37", "1.33e4#7",
///         "2.0e4#1", "0.699#8", "18.7#9", "0.1#1", "0.0005554#13", "533.0#10", "0.092#7",
///         "0.000666152806#28"
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
/// Simpler [`Float`]s (those with a lower absolute sci-exponent or precision) are more likely to
/// be chosen. You can specify the numerator and denominator of the probability that a zero will be
/// generated. You can also specify the mean absolute sci-exponent and precision by passing the
/// numerators and denominators of their means of the nonzero [`Float`]s.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The
/// practical implications are that
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
/// use malachite_float::ComparableFloat;
/// use malachite_float::random::random_non_negative_finite_floats;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_non_negative_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "5.5e4#5", "0.0155402#17", "4.7969e6#14", "0.0", "0.0063#5", "0.009216#11", "1.0#5",
///         "1.541#10", "0.437477#16", "5144.14763838#38", "4.61#10", "0.015024275#23",
///         "155.7261#19", "0.0", "536.0#7", "0.000483#9", "29579.761#27", "0.0", "0.0000176#6",
///         "8.0#1"
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
/// Simpler [`Float`]s (those with a lower absolute sci-exponent or precision) are more likely to
/// be chosen. You can specify the numerator and denominator of the probability that a zero will be
/// generated. You can also specify the mean absolute sci-exponent and precision by passing the
/// numerators and denominators of their means of the nonzero [`Float`]s.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The
/// practical implications are that
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
/// use malachite_float::ComparableFloat;
/// use malachite_float::random::random_non_positive_finite_floats;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_non_positive_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "-5.5e4#5", "-0.0155402#17", "-4.7969e6#14", "-0.0", "-0.0063#5", "-0.009216#11",
///         "-1.0#5", "-1.541#10", "-0.437477#16", "-5144.14763838#38", "-4.61#10",
///         "-0.015024275#23", "-155.7261#19", "-0.0", "-536.0#7", "-0.000483#9", "-29579.761#27",
///         "-0.0", "-0.0000176#6", "-8.0#1"
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
/// This `struct` is created by [`random_nonzero_finite_floats`]; see its documentation for
/// more.
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
/// Simpler [`Float`]s (those with a lower absolute sci-exponent or precision) are more likely to
/// be chosen. You can specify the mean absolute sci-exponent and precision by passing the
/// numerators and denominators of their means.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The
/// practical implications are that
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
/// use malachite_float::ComparableFloat;
/// use malachite_float::random::random_nonzero_finite_floats;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_nonzero_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1)
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "-5.5e4#5", "-0.0155402#17", "-4.7969e6#14", "0.0063#5", "-0.009216#11", "1.0#5",
///         "-1.541#10", "-0.437477#16", "-5144.14763838#38", "4.61#10", "0.015024275#23",
///         "155.7261#19", "-536.0#7", "-0.000483#9", "29579.761#27", "-0.0000176#6", "-8.0#1",
///         "-60.0#5", "-4.8e2#5", "-179.1201#20"
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
/// Simpler [`Float`]s (those with a lower absolute sci-exponent or precision) are more likely to
/// be chosen. You can specify the numerator and denominator of the probability that a zero will be
/// generated. You can also specify the mean absolute sci-exponent and precision by passing the
/// numerators and denominators of their means of the nonzero [`Float`]s.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The
/// practical implications are that
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
/// use malachite_float::ComparableFloat;
/// use malachite_float::random::random_finite_floats;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_finite_floats(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "-1.22#7", "-1.161697943e-8#30", "-0.043#6", "504.6885#20", "-0.00041#6", "0.99#10",
///         "-9.5e-7#3", "-1.341e5#14", "-0.0001693#10", "3.0#2", "0.0", "0.051#5", "-0.6832#13",
///         "-1.6e9#2", "0.059#4", "-0.09#2", "-0.015#7", "-2.0e-6#2", "-57.0#6", "-2001.0#13"
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
/// Simpler [`Float`]s (those with a lower absolute sci-exponent or precision) are more likely to
/// be chosen. You can specify the numerator and denominator of the probability that a zero, an
/// infinity, or a NaN will be generated. You can also specify the mean absolute sci-exponent and
/// precision by passing the numerators and denominators of their means of the nonzero [`Float`]s.
///
/// But note that the specified means are only approximate, since the distributions we are sampling
/// are truncated geometric, and their exact means are somewhat annoying to deal with. The
/// practical implications are that
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
/// use malachite_float::ComparableFloat;
/// use malachite_float::random::random_floats;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     random_floats(EXAMPLE_SEED, 10, 1, 10, 1, 1, 10)
///         .take(50).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "3.602#10", "19.6#8", "0.0", "NaN", "-0.000015#2", "-3.0e2#1", "-0.0439#8",
///         "-47.5601#17", "0.19038#14", "0.000069018#15", "-0.0547#7", "-5.156#12",
///         "-6.841984502561296#51", "Infinity", "-0.17#4", "-3.6e-12#5", "-197292.0#16", "NaN",
///         "6.8#5", "-0.0", "-0.0032#5", "0.03#1", "0.09467#12", "0.00002#6", "-2.4094e-8#13",
///         "5.8e2#6", "-9.6e6#7", "237.867#17", "5.5e-7#7", "Infinity", "-12.0#3", "-2.0e-15#1",
///         "-Infinity", "0.252#11", "-5.0e2#3", "-0.0000141#6", "-9.8e4#2", "3.21589e-6#20",
///         "-0.000095#5", "-0.0", "-15.0#4", "0.1#1", "-0.00314969#18", "2.29139385931e-6#38",
///         "-0.0001353542#19", "6.564e-6#10", "NaN", "-0.0", "3.0e7#1", "10131.8#16"
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
/// `mean_stripe_numerator / mean_stripe_denominator`; $m$ must be greater than 0. A striped bit
/// sequence with the given stripe parameter is generated and truncated at the bit length. The
/// highest bit is forced to be 1, and the [`Float`] is generated from the sequence and a random
/// sci-exponent.
///
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for
/// information about generating striped random numbers.
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
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator < mean_stripe_denominator`, if `mean_precision_numerator` or
/// `mean_precision_denominator` are zero, or, if after being reduced to lowest terms, their sum is
/// greater than or equal to $2^{64}$.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::ComparableFloat;
/// use malachite_float::random::striped_random_positive_finite_floats;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_positive_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1)
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "0.47#4", "9.532e-7#11", "0.004#2", "0.25#3", "49166.0#21",
///         "0.007080316550854948256#60", "0.01#2", "1.055#8", "2.00001514144221892#57",
///         "0.000028610638416637#43", "0.12500002991873621069#63", "12288.0#12", "2.0e4#1",
///         "0.99216#16", "16.75#12", "0.1#1", "0.0004884003#23", "639.75#25", "0.0625#7",
///         "0.0007324367693311#42"
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

/// Generates striped random negative finite [`Float`]s.
///
/// The actual precision is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_stripe_numerator / mean_stripe_denominator`; $m$ must be greater than 0. A striped bit
/// sequence with the given stripe parameter is generated and truncated at the bit length. The
/// highest bit is forced to be 1, and the [`Float`] is generated from the sequence and a random
/// sci-exponent.
///
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for
/// information about generating striped random numbers.
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
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator < mean_stripe_denominator`, if `mean_precision_numerator` or
/// `mean_precision_denominator` are zero, or, if after being reduced to lowest terms, their sum is
/// greater than or equal to $2^{64}$.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::ComparableFloat;
/// use malachite_float::random::striped_random_negative_finite_floats;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_negative_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1)
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "-0.47#4", "-9.532e-7#11", "-0.004#2", "-0.25#3", "-49166.0#21",
///         "-0.007080316550854948256#60", "-0.01#2", "-1.055#8", "-2.00001514144221892#57",
///         "-0.000028610638416637#43", "-0.12500002991873621069#63", "-12288.0#12", "-2.0e4#1",
///         "-0.99216#16", "-16.75#12", "-0.1#1", "-0.0004884003#23", "-639.75#25", "-0.0625#7",
///         "-0.0007324367693311#42"
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
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for
/// information about generating striped random numbers.
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
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator < mean_stripe_denominator`, if `mean_precision_numerator` or
/// `mean_precision_denominator` are zero, or, if after being reduced to lowest terms, their sum is
/// greater than or equal to $2^{64}$.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::ComparableFloat;
/// use malachite_float::random::striped_random_non_negative_finite_floats;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_non_negative_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1, 1, 10)
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "3.28e4#7", "0.0107421875#26", "4.20248e6#19", "0.0", "0.0077533#16", "0.01560976#20",
///         "1.97#6", "1.0019#15", "0.3085632#21", "8191.989257573615703#61", "6.0#14",
///         "0.0097655062#31", "190.000114#25", "0.0", "755.8#12", "0.00045779#14",
///         "16399.9998760223#46", "0.0", "0.0000153#6", "1.0e1#2"
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
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for
/// information about generating striped random numbers.
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
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator < mean_stripe_denominator`, if `mean_precision_numerator` or
/// `mean_precision_denominator` are zero, or, if after being reduced to lowest terms, their sum is
/// greater than or equal to $2^{64}$.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::ComparableFloat;
/// use malachite_float::random::striped_random_non_positive_finite_floats;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_non_positive_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1, 1, 10)
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "-3.28e4#7", "-0.0107421875#26", "-4.20248e6#19", "-0.0", "-0.0077533#16",
///         "-0.01560976#20", "-1.97#6", "-1.0019#15", "-0.3085632#21", "-8191.989257573615703#61",
///         "-6.0#14", "-0.0097655062#31", "-190.000114#25", "-0.0", "-755.8#12", "-0.00045779#14",
///         "-16399.9998760223#46", "-0.0", "-0.0000153#6", "-1.0e1#2"
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
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for
/// information about generating striped random numbers.
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
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator < mean_stripe_denominator`, if `mean_precision_numerator` or
/// `mean_precision_denominator` are zero, or, if after being reduced to lowest terms, their sum is
/// greater than or equal to $2^{64}$.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::ComparableFloat;
/// use malachite_float::random::striped_random_nonzero_finite_floats;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_nonzero_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1)
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "-3.28e4#7", "-0.0107421875#26", "-4.20248e6#19", "0.0077533#16", "-0.01560976#20",
///         "1.97#6", "-1.0019#15", "-0.3085632#21", "-8191.989257573615703#61", "6.0#14",
///         "0.0097655062#31", "190.000114#25", "-755.8#12", "-0.00045779#14",
///         "16399.9998760223#46", "-0.0000153#6", "-1.0e1#2", "-32.0#9", "-380.0#7",
///         "-143.88281249#34"
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
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for
/// information about generating striped random numbers.
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
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator < mean_stripe_denominator`, if `mean_precision_numerator` or
/// `mean_precision_denominator` are zero, or, if after being reduced to lowest terms, their sum is
/// greater than or equal to $2^{64}$.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::ComparableFloat;
/// use malachite_float::random::striped_random_finite_floats;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_finite_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1, 1, 10)
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "-1.946#14", "-1.30385160461398e-8#47", "-0.04688#11", "263.999999877#38",
///         "-0.000256#7", "0.50192#17", "-9.5e-7#3", "-262136.0#16", "-0.000220354#18", "3.9#5",
///         "0.0", "0.06226#12", "-0.9960933#21", "-1.6e9#2", "0.0312#8", "-0.11#3", "-0.00781#11",
///         "-1.9e-6#4", "-32.0#13", "-2032.0#19"
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
/// Special values (NaN, infinities, and zeros) are generated with the specified probability. If
/// the [`Float`] to be generated is finite and nonzero, then the actual precision is chosen from a
/// geometric distribution with mean $m$, where $m$ is
/// `mean_stripe_numerator / mean_stripe_denominator`; $m$ must be greater than 0. A striped bit
/// sequence with the given stripe parameter is generated and truncated at the bit length. The
/// highest bit is forced to be 1, and the [`Float`] is generated from the sequence and a random
/// sci-exponent.
///
/// See [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource) for
/// information about generating striped random numbers.
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
/// Panics if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator < mean_stripe_denominator`, if `mean_precision_numerator` or
/// `mean_precision_denominator` are zero, or, if after being reduced to lowest terms, their sum is
/// greater than or equal to $2^{64}$.
///
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_float::ComparableFloat;
/// use malachite_float::random::striped_random_floats;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     striped_random_floats(EXAMPLE_SEED, 10, 1, 8, 1, 16, 1, 1, 10)
///         .take(50).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "3.9999#15", "16.38#9", "0.0", "NaN", "-0.000023#2", "-3.0e2#1", "-0.06244#10",
///         "-63.7499926#28", "0.24999994#22", "0.0001219511046#28", "-0.05859#11", "-4.984375#23",
///         "-7.9922331646080293499066#75", "Infinity", "-0.24#5", "-7.0e-12#5", "-131072.0#21",
///         "NaN", "4.438#12", "-0.0", "-0.00293#7", "0.03#1", "0.06347653#22", "0.00003049#10",
///         "-1.5366815e-8#22", "512.0#9", "-1.5759e7#13", "241.9692382#31", "4.9162e-7#17",
///         "Infinity", "-12.0#6", "-2.0e-15#1", "-Infinity", "0.3041992222#31", "-5.0e2#4",
///         "-7.6e-6#7", "-7.0e4#2", "1.9148572e-6#24", "-0.0000618#10", "-0.0", "-11.97#9",
///         "0.1#1", "-0.003663062946#31", "1.909211266861208422e-6#61", "-0.00024414061863#34",
///         "7.5769e-6#16", "NaN", "-0.0", "3.0e7#1", "10211.992188#33"
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
