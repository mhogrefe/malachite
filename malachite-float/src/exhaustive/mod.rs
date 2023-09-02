use crate::Float;
use crate::InnerFloat::Finite;
use malachite_base::num::arithmetic::traits::{NegModPowerOf2, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::exhaustive::{
    exhaustive_signeds, primitive_int_increasing_inclusive_range, PrimitiveIntIncreasingRange,
    PrimitiveIntUpDown,
};
use malachite_base::num::iterators::{ruler_sequence, RulerSequence};
use malachite_base::num::logic::traits::{LowMask, NotAssign};
use malachite_base::tuples::exhaustive::{
    exhaustive_dependent_pairs, lex_dependent_pairs, ExhaustiveDependentPairs,
    ExhaustiveDependentPairsYsGenerator, LexDependentPairs,
};
use malachite_nz::natural::exhaustive::{
    exhaustive_natural_inclusive_range, ExhaustiveNaturalRange,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::iter::{once, Chain, Once};
use std::mem::swap;
use std::vec::IntoIter;

/// Generates all finite positive [`Float`]s with a specified `sci_exponent` (one less than the raw
/// exponent) and precision.
///
/// This `struct` is created by [`exhaustive_floats_with_sci_exponent_and_precision`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveFloatsWithSciExponentAndPrecision {
    exponent: i64,
    precision: u64,
    shift: u64,
    significands: ExhaustiveNaturalRange,
}

impl Iterator for ExhaustiveFloatsWithSciExponentAndPrecision {
    type Item = Float;

    fn next(&mut self) -> Option<Float> {
        self.significands.next().map(|s| {
            Float(Finite {
                sign: true,
                exponent: self.exponent,
                precision: self.precision,
                significand: s << self.shift,
            })
        })
    }
}

/// Generates all finite positive [`Float`]s with a specified `sci_exponent` (one less than the raw
/// exponent) and precision.
///
/// Positive and negative zero are both excluded.
///
/// A finite positive [`Float`] may be uniquely expressed as $x = m_s2^e_s$, where $1 \leq m_s < 2$
/// and $e_s$ is an integer; then $e_s$ is the sci-exponent.
///
/// The output length is $2^{p-1}$.
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
///
/// # Panics
/// Panics if the precision is zero.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_float::exhaustive::exhaustive_floats_with_sci_exponent_and_precision;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     exhaustive_floats_with_sci_exponent_and_precision(0, 4)
///         .map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &["1.0#4", "1.1#4", "1.2#4", "1.4#4", "1.5#4", "1.6#4", "1.8#4", "1.9#4"]
/// );
///
/// assert_eq!(
///     exhaustive_floats_with_sci_exponent_and_precision(2, 5)
///         .map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "4.0#5", "4.2#5", "4.5#5", "4.8#5", "5.0#5", "5.2#5", "5.5#5", "5.8#5", "6.0#5",
///         "6.2#5", "6.5#5", "6.8#5", "7.0#5", "7.2#5", "7.5#5", "7.8#5"
///     ]
/// );
/// ```
pub fn exhaustive_floats_with_sci_exponent_and_precision(
    sci_exponent: i64,
    prec: u64,
) -> ExhaustiveFloatsWithSciExponentAndPrecision {
    assert_ne!(prec, 0);
    ExhaustiveFloatsWithSciExponentAndPrecision {
        exponent: sci_exponent + 1,
        precision: prec,
        shift: prec.neg_mod_power_of_2(Limb::LOG_WIDTH),
        significands: exhaustive_natural_inclusive_range(
            Natural::power_of_2(prec - 1),
            Natural::low_mask(prec),
        ),
    }
}

#[derive(Clone, Debug)]
struct FloatsWithSciExponentAndPrecisionGenerator {
    sci_exponent: i64,
}

impl ExhaustiveDependentPairsYsGenerator<u64, Float, ExhaustiveFloatsWithSciExponentAndPrecision>
    for FloatsWithSciExponentAndPrecisionGenerator
{
    #[inline]
    fn get_ys(&self, &prec: &u64) -> ExhaustiveFloatsWithSciExponentAndPrecision {
        exhaustive_floats_with_sci_exponent_and_precision(self.sci_exponent, prec)
    }
}

#[inline]
fn exhaustive_floats_with_sci_exponent_helper(
    sci_exponent: i64,
) -> LexDependentPairs<
    u64,
    Float,
    FloatsWithSciExponentAndPrecisionGenerator,
    PrimitiveIntIncreasingRange<u64>,
    ExhaustiveFloatsWithSciExponentAndPrecision,
> {
    lex_dependent_pairs(
        primitive_int_increasing_inclusive_range(1, u64::MAX),
        FloatsWithSciExponentAndPrecisionGenerator { sci_exponent },
    )
}

/// Generates all finite positive [`Float`]s with a specified `sci_exponent` (one less than the raw
/// exponent).
///
/// This `struct` is created by [`exhaustive_floats_with_sci_exponent`]; see its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct ExhaustiveFloatsWithSciExponent(
    LexDependentPairs<
        u64,
        Float,
        FloatsWithSciExponentAndPrecisionGenerator,
        PrimitiveIntIncreasingRange<u64>,
        ExhaustiveFloatsWithSciExponentAndPrecision,
    >,
);

