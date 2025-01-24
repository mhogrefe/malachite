// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::arithmetic::denominators_in_closed_interval::DenominatorsInClosedRationalInterval;
use crate::arithmetic::traits::DenominatorsInClosedInterval;
use crate::exhaustive::RationalsWithDenominator;
use crate::Rational;
use core::cmp::min;
use malachite_base::bools::random::{random_bools, RandomBools};
use malachite_base::iterators::iterator_cache::IteratorCache;
use malachite_base::num::arithmetic::traits::{
    CoprimeWith, Reciprocal, RoundToMultiple, UnsignedAbs,
};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::random::geometric::{
    geometric_random_unsigneds, GeometricRandomNaturalValues,
};
use malachite_base::num::random::striped::StripedBitSource;
use malachite_base::num::random::{
    random_primitive_ints, RandomPrimitiveInts, VariableRangeGenerator,
};
use malachite_base::random::Seed;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::integer::random::{
    get_random_integer_from_range_to_infinity, get_random_integer_from_range_to_negative_infinity,
    get_striped_random_integer_from_range_to_infinity,
    get_striped_random_integer_from_range_to_negative_infinity, random_integer_range,
    random_integer_range_to_infinity, random_integer_range_to_negative_infinity,
    RandomIntegerRange, RandomIntegerRangeToInfinity,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::random::{
    random_naturals, random_positive_naturals, striped_random_naturals,
    striped_random_positive_naturals, RandomNaturals, StripedRandomNaturals,
};
use malachite_nz::natural::Natural;
use std::collections::HashMap;

/// Generates random non-negative [`Rational`]s, given an iterator of random [`Natural`] numerators
/// and denominators.
#[derive(Clone, Debug)]
pub struct RandomRationalsFromSingle<I: Iterator<Item = Natural>> {
    xs: I,
}

impl<I: Iterator<Item = Natural>> Iterator for RandomRationalsFromSingle<I> {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        Some(Rational::from_naturals(
            self.xs.next().unwrap(),
            self.xs.next().unwrap(),
        ))
    }
}

/// Generates random positive [`Rational`]s with a specified numerator and denominator mean bit
/// length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 1. Then the numerator
/// and denominator are chosen from all positive [`Natural`]s with that bit length.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_bits_numerator /
/// mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero or if `mean_bits_numerator
/// <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::random_positive_rationals;
///
/// assert_eq!(
///     prefix_to_string(random_positive_rationals(EXAMPLE_SEED, 32, 1), 10),
///     "[11/2, 89/27922830575, 46627409/3788983764809694, 8/11316951483471, \
///     11/1005760138411689342464923704482, 948931/42716754, \
///     81013760999253680590984897748479904878392/23, 1/97645164585502, 1558028859598/29, \
///     200127331174844881647/4058622214797175252, ...]"
/// )
/// ```
pub fn random_positive_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromSingle<RandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomRationalsFromSingle {
        xs: random_positive_naturals(seed, mean_bits_numerator, mean_bits_denominator),
    }
}

/// Generates random non-negative [`Rational`]s, given an iterator of random [`Natural`] numerators
/// and an iterator of random [`Natural`] denominators.
#[derive(Clone, Debug)]
pub struct RandomRationalsFromDouble<I: Iterator<Item = Natural>, J: Iterator<Item = Natural>> {
    xs: I,
    ys: J,
}

impl<I: Iterator<Item = Natural>, J: Iterator<Item = Natural>> Iterator
    for RandomRationalsFromDouble<I, J>
{
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        Some(Rational::from_naturals(
            self.xs.next().unwrap(),
            self.ys.next().unwrap(),
        ))
    }
}