impl Iterator for ExhaustiveFloatsWithSciExponent {
    type Item = Float;

    #[inline]
    fn next(&mut self) -> Option<Float> {
        self.0.next().map(|p| p.1)
    }
}

/// Generates all finite positive [`Float`]s with a specified `sci_exponent` (one less than the raw
/// exponent).
///
/// Positive and negative zero are both excluded.
///
/// A finite positive [`Float`] may be uniquely expressed as $x = m_s2^e_s$, where $1 \leq m_s < 2$
/// and $e_s$ is an integer; then $e_s$ is the sci-exponent.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// # Panics
/// Panics if the precision is zero.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_float::exhaustive::exhaustive_floats_with_sci_exponent;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     exhaustive_floats_with_sci_exponent(0)
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "1.0#1", "1.0#2", "1.5#2", "1.0#3", "1.2#3", "1.5#3", "1.8#3", "1.0#4", "1.1#4",
///         "1.2#4", "1.4#4", "1.5#4", "1.6#4", "1.8#4", "1.9#4", "1.0#5", "1.06#5", "1.12#5",
///         "1.19#5", "1.25#5"
///     ]
/// );
///
/// assert_eq!(
///     exhaustive_floats_with_sci_exponent(2)
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "4.0#1", "4.0#2", "6.0#2", "4.0#3", "5.0#3", "6.0#3", "7.0#3", "4.0#4", "4.5#4",
///         "5.0#4", "5.5#4", "6.0#4", "6.5#4", "7.0#4", "7.5#4", "4.0#5", "4.2#5", "4.5#5",
///         "4.8#5", "5.0#5"
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_floats_with_sci_exponent(sci_exponent: i64) -> ExhaustiveFloatsWithSciExponent {
    ExhaustiveFloatsWithSciExponent(exhaustive_floats_with_sci_exponent_helper(sci_exponent))
}

#[derive(Clone, Debug)]
struct ExhaustivePositiveFiniteFloatsGenerator;

impl ExhaustiveDependentPairsYsGenerator<i64, Float, ExhaustiveFloatsWithSciExponent>
    for ExhaustivePositiveFiniteFloatsGenerator
{
    #[inline]
    fn get_ys(&self, &sci_exponent: &i64) -> ExhaustiveFloatsWithSciExponent {
        exhaustive_floats_with_sci_exponent(sci_exponent)
    }
}

#[inline]
fn exhaustive_positive_finite_floats_helper() -> ExhaustiveDependentPairs<
    i64,
    Float,
    RulerSequence<usize>,
    ExhaustivePositiveFiniteFloatsGenerator,
    Chain<Once<i64>, PrimitiveIntUpDown<i64>>,
    ExhaustiveFloatsWithSciExponent,
> {
    exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_signeds(),
        ExhaustivePositiveFiniteFloatsGenerator,
    )
}

/// Generates all positive finite [`Float`]s.
///
/// This `struct` is created by [`exhaustive_positive_finite_floats`]; see its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct ExhaustivePositiveFiniteFloats(
    ExhaustiveDependentPairs<
        i64,
        Float,
        RulerSequence<usize>,
        ExhaustivePositiveFiniteFloatsGenerator,
        Chain<Once<i64>, PrimitiveIntUpDown<i64>>,
        ExhaustiveFloatsWithSciExponent,
    >,
);

impl Iterator for ExhaustivePositiveFiniteFloats {
    type Item = Float;

    #[inline]
    fn next(&mut self) -> Option<Float> {
        self.0.next().map(|p| p.1)
    }
}

/// Generates all positive finite [`Float`]s.
///
/// Positive and negative zero are both excluded.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// ```
/// use itertools::Itertools;
/// use malachite_float::exhaustive::exhaustive_positive_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     exhaustive_positive_finite_floats()
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "1.0#1", "2.0#1", "1.0#2", "0.5#1", "1.5#2", "2.0#2", "1.0#3", "4.0#1", "1.2#3",
///         "3.0#2", "1.5#3", "0.5#2", "1.8#3", "2.0#3", "1.0#4", "0.2#1", "1.1#4", "2.5#3",
///         "1.2#4", "0.8#2"
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_positive_finite_floats() -> ExhaustivePositiveFiniteFloats {
    ExhaustivePositiveFiniteFloats(exhaustive_positive_finite_floats_helper())
}

/// Generates all negative finite [`Float`]s.
///
/// This `struct` is created by [`exhaustive_negative_finite_floats`]; see its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNegativeFiniteFloats(ExhaustivePositiveFiniteFloats);

impl Iterator for ExhaustiveNegativeFiniteFloats {
    type Item = Float;

    #[inline]
    fn next(&mut self) -> Option<Float> {
        self.0.next().map(|f| -f)
    }
}

/// Generates all negative finite [`Float`]s.
///
/// Positive and negative zero are both excluded.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// ```
/// use itertools::Itertools;
/// use malachite_float::exhaustive::exhaustive_negative_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     exhaustive_negative_finite_floats()
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "-1.0#1", "-2.0#1", "-1.0#2", "-0.5#1", "-1.5#2", "-2.0#2", "-1.0#3", "-4.0#1",
///         "-1.2#3", "-3.0#2", "-1.5#3", "-0.5#2", "-1.8#3", "-2.0#3", "-1.0#4", "-0.2#1",
///         "-1.1#4", "-2.5#3", "-1.2#4", "-0.8#2"
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_negative_finite_floats() -> ExhaustiveNegativeFiniteFloats {
    ExhaustiveNegativeFiniteFloats(exhaustive_positive_finite_floats())
}

/// Generates all nonzero finite [`Float`]s.
///
/// This `struct` is created by [`exhaustive_negative_finite_floats`]; see its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNonzeroFiniteFloats {
    toggle: bool,
    xs: ExhaustivePositiveFiniteFloats,
    x: Float,
}

impl Iterator for ExhaustiveNonzeroFiniteFloats {
    type Item = Float;

    #[inline]
    fn next(&mut self) -> Option<Float> {
        self.toggle.not_assign();
        Some(if self.toggle {
            self.x = self.xs.next().unwrap();
            self.x.clone()
        } else {
            let mut out = Float::NAN;
            swap(&mut out, &mut self.x);
            -out
        })
    }
}

/// Generates all nonzero finite [`Float`]s.
///
/// Positive and negative zero are both excluded.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// ```
/// use itertools::Itertools;
/// use malachite_float::exhaustive::exhaustive_nonzero_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     exhaustive_nonzero_finite_floats()
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "1.0#1", "-1.0#1", "2.0#1", "-2.0#1", "1.0#2", "-1.0#2", "0.5#1", "-0.5#1", "1.5#2",
///         "-1.5#2", "2.0#2", "-2.0#2", "1.0#3", "-1.0#3", "4.0#1", "-4.0#1", "1.2#3", "-1.2#3",
///         "3.0#2", "-3.0#2"
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_nonzero_finite_floats() -> ExhaustiveNonzeroFiniteFloats {
    ExhaustiveNonzeroFiniteFloats {
        toggle: false,
        xs: exhaustive_positive_finite_floats(),
        x: Float::NAN,
    }
}

type ExhaustiveNonNegativeFiniteFloats = Chain<Once<Float>, ExhaustivePositiveFiniteFloats>;