/// Generates random non-negative [`Rational`]s with a specified numerator and denominator mean bit
/// length.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_bits_numerator /
/// mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero or if `mean_bits_numerator
/// <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::random_non_negative_rationals;
///
/// assert_eq!(
///     prefix_to_string(random_non_negative_rationals(EXAMPLE_SEED, 32, 1), 10),
///     "[7301/34, 4183103/1234731190583, 54812347098686/6195807891591254727, 812739/17841539017, \
///     665/908, 677/1138982845180, 166/22491855393807861245619791028129, 270142/5, \
///     52040856788711439301087669967/15975369961878544862054, 5718607/1953563256716085077, ...]"
/// )
/// ```
pub fn random_non_negative_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromDouble<
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
> {
    RandomRationalsFromDouble {
        xs: random_naturals(
            seed.fork("numerator"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        ys: random_positive_naturals(
            seed.fork("denominator"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates random negative [`Rational`]s, given an iterator of positive [`Rational`]s.
#[derive(Clone, Debug)]
pub struct NegativeRationals<I: Iterator<Item = Rational>> {
    xs: I,
}

impl<I: Iterator<Item = Rational>> Iterator for NegativeRationals<I> {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        self.xs.next().map(|mut q| {
            q.sign = false;
            q
        })
    }
}

/// Generates random negative [`Rational`]s with a specified numerator and denominator mean bit
/// length.
///
/// The actual bit length is chosen from a geometric distribution with mean $m$, where $m$ is
/// `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than 1. Then the numerator
/// and denominator are chosen from all positive [`Natural`]s with that bit length. Finally, the
/// resulting [`Rational`] is reduced and negated.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_bits_numerator /
/// mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero or if `mean_bits_numerator
/// <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::random_negative_rationals;
///
/// assert_eq!(
///     prefix_to_string(random_negative_rationals(EXAMPLE_SEED, 32, 1), 10),
///     "[-11/2, -89/27922830575, -46627409/3788983764809694, -8/11316951483471, \
///     -11/1005760138411689342464923704482, -948931/42716754, \
///     -81013760999253680590984897748479904878392/23, -1/97645164585502, -1558028859598/29, \
///     -200127331174844881647/4058622214797175252, ...]"
/// )
/// ```
pub fn random_negative_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> NegativeRationals<RandomRationalsFromSingle<RandomNaturals<GeometricRandomNaturalValues<u64>>>>
{
    NegativeRationals {
        xs: random_positive_rationals(seed, mean_bits_numerator, mean_bits_denominator),
    }
}

/// Generates random non-negative [`Rational`]s, given an iterator of random [`Natural`] numerators
/// and an iterator of [`bool`] signs.
#[derive(Clone, Debug)]
pub struct RandomRationalsFromSingleAndSign<I: Iterator<Item = Natural>> {
    bs: RandomBools,
    xs: I,
}

impl<I: Iterator<Item = Natural>> Iterator for RandomRationalsFromSingleAndSign<I> {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        Some(Rational::from_sign_and_naturals(
            self.bs.next().unwrap(),
            self.xs.next().unwrap(),
            self.xs.next().unwrap(),
        ))
    }
}

/// Generates random nonzero [`Rational`]s with a specified numerator and denominator mean bit
/// length.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_bits_numerator /
/// mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero or if `mean_bits_numerator
/// <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::random_nonzero_rationals;
///
/// assert_eq!(
///     prefix_to_string(random_nonzero_rationals(EXAMPLE_SEED, 32, 1), 10),
///     "[-80861953616/9687130509484985, -14557437513/313, 100721397389/392237929981, \
///     713431423/1285, -3887883364/889, 14185/969, 12609/11359517108746272468338071, \
///     3443/4354945, 1/29, 5551/892095, ...]"
/// )
/// ```
pub fn random_nonzero_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromSingleAndSign<RandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomRationalsFromSingleAndSign {
        bs: random_bools(seed.fork("sign")),
        xs: random_positive_naturals(seed.fork("abs"), mean_bits_numerator, mean_bits_denominator),
    }
}

/// Generates random non-negative [`Rational`]s, given an iterator of random [`Natural`] numerators,
/// an iterator of random [`Natural`] denominators, and an iterator of [`bool`] signs.
#[derive(Clone, Debug)]
pub struct RandomRationalsFromDoubleAndSign<
    I: Iterator<Item = Natural>,
    J: Iterator<Item = Natural>,
> {
    pub bs: RandomBools,
    pub xs: I,
    pub ys: J,
}

impl<I: Iterator<Item = Natural>, J: Iterator<Item = Natural>> Iterator
    for RandomRationalsFromDoubleAndSign<I, J>
{
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        Some(Rational::from_sign_and_naturals(
            self.bs.next().unwrap(),
            self.xs.next().unwrap(),
            self.ys.next().unwrap(),
        ))
    }
}

/// Generates random [`Rational`]s with a specified numerator and denominator mean bit length.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_bits_numerator /
/// mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero or if `mean_bits_numerator
/// <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::random_rationals;
///
/// assert_eq!(
///     prefix_to_string(random_rationals(EXAMPLE_SEED, 32, 1), 10),
///     "[-7301/34, -4183103/1234731190583, 54812347098686/6195807891591254727, \
///     812739/17841539017, -665/908, 677/1138982845180, 166/22491855393807861245619791028129, \
///     270142/5, 52040856788711439301087669967/15975369961878544862054, \
///     5718607/1953563256716085077, ...]"
/// )
/// ```
pub fn random_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromDoubleAndSign<
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
> {
    RandomRationalsFromDoubleAndSign {
        bs: random_bools(seed.fork("sign")),
        xs: random_naturals(
            seed.fork("numerator"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        ys: random_positive_naturals(
            seed.fork("denominator"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates striped random positive [`Rational`]s with a specified mean numerator and denominator
/// bit length.
///
/// The actual numerator and denominator bit lengths are chosen from a geometric distribution with
/// mean $m$, where $m$ is `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than
/// `1`. A striped bit sequence with the given stripe parameter is generated and truncated at the
/// bit lengths to produce the numerators and denominators. The highest bits are forced to be `1`.
/// Finally, the [`Rational`] is reduced.
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_bits_numerator /
/// mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if
/// `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::striped_random_positive_rationals;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_positive_rationals(EXAMPLE_SEED, 16, 1, 32, 1),
///         10
///     ),
///     "[4, 1/268681216, 75493376/9007199120523391, 8/8796094070783, \
///     8/950737950171027935941967741439, 1040391/33554432, \
///     2813000899879757964630563421437095845888, 1/79164837199872, 2199023255551/16, \
///     220784470296873664512/4611685966886694919, ...]"
/// )
/// ```
pub fn striped_random_positive_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromSingle<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomRationalsFromSingle {
        xs: striped_random_positive_naturals(
            seed,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates striped random non-positive [`Rational`]s with a specified mean numerator and
/// denominator bit length.
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_bits_numerator /
/// mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if
/// `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::striped_random_non_negative_rationals;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_non_negative_rationals(EXAMPLE_SEED, 16, 1, 32, 1),
///         10
///     ),
///     "[8192/127, 16776704/4396972769407, 8796093005951/648518346332962816, 87381/2863267840, \
///     1024/2043, 51/58408828928, 85/13521606402434254795714066382848, 270335/7, \
///     59421159664630116152453890047/9444741445172838006656, 6291455/1154891846623166464, ...]"
/// )
/// ```
pub fn striped_random_non_negative_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromDouble<
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
> {
    RandomRationalsFromDouble {
        xs: striped_random_naturals(
            seed.fork("numerator"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        ys: striped_random_positive_naturals(
            seed.fork("denominator"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates striped random negative [`Rational`]s with a specified mean numerator and denominator
/// bit length.
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_bits_numerator /
/// mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if
/// `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::striped_random_negative_rationals;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_negative_rationals(EXAMPLE_SEED, 16, 1, 32, 1),
///         10
///     ),
///     "[-4, -1/268681216, -75493376/9007199120523391, -8/8796094070783, \
///     -8/950737950171027935941967741439, -1040391/33554432, \
///     -2813000899879757964630563421437095845888, -1/79164837199872, -2199023255551/16, \
///     -220784470296873664512/4611685966886694919, ...]"
/// )
/// ```
pub fn striped_random_negative_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> NegativeRationals<
    RandomRationalsFromSingle<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>>,
> {
    NegativeRationals {
        xs: striped_random_positive_rationals(
            seed,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates striped random nonzero [`Rational`]s with a specified mean numerator and denominator
/// bit length.
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_bits_numerator /
/// mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if
/// `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::striped_random_nonzero_rationals;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_nonzero_rationals(EXAMPLE_SEED, 16, 1, 32, 1),
///         10
///     ),
///     "[-68720000000/18006083452797439, -2545165805/29, 549754781664/1236950581247, \
///     1065353727/2047, -2147745791/513, 16128/575, 8192/17000482516899619632318463, \
///     18431/16778240, 1/31, 4096/526335, ...]"
/// )
/// ```
pub fn striped_random_nonzero_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromSingleAndSign<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomRationalsFromSingleAndSign {
        bs: random_bools(seed.fork("sign")),
        xs: striped_random_positive_naturals(
            seed.fork("abs"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates striped random [`Rational`]s with a specified mean numerator and denominator bit
/// length.
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_bits_numerator /
/// mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if
/// `mean_bits_numerator <= mean_bits_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::striped_random_rationals;
///
/// assert_eq!(
///     prefix_to_string(striped_random_rationals(EXAMPLE_SEED, 16, 1, 32, 1), 10),
///     "[-8192/127, -16776704/4396972769407, 8796093005951/648518346332962816, 87381/2863267840, \
///     -1024/2043, 51/58408828928, 85/13521606402434254795714066382848, 270335/7, \
///     59421159664630116152453890047/9444741445172838006656, 6291455/1154891846623166464, ...]"
/// )
/// ```
pub fn striped_random_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromDoubleAndSign<
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
> {
    RandomRationalsFromDoubleAndSign {
        bs: random_bools(seed.fork("sign")),
        xs: striped_random_naturals(
            seed.fork("numerator"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        ys: striped_random_positive_naturals(
            seed.fork("denominator"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates random [`Rational`]s greater than or equal to a lower bound $a$ and with a specific
/// denominator.
///
/// The mean bit length $m$ of the absolute values of the numerators of the generated values is
/// specified. $m$ is equal to `mean_numerator_bits_numerator / mean_numerator_bits_denominator`.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_numerator_bits_numerator /
/// mean_numerator_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if $a > 0$ and their ratio
/// is less than or equal to the bit length of $a$, or if they are too large and manipulating them
/// leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::Natural;
/// use malachite_q::random::random_rational_with_denominator_range_to_infinity;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_with_denominator_range_to_infinity(
///             EXAMPLE_SEED,
///             Natural::from(3u32),
///             Rational::from_signeds(-3i32, 2),
///             3,
///             1
///         ),
///         10
///     ),
///     "[1/3, 4/3, -2/3, 94/3, 4/3, -2/3, 1/3, 145/3, 7/3, 1/3, ...]"
/// )
/// ```
pub fn random_rational_with_denominator_range_to_infinity(
    seed: Seed,
    d: Natural,
    a: Rational,
    mean_numerator_bits_numerator: u64,
    mean_numerator_bits_denominator: u64,
) -> RationalsWithDenominator<RandomIntegerRangeToInfinity> {
    assert_ne!(d, 0u32);
    RationalsWithDenominator {
        numerators: random_integer_range_to_infinity(
            seed,
            Integer::rounding_from(a * Rational::from(&d), Ceiling).0,
            mean_numerator_bits_numerator,
            mean_numerator_bits_denominator,
        ),
        denominator: d,
    }
}

/// Generates random [`Rational`]s less than or equal to a lower bound $a$ and with a specific
/// denominator.
///
/// The mean bit length $m$ of the absolute values of the numerators of the generated values is
/// specified. $m$ is equal to `mean_numerator_bits_numerator / mean_numerator_bits_denominator`.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_numerator_bits_numerator /
/// mean_numerator_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if $a < 0$ and their ratio
/// is less than or equal to the bit length of $a$, or if they are too large and manipulating them
/// leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::Natural;
/// use malachite_q::random::random_rational_with_denominator_range_to_negative_infinity;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_with_denominator_range_to_negative_infinity(
///             EXAMPLE_SEED,
///             Natural::from(3u32),
///             Rational::from_unsigneds(3u32, 2),
///             3,
///             1
///         ),
///         10
///     ),
///     "[1/3, 4/3, -56/3, -2/3, 2/3, -1/3, -7/3, -11/3, -17/3, 4/3, ...]"
/// )
/// ```
pub fn random_rational_with_denominator_range_to_negative_infinity(
    seed: Seed,
    d: Natural,
    a: Rational,
    mean_numerator_bits_numerator: u64,
    mean_numerator_bits_denominator: u64,
) -> RationalsWithDenominator<RandomIntegerRangeToInfinity> {
    assert_ne!(d, 0u32);
    RationalsWithDenominator {
        numerators: random_integer_range_to_negative_infinity(
            seed,
            Integer::rounding_from(a * Rational::from(&d), Floor).0,
            mean_numerator_bits_numerator,
            mean_numerator_bits_denominator,
        ),
        denominator: d,
    }
}

/// Generates random [`Rational`]s in the half-open interval $[a, b)$ and with a specific
/// denominator.
///
/// In general, the [`Rational`]s are not generated uniformly. Instead, [`Rational`]s whose
/// numerators have smaller bit lengths are generated more frequently.
///
/// The distribution of generated values is parametrized by a number $m$, given by
/// `mean_numerator_bits_numerator / mean_numerator_bits_denominator`. It is not actually the mean
/// bit length of the numerators, though it approaches the mean bit length of the numerators minus
/// $\lceil ad \right$ as $\log (b/a)$ approaches infinity. $m$ cannot be 0, and must be greater
/// than the bit length of the numerator of the generated [`Rational`] with the smallest absolute
/// value, but it may be arbitrarily large. The smaller it is, the more quickly the probabilities
/// decrease as bit length increases. The larger it is, the more closely the distribution approaches
/// a uniform distribution over the bit lengths.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_numerator_bits_numerator /
/// mean_numerator_bits_denominator`.
///
/// # Panics
/// Panics if $a \geq b$, if `mean_numerator_bits_numerator` or `mean_numerator_bits_denominator`
/// are zero, if their ratio is less than or equal to the bit length of the [`Rational`] with
/// smallest absolute numerator in the range, or if they are too large and manipulating them leads
/// to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::Natural;
/// use malachite_q::random::random_rational_with_denominator_range;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_with_denominator_range(
///             EXAMPLE_SEED,
///             Natural::from(100u32),
///             Rational::from_unsigneds(1u32, 3),
///             Rational::from_unsigneds(1u32, 2),
///             3,
///             1
///         ),
///         10
///     ),
///     "[41/100, 43/100, 41/100, 41/100, 39/100, 41/100, 49/100, 41/100, 41/100, 39/100, ...]"
/// )
/// ```
pub fn random_rational_with_denominator_range(
    seed: Seed,
    d: Natural,
    a: Rational,
    b: Rational,
    mut mean_numerator_bits_numerator: u64,
    mut mean_numerator_bits_denominator: u64,
) -> RationalsWithDenominator<RandomIntegerRange> {
    assert_ne!(d, 0u32);
    assert!(a < b);
    let q_d = Rational::from(&d);
    let a_i = Integer::rounding_from(a * &q_d, Ceiling).0;
    let upper_included = b.denominator_ref() == &d;
    let mut b_i = Integer::rounding_from(b * q_d, Floor).0;
    if !upper_included {
        b_i += Integer::ONE;
    }
    if (a_i >= 0) == (b_i >= 0) {
        let (n, d) = (Rational::from_unsigneds(
            mean_numerator_bits_numerator,
            mean_numerator_bits_denominator,
        ) + Rational::from(min(a_i.significant_bits(), b_i.significant_bits())))
        .into_numerator_and_denominator();
        mean_numerator_bits_numerator = u64::exact_from(&n);
        mean_numerator_bits_denominator = u64::exact_from(&d);
    }
    RationalsWithDenominator {
        numerators: random_integer_range(
            seed,
            a_i,
            b_i,
            mean_numerator_bits_numerator,
            mean_numerator_bits_denominator,
        ),
        denominator: d,
    }
}

/// Generates random [`Rational`]s in the closed interval $[a, b]$ and with a specific denominator.
///
/// In general, the [`Rational`]s are not generated uniformly. Instead, [`Rational`]s whose
/// numerators have smaller bit lengths are generated more frequently.
///
/// The distribution of generated values is parametrized by a number $m$, given by
/// `mean_numerator_bits_numerator / mean_numerator_bits_denominator`. It is not actually the mean
/// bit length of the numerators, though it approaches the mean bit length of the numerators minus
/// $\lceil ad \right$ as $\log (b/a)$ approaches infinity. $m$ cannot be 0, and must be greater
/// than the bit length of the numerator of the generated [`Rational`] with the smallest absolute
/// value, but it may be arbitrarily large. The smaller it is, the more quickly the probabilities
/// decrease as bit length increases. The larger it is, the more closely the distribution approaches
/// a uniform distribution over the bit lengths.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_numerator_bits_numerator /
/// mean_numerator_bits_denominator`.
///
/// # Panics
/// Panics if $a > b$, if `mean_numerator_bits_numerator` or `mean_numerator_bits_denominator` are
/// zero, if their ratio is less than or equal to the bit length of the [`Rational`] with smallest
/// absolute numerator in the range, or if they are too large and manipulating them leads to
/// arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::Natural;
/// use malachite_q::random::random_rational_with_denominator_inclusive_range;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_with_denominator_inclusive_range(
///             EXAMPLE_SEED,
///             Natural::from(100u32),
///             Rational::from_unsigneds(1u32, 3),
///             Rational::from_unsigneds(1u32, 2),
///             3,
///             1
///         ),
///         10
///     ),
///     "[41/100, 43/100, 41/100, 41/100, 39/100, 41/100, 49/100, 41/100, 41/100, 39/100, ...]"
/// )
/// ```
pub fn random_rational_with_denominator_inclusive_range(
    seed: Seed,
    d: Natural,
    a: Rational,
    b: Rational,
    mut mean_numerator_bits_numerator: u64,
    mut mean_numerator_bits_denominator: u64,
) -> RationalsWithDenominator<RandomIntegerRange> {
    assert_ne!(d, 0u32);
    assert!(a <= b);
    let q_d = Rational::from(&d);
    let a_i = Integer::rounding_from(a * &q_d, Ceiling).0;
    let b_i = Integer::rounding_from(b * q_d, Floor).0 + Integer::ONE;
    if (a_i >= 0) == (b_i >= 0) {
        let (n, d) = (Rational::from_unsigneds(
            mean_numerator_bits_numerator,
            mean_numerator_bits_denominator,
        ) + Rational::from(min(a_i.significant_bits(), b_i.significant_bits())))
        .into_numerator_and_denominator();
        mean_numerator_bits_numerator = u64::exact_from(&n);
        mean_numerator_bits_denominator = u64::exact_from(&d);
    }
    RationalsWithDenominator {
        numerators: random_integer_range(
            seed,
            a_i,
            b_i,
            mean_numerator_bits_numerator,
            mean_numerator_bits_denominator,
        ),
        denominator: d,
    }
}

/// Generates random [`Rational`]s greater than or equal to a lower bound. See
/// [`random_rational_range_to_infinity`] for more details.
#[derive(Clone, Debug)]
pub struct RandomRationalRangeToInfinity {
    a: Rational,
    limbs: RandomPrimitiveInts<u64>,
    range_generator: VariableRangeGenerator,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    denominators: RandomNaturals<GeometricRandomNaturalValues<u64>>,
}

impl Iterator for RandomRationalRangeToInfinity {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        let d = self.denominators.next().unwrap();
        let numerator_bound = Integer::rounding_from(&self.a * Rational::from(&d), Ceiling).0;
        let (numerator, denominator) = (Rational::from(d.significant_bits())
            + Rational::from_unsigneds(self.mean_bits_numerator, self.mean_bits_denominator))
        .into_numerator_and_denominator();
        let numerator = u64::exact_from(&numerator);
        let denominator = u64::exact_from(&denominator);
        loop {
            let n = get_random_integer_from_range_to_infinity(
                &mut self.limbs,
                &mut self.range_generator,
                numerator_bound.clone(),
                numerator,
                denominator,
            );
            if n.unsigned_abs_ref().coprime_with(&d) {
                return Some(Rational {
                    sign: n >= 0,
                    numerator: n.unsigned_abs(),
                    denominator: d,
                });
            }
        }
    }
}

/// Generates random [`Rational`]s greater than or equal to a lower bound $a$.
///
/// The mean bit length $m$ of the absolute values of the numerators of the generated values is
/// specified. $m$ is equal to `mean_numerator_bits_numerator / mean_numerator_bits_denominator`.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_numerator_bits_numerator /
/// mean_numerator_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if $a > 0$ and their ratio
/// is less than or equal to the bit length of $a$, or if they are too large and manipulating them
/// leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::random_rational_range_to_infinity;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_range_to_infinity(EXAMPLE_SEED, Rational::from_signeds(-3i32, 2), 3, 1),
///         10
///     ),
///     "[2/3, 56, 1/2, -1/34, -15/23, -4/5, 1/2, 5, 195, -1, ...]"
/// )
/// ```
pub fn random_rational_range_to_infinity(
    seed: Seed,
    a: Rational,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalRangeToInfinity {
    RandomRationalRangeToInfinity {
        a,
        limbs: random_primitive_ints(seed.fork("limbs")),
        range_generator: VariableRangeGenerator::new(seed.fork("range generator")),
        mean_bits_numerator,
        mean_bits_denominator,
        denominators: random_positive_naturals(
            seed.fork("denominators"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates random [`Rational`]s less than or equal to a lower bound. See
/// [`random_rational_range_to_negative_infinity`] for more details.
#[derive(Clone, Debug)]
pub struct RandomRationalRangeToNegativeInfinity {
    a: Rational,
    limbs: RandomPrimitiveInts<u64>,
    range_generator: VariableRangeGenerator,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    denominators: RandomNaturals<GeometricRandomNaturalValues<u64>>,
}

impl Iterator for RandomRationalRangeToNegativeInfinity {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        let d = self.denominators.next().unwrap();
        let numerator_bound = Integer::rounding_from(&self.a * Rational::from(&d), Floor).0;
        let (numerator, denominator) = (Rational::from(d.significant_bits())
            + Rational::from_unsigneds(self.mean_bits_numerator, self.mean_bits_denominator))
        .into_numerator_and_denominator();
        let numerator = u64::exact_from(&numerator);
        let denominator = u64::exact_from(&denominator);
        loop {
            let n = get_random_integer_from_range_to_negative_infinity(
                &mut self.limbs,
                &mut self.range_generator,
                numerator_bound.clone(),
                numerator,
                denominator,
            );
            if n.unsigned_abs_ref().coprime_with(&d) {
                return Some(Rational {
                    sign: n >= 0,
                    numerator: n.unsigned_abs(),
                    denominator: d,
                });
            }
        }
    }
}

/// Generates random [`Rational`]s less than or equal to a lower bound $a$.
///
/// The mean bit length $m$ of the absolute values of the numerators of the generated values is
/// specified. $m$ is equal to `mean_bits_numerator / mean_bits_denominator`.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_bits_numerator /
/// mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if $a > 0$ and their ratio
/// is less than or equal to the bit length of $a$, or if they are too large and manipulating them
/// leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::random_rational_range_to_negative_infinity;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_range_to_negative_infinity(
///             EXAMPLE_SEED,
///             Rational::from_signeds(-3i32, 2),
///             3,
///             1
///         ),
///         10
///     ),
///     "[-8/3, -114, -31/2, -1187/34, -61/23, -81/5, -3/2, -19, -82, -312, ...]"
/// )
/// ```
pub fn random_rational_range_to_negative_infinity(
    seed: Seed,
    a: Rational,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalRangeToNegativeInfinity {
    RandomRationalRangeToNegativeInfinity {
        a,
        limbs: random_primitive_ints(seed.fork("limbs")),
        range_generator: VariableRangeGenerator::new(seed.fork("range generator")),
        mean_bits_numerator,
        mean_bits_denominator,
        denominators: random_positive_naturals(
            seed.fork("denominators"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates random [`Rational`]s in a half-open interval $[a,b)$. See [`random_rational_range`]
/// for more details.
#[derive(Clone, Debug)]
pub struct RandomRationalRange {
    seed: Seed,
    mean_numerator_bits_numerator: u64,
    mean_numerator_bits_denominator: u64,
    a: Rational,
    b: Rational,
    denominators: IteratorCache<DenominatorsInClosedRationalInterval>,
    denominator_map: HashMap<Natural, RationalsWithDenominator<RandomIntegerRange>>,
    indices: GeometricRandomNaturalValues<usize>,
}

impl Iterator for RandomRationalRange {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        loop {
            let d = self.denominators.get(self.indices.next().unwrap()).unwrap();
            if (&self.a)
                .round_to_multiple(Rational::from(d).reciprocal(), Ceiling)
                .0
                >= self.b
            {
                // This can happen when d is the denominator of b
                continue;
            }
            return self
                .denominator_map
                .entry(d.clone())
                .or_insert_with(|| {
                    random_rational_with_denominator_range(
                        self.seed.fork(&d.to_string()),
                        d.clone(),
                        self.a.clone(),
                        self.b.clone(),
                        self.mean_numerator_bits_numerator,
                        self.mean_numerator_bits_denominator,
                    )
                })
                .next();
        }
    }
}

/// Generates random [`Rational`]s in the half-open interval $[a, b)$.
///
/// In general, the [`Rational`]s are not generated uniformly. Instead, [`Rational`]s whose
/// numerators have smaller bit lengths are generated more frequently.
///
/// The distribution of generated values is parametrized by a number $m$, given by
/// `mean_numerator_bits_numerator / mean_numerator_bits_denominator`. It is not actually the mean
/// bit length of the numerators, though it approaches the mean bit length of the numerators minus
/// $\lceil ad \right$ as $\log (b/a)$ approaches infinity. $m$ cannot be 0, and must be greater
/// than the bit length of the numerator of the generated [`Rational`] with the smallest absolute
/// value, but it may be arbitrarily large. The smaller it is, the more quickly the probabilities
/// decrease as bit length increases. The larger it is, the more closely the distribution approaches
/// a uniform distribution over the bit lengths.
///
/// The distribution of denominators is parametrized by `mean_denominator_index_numerator /
/// mean_denominator_index_denominator.` The larger this value is, the larger the average
/// denominator produced.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_numerator_bits_numerator /
/// mean_numerator_bits_denominator`.
///
/// # Panics
/// Panics if $a \geq b$, if `mean_numerator_bits_numerator`, `mean_numerator_bits_denominator`,
/// `mean_denominator_index_numerator`, or `mean_denominator_index_denominator` are zero, if
/// `mean_numerator_bits_numerator / mean_numerator_bits_denominator` is less than or equal to the
/// bit length of the [`Rational`] with smallest absolute numerator in the range, or if any of these
/// values are too are too large and manipulating them leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::random_rational_range;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_range(
///             EXAMPLE_SEED,
///             Rational::from_unsigneds(1u32, 3),
///             Rational::from_unsigneds(1u32, 2),
///             3,
///             1,
///             10,
///             1
///         ),
///         10
///     ),
///     "[1/3, 4/9, 7/19, 5/13, 13/28, 4/9, 5/14, 7/19, 14/33, 8/17, ...]"
/// )
/// ```
pub fn random_rational_range(
    seed: Seed,
    a: Rational,
    b: Rational,
    mean_numerator_bits_numerator: u64,
    mean_numerator_bits_denominator: u64,
    mean_denominator_index_numerator: u64,
    mean_denominator_index_denominator: u64,
) -> RandomRationalRange {
    assert!(a < b);
    assert_ne!(mean_numerator_bits_denominator, 0);
    assert_ne!(mean_denominator_index_denominator, 0);
    RandomRationalRange {
        seed: seed.fork("numerators"),
        mean_numerator_bits_numerator,
        mean_numerator_bits_denominator,
        a: a.clone(),
        b: b.clone(),
        denominators: IteratorCache::new(Rational::denominators_in_closed_interval(a, b)),
        denominator_map: HashMap::new(),
        indices: geometric_random_unsigneds(
            seed.fork("indices"),
            mean_denominator_index_numerator,
            mean_denominator_index_denominator,
        ),
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct RandomRationalInclusiveRangeInner {
    seed: Seed,
    mean_numerator_bits_numerator: u64,
    mean_numerator_bits_denominator: u64,
    a: Rational,
    b: Rational,
    denominators: IteratorCache<DenominatorsInClosedRationalInterval>,
    denominator_map: HashMap<Natural, RationalsWithDenominator<RandomIntegerRange>>,
    indices: GeometricRandomNaturalValues<usize>,
}

impl Iterator for RandomRationalInclusiveRangeInner {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        let d = self.denominators.get(self.indices.next().unwrap()).unwrap();
        self.denominator_map
            .entry(d.clone())
            .or_insert_with(|| {
                random_rational_with_denominator_inclusive_range(
                    self.seed.fork(&d.to_string()),
                    d.clone(),
                    self.a.clone(),
                    self.b.clone(),
                    self.mean_numerator_bits_numerator,
                    self.mean_numerator_bits_denominator,
                )
            })
            .next()
    }
}

/// Generates random [`Rational`]s in a closed interval $\[a,b\]$. See
/// [`random_rational_inclusive_range`] for more details.
#[derive(Clone, Debug)]
pub enum RandomRationalInclusiveRange {
    Single(Rational),
    Multiple(RandomRationalInclusiveRangeInner),
}

impl Iterator for RandomRationalInclusiveRange {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        match self {
            RandomRationalInclusiveRange::Single(x) => Some(x.clone()),
            RandomRationalInclusiveRange::Multiple(xs) => xs.next(),
        }
    }
}

/// Generates random [`Rational`]s in the closed interval $[a, b]$.
///
/// In general, the [`Rational`]s are not generated uniformly. Instead, [`Rational`]s whose
/// numerators have smaller bit lengths are generated more frequently.
///
/// The distribution of generated values is parametrized by a number $m$, given by
/// `mean_numerator_bits_numerator / mean_numerator_bits_denominator`. It is not actually the mean
/// bit length of the numerators, though it approaches the mean bit length of the numerators minus
/// $\lceil ad \right$ as $\log (b/a)$ approaches infinity. $m$ cannot be 0, and must be greater
/// than the bit length of the numerator of the generated [`Rational`] with the smallest absolute
/// value, but it may be arbitrarily large. The smaller it is, the more quickly the probabilities
/// decrease as bit length increases. The larger it is, the more closely the distribution approaches
/// a uniform distribution over the bit lengths.
///
/// The distribution of denominators is parametrized by `mean_denominator_index_numerator /
/// mean_denominator_index_denominator.` The larger this value is, the larger the average
/// denominator produced.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_numerator_bits_numerator /
/// mean_numerator_bits_denominator`.
///
/// # Panics
/// Panics if $a>b$, if `mean_numerator_bits_numerator`, `mean_numerator_bits_denominator`,
/// `mean_denominator_index_numerator`, or `mean_denominator_index_denominator` are zero, if
/// `mean_numerator_bits_numerator / mean_numerator_bits_denominator` is less than or equal to the
/// bit length of the [`Rational`] with smallest absolute numerator in the range, or if any of these
/// values are too are too large and manipulating them leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::random_rational_inclusive_range;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_inclusive_range(
///             EXAMPLE_SEED,
///             Rational::from_unsigneds(1u32, 3),
///             Rational::from_unsigneds(1u32, 2),
///             3,
///             1,
///             10,
///             1
///         ),
///         10
///     ),
///     "[1/3, 4/9, 1/2, 7/19, 5/13, 13/28, 1/2, 4/9, 1/2, 5/14, ...]"
/// )
/// ```
pub fn random_rational_inclusive_range(
    seed: Seed,
    a: Rational,
    b: Rational,
    mean_numerator_bits_numerator: u64,
    mean_numerator_bits_denominator: u64,
    mean_denominator_index_numerator: u64,
    mean_denominator_index_denominator: u64,
) -> RandomRationalInclusiveRange {
    assert!(a <= b);
    if a == b {
        return RandomRationalInclusiveRange::Single(a);
    }
    assert_ne!(mean_numerator_bits_denominator, 0);
    assert_ne!(mean_denominator_index_denominator, 0);
    RandomRationalInclusiveRange::Multiple(RandomRationalInclusiveRangeInner {
        seed: seed.fork("numerators"),
        mean_numerator_bits_numerator,
        mean_numerator_bits_denominator,
        a: a.clone(),
        b: b.clone(),
        denominators: IteratorCache::new(Rational::denominators_in_closed_interval(a, b)),
        denominator_map: HashMap::new(),
        indices: geometric_random_unsigneds(
            seed.fork("indices"),
            mean_denominator_index_numerator,
            mean_denominator_index_denominator,
        ),
    })
}

/// Generates striped random [`Rational`]s greater than or equal to a lower bound. See
/// [`striped_random_rational_range_to_infinity`] for more details.
#[derive(Clone, Debug)]
pub struct StripedRandomRationalRangeToInfinity {
    a: Rational,
    xs: StripedBitSource,
    range_generator: VariableRangeGenerator,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    denominators: StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
}

impl Iterator for StripedRandomRationalRangeToInfinity {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        let d = self.denominators.next().unwrap();
        let numerator_bound = Integer::rounding_from(&self.a * Rational::from(&d), Ceiling).0;
        let (numerator, denominator) = (Rational::from(d.significant_bits())
            + Rational::from_unsigneds(self.mean_bits_numerator, self.mean_bits_denominator))
        .into_numerator_and_denominator();
        let numerator = u64::exact_from(&numerator);
        let denominator = u64::exact_from(&denominator);
        loop {
            let n = get_striped_random_integer_from_range_to_infinity(
                &mut self.xs,
                &mut self.range_generator,
                numerator_bound.clone(),
                numerator,
                denominator,
            );
            if n.unsigned_abs_ref().coprime_with(&d) {
                return Some(Rational {
                    sign: n >= 0,
                    numerator: n.unsigned_abs(),
                    denominator: d,
                });
            }
        }
    }
}

/// Generates striped random [`Rational`]s greater than or equal to a lower bound $a$.
///
/// The actual numerator and denominator bit lengths are chosen from a geometric distribution with
/// mean $m$, where $m$ is `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than
/// `1`. A striped bit sequence with the given stripe parameter is generated and truncated at the
/// bit lengths to produce the numerators and denominators. The highest bits are forced to be `1`.
/// Finally, the [`Rational`] is reduced.
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_numerator_bits_numerator /
/// mean_numerator_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, if $a >
/// 0$ and their ratio is less than or equal to the bit length of $a$, or if they are too large and
/// manipulating them leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::striped_random_rational_range_to_infinity;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_rational_range_to_infinity(
///             EXAMPLE_SEED,
///             Rational::from_signeds(-3i32, 2),
///             10,
///             1,
///             3,
///             1
///         ),
///         10
///     ),
///     "[3/2, 39, 239/2, -32/63, 127/16, 16383/4, -1/2, 1, 583664, 1, ...]"
/// )
/// ```
pub fn striped_random_rational_range_to_infinity(
    seed: Seed,
    a: Rational,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomRationalRangeToInfinity {
    StripedRandomRationalRangeToInfinity {
        a,
        xs: StripedBitSource::new(
            seed.fork("xs"),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
        range_generator: VariableRangeGenerator::new(seed.fork("range generator")),
        mean_bits_numerator,
        mean_bits_denominator,
        denominators: striped_random_positive_naturals(
            seed.fork("denominators"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

/// Generates random striped [`Rational`]s less than or equal to a lower bound. See
/// [`striped_random_rational_range_to_negative_infinity`] for more details.
#[derive(Clone, Debug)]
pub struct StripedRandomRationalRangeToNegativeInfinity {
    a: Rational,
    xs: StripedBitSource,
    range_generator: VariableRangeGenerator,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    denominators: StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
}

impl Iterator for StripedRandomRationalRangeToNegativeInfinity {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        let d = self.denominators.next().unwrap();
        let numerator_bound = Integer::rounding_from(&self.a * Rational::from(&d), Floor).0;
        let (numerator, denominator) = (Rational::from(d.significant_bits())
            + Rational::from_unsigneds(self.mean_bits_numerator, self.mean_bits_denominator))
        .into_numerator_and_denominator();
        let numerator = u64::exact_from(&numerator);
        let denominator = u64::exact_from(&denominator);
        loop {
            let n = get_striped_random_integer_from_range_to_negative_infinity(
                &mut self.xs,
                &mut self.range_generator,
                numerator_bound.clone(),
                numerator,
                denominator,
            );
            if n.unsigned_abs_ref().coprime_with(&d) {
                return Some(Rational {
                    sign: n >= 0,
                    numerator: n.unsigned_abs(),
                    denominator: d,
                });
            }
        }
    }
}

/// Generates striped random [`Rational`]s less than or equal to a lower bound $a$.
///
/// The actual numerator and denominator bit lengths are chosen from a geometric distribution with
/// mean $m$, where $m$ is `mean_bits_numerator / mean_bits_denominator`; $m$ must be greater than
/// `1`. A striped bit sequence with the given stripe parameter is generated and truncated at the
/// bit lengths to produce the numerators and denominators. The highest bits are forced to be `1`.
/// Finally, the [`Rational`] is reduced.
///
/// The output length is infinite.
///
/// See [`StripedBitSource`] for information about generating striped random numbers.
///
/// # Expected complexity per iteration
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `mean_numerator_bits_numerator /
/// mean_numerator_bits_denominator`.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, if $a <
/// 0$ and their ratio is less than or equal to the bit length of $a$, or if they are too large and
/// manipulating them leads to arithmetic overflow.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::random::striped_random_rational_range_to_negative_infinity;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_rational_range_to_negative_infinity(
///             EXAMPLE_SEED,
///             Rational::from_signeds(-3i32, 2),
///             10,
///             1,
///             3,
///             1
///         ),
///         10
///     ),
///     "[-79/2, -7, -1051/2, -95/63, -255/16, -159/4, -3/2, -16, -2, -22, ...]"
/// )
/// ```
pub fn striped_random_rational_range_to_negative_infinity(
    seed: Seed,
    a: Rational,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomRationalRangeToNegativeInfinity {
    StripedRandomRationalRangeToNegativeInfinity {
        a,
        xs: StripedBitSource::new(
            seed.fork("xs"),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
        range_generator: VariableRangeGenerator::new(seed.fork("range generator")),
        mean_bits_numerator,
        mean_bits_denominator,
        denominators: striped_random_positive_naturals(
            seed.fork("denominators"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}