/// Generates all non-negative finite [`Float`]s.
///
/// Positive zero is included, but negative zero is not.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// ```
/// use itertools::Itertools;
/// use malachite_float::exhaustive::exhaustive_non_negative_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     exhaustive_non_negative_finite_floats()
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "0.0", "1.0#1", "2.0#1", "1.0#2", "0.5#1", "1.5#2", "2.0#2", "1.0#3", "4.0#1", "1.2#3",
///         "3.0#2", "1.5#3", "0.5#2", "1.8#3", "2.0#3", "1.0#4", "0.2#1", "1.1#4", "2.5#3",
///         "1.2#4"
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_non_negative_finite_floats() -> ExhaustiveNonNegativeFiniteFloats {
    once(Float::ZERO).chain(exhaustive_positive_finite_floats())
}

type ExhaustiveNonPositiveFiniteFloats = Chain<Once<Float>, ExhaustiveNegativeFiniteFloats>;

/// Generates all non-positive finite [`Float`]s.
///
/// Negative zero is included, but positive zero is not.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// ```
/// use itertools::Itertools;
/// use malachite_float::exhaustive::exhaustive_non_positive_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     exhaustive_non_positive_finite_floats()
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "-0.0", "-1.0#1", "-2.0#1", "-1.0#2", "-0.5#1", "-1.5#2", "-2.0#2", "-1.0#3", "-4.0#1",
///         "-1.2#3", "-3.0#2", "-1.5#3", "-0.5#2", "-1.8#3", "-2.0#3", "-1.0#4", "-0.2#1",
///         "-1.1#4", "-2.5#3", "-1.2#4"
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_non_positive_finite_floats() -> ExhaustiveNonPositiveFiniteFloats {
    once(Float::NEGATIVE_ZERO).chain(exhaustive_negative_finite_floats())
}

type ExhaustiveFloats = Chain<IntoIter<Float>, ExhaustiveNonzeroFiniteFloats>;

/// Generates all finite [`Float`]s.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// ```
/// use itertools::Itertools;
/// use malachite_float::exhaustive::exhaustive_finite_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     exhaustive_finite_floats()
///         .take(20).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "0.0", "-0.0", "1.0#1", "-1.0#1", "2.0#1", "-2.0#1", "1.0#2", "-1.0#2", "0.5#1",
///         "-0.5#1", "1.5#2", "-1.5#2", "2.0#2", "-2.0#2", "1.0#3", "-1.0#3", "4.0#1", "-4.0#1",
///         "1.2#3", "-1.2#3"
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_finite_floats() -> ExhaustiveFloats {
    vec![Float::ZERO, Float::NEGATIVE_ZERO]
        .into_iter()
        .chain(exhaustive_nonzero_finite_floats())
}

/// Generates all [`Float`]s.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\log i)$
///
/// $M(i) = O(\log i)$
///
/// where $T$ is time, $M$ is additional memory, and $i$ is the iteration number.
///
/// ```
/// use itertools::Itertools;
/// use malachite_float::exhaustive::exhaustive_floats;
/// use malachite_float::ComparableFloat;
///
/// // The number after the '#' is the precision.
/// assert_eq!(
///     exhaustive_floats()
///         .take(50).map(|f| ComparableFloat(f).to_string()).collect_vec().as_slice(),
///     &[
///         "NaN", "Infinity", "-Infinity", "0.0", "-0.0", "1.0#1", "-1.0#1", "2.0#1", "-2.0#1",
///         "1.0#2", "-1.0#2", "0.5#1", "-0.5#1", "1.5#2", "-1.5#2", "2.0#2", "-2.0#2", "1.0#3",
///         "-1.0#3", "4.0#1", "-4.0#1", "1.2#3", "-1.2#3", "3.0#2", "-3.0#2", "1.5#3", "-1.5#3",
///         "0.5#2", "-0.5#2", "1.8#3", "-1.8#3", "2.0#3", "-2.0#3", "1.0#4", "-1.0#4", "0.2#1",
///         "-0.2#1", "1.1#4", "-1.1#4", "2.5#3", "-2.5#3", "1.2#4", "-1.2#4", "0.8#2", "-0.8#2",
///         "1.4#4", "-1.4#4", "3.0#3", "-3.0#3", "1.5#4"
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_floats() -> ExhaustiveFloats {
    vec![Float::NAN, Float::INFINITY, Float::NEGATIVE_INFINITY, Float::ZERO, Float::NEGATIVE_ZERO]
        .into_iter()
        .chain(exhaustive_nonzero_finite_floats())
}
