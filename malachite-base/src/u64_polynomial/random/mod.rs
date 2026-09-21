// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::iterators::{NonzeroValues, nonzero_values};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::logic::traits::LowMask;
use crate::num::random::geometric::GeometricRandomNaturalValues;
use crate::num::random::striped::{
    StripedRandomUnsignedBitChunks, StripedRandomUnsignedInclusiveRange,
    striped_random_positive_unsigneds, striped_random_unsigned_bit_chunks,
    striped_random_unsigned_range, striped_random_unsigneds,
};
use crate::num::random::{
    RandomPrimitiveInts, RandomUnsignedInclusiveRange, RandomUnsignedRange,
    random_positive_unsigneds, random_primitive_ints, random_unsigned_inclusive_range,
    random_unsigned_range,
};
use crate::random::Seed;
use crate::u64_polynomial::U64Polynomial;
use crate::vecs::random::{
    RandomFixedLengthVecsWithLast, RandomVecsWithLast, random_vecs_with_last,
    random_vecs_with_last_fixed_length, random_vecs_with_last_length_inclusive_range,
    random_vecs_with_last_min_length,
};

/// Generates random [`U64Polynomial`]s with coefficients from one iterator and leading coefficients
/// from another.
///
/// This `struct` is created by [`random_u64_polynomials_from_iterators`] and the generators built
/// on it; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomU64Polynomials<
    I: Iterator<Item = u64>,
    J: Iterator<Item = u64>,
    K: Iterator<Item = u64>,
>(RandomVecsWithLast<u64, I, J, K>);

impl<I: Iterator<Item = u64>, J: Iterator<Item = u64>, K: Iterator<Item = u64>> Iterator
    for RandomU64Polynomials<I, J, K>
{
    type Item = U64Polynomial;

    #[inline]
    fn next(&mut self) -> Option<U64Polynomial> {
        self.0.next().map(U64Polynomial::from_coefficients_asc)
    }
}

/// The type of the [`U64Polynomial`] generators that draw their coefficients from every [`u64`] and
/// their leading coefficients from every positive one, with lengths from a geometric distribution.
pub type RandomU64PolynomialsFromU64s = RandomU64Polynomials<
    GeometricRandomNaturalValues<u64>,
    RandomPolynomialCoefficients,
    RandomPolynomialLeadingCoefficients,
>;

/// Generates random [`U64Polynomial`]s whose coefficients come from one iterator and whose leading
/// coefficients come from another.
///
/// A polynomial is its coefficients, and the only thing that distinguishes them from any other list
/// of [`u64`]s is that the last of them may not be zero. Singling out that one coefficient is
/// therefore all it takes: `xs_gen` supplies every coefficient below the leading one, and `ys_gen`
/// supplies the leading one.
///
/// `ys_gen` should produce no zeros, since a polynomial's leading coefficient is never zero. If it
/// does, the zeros are trimmed away, and the polynomial has a lower degree than its length
/// suggests.
///
/// The lengths of the polynomials — the number of coefficients, which is one more than the
/// degree, or zero for the zero polynomial — are sampled from a geometric distribution with a
/// specified mean $m$, equal to `mean_length_numerator / mean_length_denominator`. $m$ must be
/// greater than 0.
///
/// The iterators produced by `xs_gen` and `ys_gen` must be infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell T^\prime(i))$
///
/// $M(i) = O(\ell M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of the iterators produced by `xs_gen` and `ys_gen`,
/// and $\ell$ is the number of coefficients of the $i$th output.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, or if their ratio is
/// greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::random::{random_positive_unsigneds, random_primitive_ints};
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::random_u64_polynomials_from_iterators;
///
/// assert_eq!(
///     prefix_to_string(
///         random_u64_polynomials_from_iterators(
///             EXAMPLE_SEED,
///             &|seed| random_primitive_ints::<u64>(seed),
///             &|seed| random_positive_unsigneds::<u64>(seed),
///             1,
///             1,
///         ),
///         5
///     ),
///     "[6282517168718784610, 3854918945212287108*x+16126131237969988437, 3848495687584076941*x+16\
///     908237734149745446, 8242875068444962379*x+10938355129926736414, 33570146165392012, ...]"
/// );
/// ```
#[inline]
pub fn random_u64_polynomials_from_iterators<J: Iterator<Item = u64>, K: Iterator<Item = u64>>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> J,
    ys_gen: &dyn Fn(Seed) -> K,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomU64Polynomials<GeometricRandomNaturalValues<u64>, J, K> {
    RandomU64Polynomials(random_vecs_with_last(
        seed,
        xs_gen,
        ys_gen,
        mean_length_numerator,
        mean_length_denominator,
    ))
}

/// Generates random [`U64Polynomial`]s.
///
/// The coefficients are sampled from [`random_primitive_ints`] and the leading coefficient from
/// [`random_positive_unsigneds`], so each is uniform over its whole range — a [`u64`] has no mean
/// bit count to choose.
///
/// The lengths — the number of coefficients, which is one more than the degree, or zero for the
/// zero polynomial — are sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`, so the zero polynomial is generated with the probability that that
/// distribution gives to 0.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell)$
///
/// $M(i) = O(\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $\ell$ is the number
/// of coefficients of the $i$th output.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero or their ratio is
/// greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::random_u64_polynomials;
///
/// assert_eq!(
///     prefix_to_string(random_u64_polynomials(EXAMPLE_SEED, 1, 1), 5),
///     "[6282517168718784610, 3854918945212287108*x+16126131237969988437, 3848495687584076941*x+16\
///     908237734149745446, 8242875068444962379*x+10938355129926736414, 33570146165392012, ...]"
/// );
/// ```
#[inline]
pub fn random_u64_polynomials(
    seed: Seed,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomU64PolynomialsFromU64s {
    random_u64_polynomials_from_iterators(
        seed,
        &|seed_2| random_primitive_ints(seed_2),
        &|seed_2| random_positive_unsigneds(seed_2),
        mean_length_numerator,
        mean_length_denominator,
    )
}

/// Generates random [`U64Polynomial`]s of a given degree, with coefficients from one iterator and
/// leading coefficients from another.
///
/// This `struct` is created by [`random_u64_polynomials_with_degree`] and
/// [`striped_random_u64_polynomials_with_degree`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomU64PolynomialsWithDegree<J: Iterator<Item = u64>, K: Iterator<Item = u64>>(
    RandomFixedLengthVecsWithLast<u64, J, K>,
);

impl<J: Iterator<Item = u64>, K: Iterator<Item = u64>> Iterator
    for RandomU64PolynomialsWithDegree<J, K>
{
    type Item = U64Polynomial;

    #[inline]
    fn next(&mut self) -> Option<U64Polynomial> {
        self.0.next().map(U64Polynomial::from_coefficients_asc)
    }
}

/// The coefficients that the unstriped [`U64Polynomial`] generators draw on.
pub type RandomPolynomialCoefficients = RandomPrimitiveInts<u64>;

/// The leading coefficients that the unstriped [`U64Polynomial`] generators draw on: a polynomial's
/// leading coefficient is never zero.
pub type RandomPolynomialLeadingCoefficients = NonzeroValues<RandomPrimitiveInts<u64>>;

/// The coefficients that the striped [`U64Polynomial`] generators draw on.
pub type StripedRandomPolynomialCoefficients = StripedRandomUnsignedBitChunks<u64>;

/// The leading coefficients that the striped [`U64Polynomial`] generators draw on.
pub type StripedRandomPolynomialLeadingCoefficients =
    NonzeroValues<StripedRandomUnsignedBitChunks<u64>>;

/// The type of the [`U64Polynomial`] generator whose degree is fixed.
pub type RandomU64PolynomialsWithFixedDegree = RandomU64PolynomialsWithDegree<
    RandomPolynomialCoefficients,
    RandomPolynomialLeadingCoefficients,
>;

/// The type of the [`U64Polynomial`] generators whose degrees are uniform over a range.
pub type RandomU64PolynomialsInDegreeRange = RandomU64Polynomials<
    RandomUnsignedInclusiveRange<u64>,
    RandomPolynomialCoefficients,
    RandomPolynomialLeadingCoefficients,
>;

/// The type of the striped [`U64Polynomial`] generators with geometrically distributed lengths.
pub type StripedRandomU64PolynomialsFromU64s = RandomU64Polynomials<
    GeometricRandomNaturalValues<u64>,
    StripedRandomPolynomialCoefficients,
    StripedRandomPolynomialLeadingCoefficients,
>;

/// The type of the striped [`U64Polynomial`] generator whose degree is fixed.
pub type StripedRandomU64PolynomialsWithFixedDegree = RandomU64PolynomialsWithDegree<
    StripedRandomPolynomialCoefficients,
    StripedRandomPolynomialLeadingCoefficients,
>;

/// The type of the striped [`U64Polynomial`] generators whose degrees are uniform over a range.
pub type StripedRandomU64PolynomialsInDegreeRange = RandomU64Polynomials<
    RandomUnsignedInclusiveRange<u64>,
    StripedRandomPolynomialCoefficients,
    StripedRandomPolynomialLeadingCoefficients,
>;

/// Generates random [`U64Polynomial`]s of a given degree.
///
/// A polynomial of degree $d$ has $d+1$ coefficients, of which the leading one is positive. The
/// zero polynomial is never generated: it has no degree at all, so no degree is the one it has.
///
/// The coefficients are sampled from [`random_primitive_ints`] and the leading coefficient from
/// [`random_positive_unsigneds`], so each is uniform over its whole range — a [`u64`] has no mean
/// bit count to choose.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(d)$
///
/// $M(i) = O(d)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $d$ is `degree`, and
/// the coefficients are 64 bits each.
///
/// # Panics
/// Never panics.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::random_u64_polynomials_with_degree;
///
/// assert_eq!(
///     prefix_to_string(random_u64_polynomials_with_degree(EXAMPLE_SEED, 2), 5),
///     "[6282517168718784610*x^2+16908237734149745446*x+16126131237969988437, 3854918945212287108*\
///     x^2+12663883950309859797*x+10938355129926736414, 3848495687584076941*x^2+160309163093886283\
///     38*x+14328508029084493994, 8242875068444962379*x^2+6855165495190718789*x+527496784918977578\
///     9, 33570146165392012*x^2+5364743571823285937*x+12452306358869796714, ...]"
/// );
/// ```
#[inline]
pub fn random_u64_polynomials_with_degree(
    seed: Seed,
    degree: u64,
) -> RandomU64PolynomialsWithFixedDegree {
    RandomU64PolynomialsWithDegree(random_vecs_with_last_fixed_length(
        degree.saturating_add(1),
        random_primitive_ints(seed.fork("xs")),
        random_positive_unsigneds(seed.fork("ys")),
    ))
}

/// Generates random [`U64Polynomial`]s with a minimum degree.
///
/// The zero polynomial is never generated: it has no degree at all, so it is not of any degree at
/// least `min_degree`.
///
/// The coefficients are sampled from [`random_primitive_ints`] and the leading coefficient from
/// [`random_positive_unsigneds`], so each is uniform over its whole range — a [`u64`] has no mean
/// bit count to choose. The lengths — the number of coefficients, which is one more than the
/// degree — are sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`, which must be greater than `min_degree + 1`.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell)$
///
/// $M(i) = O(\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $\ell$ is the number
/// of coefficients of the $i$th output.
///
/// # Panics
/// Panics if `mean_length_numerator / mean_length_denominator` is less than or equal to `min_degree
/// + 1`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::random_u64_polynomials_min_degree;
///
/// assert_eq!(
///     prefix_to_string(random_u64_polynomials_min_degree(EXAMPLE_SEED, 1, 3, 1), 5),
///     "[6282517168718784610*x^2+16908237734149745446*x+16126131237969988437, 3854918945212287108*\
///     x^3+14328508029084493994*x^2+12663883950309859797*x+10938355129926736414, 38484956875840769\
///     41*x^3+6855165495190718789*x^2+5274967849189775789*x+16030916309388628338, 8242875068444962\
///     379*x^3+18084098515246349065*x^2+5364743571823285937*x+12452306358869796714, 33570146165392\
///     012*x^2+8082601913180739774*x+4929296619887363376, ...]"
/// );
/// ```
#[inline]
pub fn random_u64_polynomials_min_degree(
    seed: Seed,
    min_degree: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomU64PolynomialsFromU64s {
    RandomU64Polynomials(random_vecs_with_last_min_length(
        seed,
        min_degree.saturating_add(1),
        &|seed_2| random_primitive_ints(seed_2),
        &|seed_2| random_positive_unsigneds(seed_2),
        mean_length_numerator,
        mean_length_denominator,
    ))
}

/// Generates random [`U64Polynomial`]s with degrees in $[a, b)$.
///
/// The degrees are sampled from a uniform distribution on $[a, b)$. The zero polynomial is never
/// generated: it has no degree at all, so its degree is in no range.
///
/// The coefficients are sampled from [`random_primitive_ints`] and the leading coefficient from
/// [`random_positive_unsigneds`], so each is uniform over its whole range — a [`u64`] has no mean
/// bit count to choose.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(bd)$
///
/// $M(i) = O(bd)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $d$ is $b$, and $b$ is
/// `mean_bits_numerator / mean_bits_denominator`.
///
/// # Panics
/// Panics if $a \geq b$, nothing else.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::random_u64_polynomials_degree_range;
///
/// assert_eq!(
///     prefix_to_string(random_u64_polynomials_degree_range(EXAMPLE_SEED, 1, 3), 5),
///     "[6282517168718784610*x^2+16908237734149745446*x+16126131237969988437, 3854918945212287108*\
///     x+10938355129926736414, 3848495687584076941*x^2+14328508029084493994*x+12663883950309859797\
///     , 8242875068444962379*x^2+5274967849189775789*x+16030916309388628338, 33570146165392012*x+6\
///     855165495190718789, ...]"
/// );
/// ```
#[inline]
pub fn random_u64_polynomials_degree_range(
    seed: Seed,
    a: u64,
    b: u64,
) -> RandomU64PolynomialsInDegreeRange {
    assert!(a < b, "the degree range [{a}, {b}) is empty");
    random_u64_polynomials_degree_inclusive_range(seed, a, b - 1)
}

/// Generates random [`U64Polynomial`]s with degrees in $[a, b]$.
///
/// The degrees are sampled from a uniform distribution on $[a, b]$. The zero polynomial is never
/// generated: it has no degree at all, so its degree is in no range.
///
/// The coefficients are sampled from [`random_primitive_ints`] and the leading coefficient from
/// [`random_positive_unsigneds`], so each is uniform over its whole range — a [`u64`] has no mean
/// bit count to choose.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(b)$
///
/// $M(i) = O(b)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $b$ is the upper
/// bound on the degree.
///
/// # Panics
/// Panics if $a > b$, nothing else.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         random_u64_polynomials_degree_inclusive_range(EXAMPLE_SEED, 1, 2),
///         5
///     ),
///     "[6282517168718784610*x^2+16908237734149745446*x+16126131237969988437, 3854918945212287108*\
///     x+10938355129926736414, 3848495687584076941*x^2+14328508029084493994*x+12663883950309859797\
///     , 8242875068444962379*x^2+5274967849189775789*x+16030916309388628338, 33570146165392012*x+6\
///     855165495190718789, ...]"
/// );
/// ```
#[inline]
pub fn random_u64_polynomials_degree_inclusive_range(
    seed: Seed,
    a: u64,
    b: u64,
) -> RandomU64PolynomialsInDegreeRange {
    assert!(a <= b, "the degree range [{a}, {b}] is empty");
    RandomU64Polynomials(random_vecs_with_last_length_inclusive_range(
        seed,
        a.saturating_add(1),
        b.saturating_add(1),
        &|seed_2| random_primitive_ints(seed_2),
        &|seed_2| random_positive_unsigneds(seed_2),
    ))
}

/// Generates random [`U64Polynomial`]s with striped coefficients.
///
/// The coefficients are sampled from [`striped_random_unsigneds`] and the leading coefficient from
/// [`striped_random_positive_unsigneds`], with a mean run length of `mean_stripe_numerator /
/// mean_stripe_denominator` and a mean bit count of `mean_bits_numerator / mean_bits_denominator`.
/// A striped coefficient is one whose bits come in long runs, which is what makes the carries and
/// borrows of an arithmetic test interesting.
///
/// The lengths — the number of coefficients, which is one more than the degree, or zero for the
/// zero polynomial — are sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`, so the zero polynomial is generated with the probability that that
/// distribution gives to 0.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell)$
///
/// $M(i) = O(\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $\ell$ is the number
/// of coefficients of the $i$th output.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, if their
/// ratio is less than or equal to 1, or if `mean_length_numerator` or `mean_length_denominator` are
/// zero or their ratio is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::striped_random_u64_polynomials;
///
/// assert_eq!(
///     prefix_to_string(striped_random_u64_polynomials(EXAMPLE_SEED, 16, 1, 1, 1), 5),
///     "[271656550527, 27127151148662784*x+18302682203357708288, 8866461766451184*x+18446744005015\
///     272960, 18446181398633971712*x+9727775212300075008, 18446708889362628608, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_u64_polynomials(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StripedRandomU64PolynomialsFromU64s {
    random_u64_polynomials_from_iterators(
        seed,
        &|seed_2| striped_random_unsigneds(seed_2, mean_stripe_numerator, mean_stripe_denominator),
        &|seed_2| {
            striped_random_positive_unsigneds(
                seed_2,
                mean_stripe_numerator,
                mean_stripe_denominator,
            )
        },
        mean_length_numerator,
        mean_length_denominator,
    )
}

/// Generates random [`U64Polynomial`]s of a given degree, with striped coefficients.
///
/// A polynomial of degree $d$ has $d+1$ coefficients, of which the leading one is positive. The
/// zero polynomial is never generated: it has no degree at all, so no degree is the one it has.
///
/// The coefficients are striped, as they are in [`striped_random_u64_polynomials`].
///
/// # Worst-case complexity per iteration
/// $T(i) = O(d)$
///
/// $M(i) = O(d)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $d$ is `degree`, and
/// the coefficients are 64 bits each.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if
/// their ratio is less than or equal to 1.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_u64_polynomials_with_degree(EXAMPLE_SEED, 2, 16, 1),
///         5
///     ),
///     "[271656550527*x^2+18446744005015272960*x+18302682203357708288, 27127151148662784*x^2+22517\
///     99813816318*x+9727775212300075008, 8866461766451184*x^2+79164805742588*x+4398046510592, 184\
///     46181398633971712*x^2+13835058055549583359*x+31525223161659391, 18446708889362628608*x^2+90\
///     07199254740543*x+9223652962075148288, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_u64_polynomials_with_degree(
    seed: Seed,
    degree: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
) -> StripedRandomU64PolynomialsWithFixedDegree {
    RandomU64PolynomialsWithDegree(random_vecs_with_last_fixed_length(
        degree.saturating_add(1),
        striped_random_unsigneds(
            seed.fork("xs"),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
        striped_random_positive_unsigneds(
            seed.fork("ys"),
            mean_stripe_numerator,
            mean_stripe_denominator,
        ),
    ))
}

/// Generates random [`U64Polynomial`]s with a minimum degree and striped coefficients.
///
/// The zero polynomial is never generated: it has no degree at all, so it is not of any degree at
/// least `min_degree`.
///
/// The coefficients are striped, as they are in [`striped_random_u64_polynomials`]. The lengths —
/// one more than the degree — are sampled from a geometric distribution with mean
/// `mean_length_numerator / mean_length_denominator`, which must be greater than `min_degree + 1`.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell)$
///
/// $M(i) = O(\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $\ell$ is the number
/// of coefficients of the $i$th output.
///
/// # Panics
/// Panics if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, if their
/// ratio is less than or equal to 1, or if `mean_length_numerator / mean_length_denominator` is
/// less than or equal to `min_degree + 1`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_u64_polynomials_min_degree(EXAMPLE_SEED, 1, 16, 1, 3, 1),
///         5
///     ),
///     "[271656550527*x^2+18446744005015272960*x+18302682203357708288, 27127151148662784*x^3+43980\
///     46510592*x^2+2251799813816318*x+9727775212300075008, 8866461766451184*x^3+13835058055549583\
///     359*x^2+31525223161659391*x+79164805742588, 18446181398633971712*x^3+4398046446591*x^2+9007\
///     199254740543*x+9223652962075148288, 18446708889362628608*x^2+35047000244217*x, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_u64_polynomials_min_degree(
    seed: Seed,
    min_degree: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StripedRandomU64PolynomialsFromU64s {
    RandomU64Polynomials(random_vecs_with_last_min_length(
        seed,
        min_degree.saturating_add(1),
        &|seed_2| striped_random_unsigneds(seed_2, mean_stripe_numerator, mean_stripe_denominator),
        &|seed_2| {
            striped_random_positive_unsigneds(
                seed_2,
                mean_stripe_numerator,
                mean_stripe_denominator,
            )
        },
        mean_length_numerator,
        mean_length_denominator,
    ))
}

/// Generates random [`U64Polynomial`]s with degrees in $[a, b)$ and striped coefficients.
///
/// The degrees are sampled from a uniform distribution on $[a, b)$. The zero polynomial is never
/// generated: it has no degree at all, so its degree is in no range.
///
/// The coefficients are striped, as they are in [`striped_random_u64_polynomials`].
///
/// # Worst-case complexity per iteration
/// $T(i) = O(bd)$
///
/// $M(i) = O(bd)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $d$ is $b$, and $b$ is
/// `mean_bits_numerator / mean_bits_denominator`.
///
/// # Panics
/// Panics if $a \geq b$, if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if
/// their ratio is less than or equal to 1.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_u64_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 16, 1),
///         5
///     ),
///     "[271656550527*x^2+18446744005015272960*x+18302682203357708288, 27127151148662784*x+9727775\
///     212300075008, 8866461766451184*x^2+4398046510592*x+2251799813816318, 18446181398633971712*x\
///     ^2+31525223161659391*x+79164805742588, 18446708889362628608*x+13835058055549583359, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_u64_polynomials_degree_range(
    seed: Seed,
    a: u64,
    b: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
) -> StripedRandomU64PolynomialsInDegreeRange {
    assert!(a < b, "the degree range [{a}, {b}) is empty");
    striped_random_u64_polynomials_degree_inclusive_range(
        seed,
        a,
        b - 1,
        mean_stripe_numerator,
        mean_stripe_denominator,
    )
}

/// Generates random [`U64Polynomial`]s with degrees in $[a, b]$ and striped coefficients.
///
/// The degrees are sampled from a uniform distribution on $[a, b]$. The zero polynomial is never
/// generated: it has no degree at all, so its degree is in no range.
///
/// The coefficients are striped, as they are in [`striped_random_u64_polynomials`].
///
/// # Worst-case complexity per iteration
/// $T(i) = O(b)$
///
/// $M(i) = O(b)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $b$ is the upper
/// bound on the degree.
///
/// # Panics
/// Panics if $a > b$, if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if
/// their ratio is less than or equal to 1.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_u64_polynomials_degree_inclusive_range(EXAMPLE_SEED, 1, 2, 16, 1),
///         5
///     ),
///     "[271656550527*x^2+18446744005015272960*x+18302682203357708288, 27127151148662784*x+9727775\
///     212300075008, 8866461766451184*x^2+4398046510592*x+2251799813816318, 18446181398633971712*x\
///     ^2+31525223161659391*x+79164805742588, 18446708889362628608*x+13835058055549583359, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_u64_polynomials_degree_inclusive_range(
    seed: Seed,
    a: u64,
    b: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
) -> StripedRandomU64PolynomialsInDegreeRange {
    assert!(a <= b, "the degree range [{a}, {b}] is empty");
    RandomU64Polynomials(random_vecs_with_last_length_inclusive_range(
        seed,
        a.saturating_add(1),
        b.saturating_add(1),
        &|seed_2| striped_random_unsigneds(seed_2, mean_stripe_numerator, mean_stripe_denominator),
        &|seed_2| {
            striped_random_positive_unsigneds(
                seed_2,
                mean_stripe_numerator,
                mean_stripe_denominator,
            )
        },
    ))
}

/// The type of the [`U64Polynomial`] generator whose coefficients are reduced modulo a power of 2.
pub type RandomU64PolynomialsReducedModPowerOf2 = RandomU64Polynomials<
    GeometricRandomNaturalValues<u64>,
    RandomUnsignedInclusiveRange<u64>,
    RandomUnsignedInclusiveRange<u64>,
>;

/// Generates random [`U64Polynomial`]s that are reduced modulo $2^k$.
///
/// A polynomial is reduced modulo $2^k$ when every one of its coefficients is, so the coefficients
/// are sampled uniformly from $[0, 2^k)$ and the leading coefficient, which may not be zero, from
/// $[1, 2^k)$.
///
/// The lengths — the number of coefficients, which is one more than the degree, or zero for the
/// zero polynomial — are sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell)$
///
/// $M(i) = O(\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $\ell$ is the
/// number of coefficients of the $i$th output.
///
/// # Panics
/// Panics if `pow` is zero or greater than 64, if `mean_length_numerator` or
/// `mean_length_denominator` are zero, or if their ratio is greater than or equal to $2^{64}$. The
/// only polynomial reduced modulo $2^0$ is the zero polynomial, which leaves no leading coefficient
/// to choose, and no [`u64`] has more than 64 bits.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         random_u64_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 4, 2, 1),
///         5
///     ),
///     "[3*x^5+8*x^4+11*x^2+5*x+5, 7, \
///     9*x^7+8*x^6+6*x^5+14*x^4+11*x^3+12*x^2+8*x+8, 11, \
///     7*x^13+2*x^12+11*x^11+x^10+13*x^9+9*x^8+14*x^7+11*x^6+2*x^5+6*x^4+13*x^3+15*x^2+12*x+11, \
///     ...]"
/// );
/// ```
#[inline]
pub fn random_u64_polynomials_reduced_mod_power_of_2(
    seed: Seed,
    pow: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomU64PolynomialsReducedModPowerOf2 {
    assert_ne!(
        pow, 0,
        "the only polynomial reduced modulo 2^0 is the zero polynomial"
    );
    assert!(pow <= u64::WIDTH);
    let max = u64::low_mask(pow);
    random_u64_polynomials_from_iterators(
        seed,
        &|seed_2| random_unsigned_inclusive_range(seed_2, 0, max),
        &|seed_2| random_unsigned_inclusive_range(seed_2, 1, max),
        mean_length_numerator,
        mean_length_denominator,
    )
}

/// The type of the striped [`U64Polynomial`] generator whose coefficients are reduced modulo a
/// power of 2.
pub type StripedRandomU64PolynomialsReducedModPowerOf2 = RandomU64Polynomials<
    GeometricRandomNaturalValues<u64>,
    StripedRandomUnsignedBitChunks<u64>,
    NonzeroValues<StripedRandomUnsignedBitChunks<u64>>,
>;

/// Generates random [`U64Polynomial`]s that are reduced modulo $2^k$, with striped coefficients.
///
/// A coefficient is a striped bit chunk $k$ bits wide, which is exactly a value below $2^k$, so the
/// striping and the reduction are the same restriction rather than two: the bit chunk's width is
/// what makes the polynomial reduced. The leading coefficient, which may not be zero, is drawn from
/// the same source with the zeros filtered out.
///
/// The lengths — the number of coefficients, which is one more than the degree, or zero for the
/// zero polynomial — are sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell)$
///
/// $M(i) = O(\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $\ell$ is the
/// number of coefficients of the $i$th output.
///
/// # Panics
/// Panics if `pow` is zero or greater than 64, if `mean_stripe_denominator` is zero, if
/// `mean_stripe_numerator < mean_stripe_denominator`, if `mean_length_numerator` or
/// `mean_length_denominator` are zero, or if their ratio is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_u64_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 8, 8, 1, 2, 1),
///         5
///     ),
///     "[248*x^5+7*x^4+31*x^3+248*x^2+220*x+255, 224, \
///     111*x^7+112*x^4+255*x^3+255*x^2+x+3, 241, \
///     124*x^13+159*x^12+127*x^11+239*x^10+63*x^9+252*x^8+216*x^7+135*x^6+32*x^5+3*x^4+255*x^3+\
///     x^2, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_u64_polynomials_reduced_mod_power_of_2(
    seed: Seed,
    pow: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StripedRandomU64PolynomialsReducedModPowerOf2 {
    assert_ne!(
        pow, 0,
        "the only polynomial reduced modulo 2^0 is the zero polynomial"
    );
    assert!(pow <= u64::WIDTH);
    random_u64_polynomials_from_iterators(
        seed,
        &|seed_2| {
            striped_random_unsigned_bit_chunks(
                seed_2,
                pow,
                mean_stripe_numerator,
                mean_stripe_denominator,
            )
        },
        &|seed_2| {
            nonzero_values(striped_random_unsigned_bit_chunks(
                seed_2,
                pow,
                mean_stripe_numerator,
                mean_stripe_denominator,
            ))
        },
        mean_length_numerator,
        mean_length_denominator,
    )
}

/// The type of the [`U64Polynomial`] generator whose coefficients are reduced modulo a number.
pub type RandomU64PolynomialsReducedMod = RandomU64Polynomials<
    GeometricRandomNaturalValues<u64>,
    RandomUnsignedRange<u64>,
    RandomUnsignedRange<u64>,
>;

/// Generates random [`U64Polynomial`]s that are reduced modulo $m$.
///
/// A polynomial is reduced modulo $m$ when every one of its coefficients is, so the coefficients
/// are sampled uniformly from $[0, m)$ and the leading coefficient, which may not be zero, from
/// $[1, m)$.
///
/// The lengths — the number of coefficients, which is one more than the degree, or zero for the
/// zero polynomial — are sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`.
///
/// Where $m$ is a power of 2, [`random_u64_polynomials_reduced_mod_power_of_2`] generates from the
/// same set, though not the same sequence.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell)$
///
/// $M(i) = O(\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $\ell$ is the
/// number of coefficients of the $i$th output.
///
/// # Panics
/// Panics if `m` is less than 2, if `mean_length_numerator` or `mean_length_denominator` are zero,
/// or if their ratio is greater than or equal to $2^{64}$. Nothing is reduced modulo 0, and the
/// only polynomial reduced modulo 1 is the zero polynomial, which leaves no leading coefficient to
/// choose.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         random_u64_polynomials_reduced_mod(EXAMPLE_SEED, 10, 2, 1),
///         5
///     ),
///     "[3*x^5+8*x^4+8*x^3+5*x+5, 7, 9*x^7+x^6+9*x^5+2*x^4+6*x^3+8*x^2+6*x+8, 7, \
///     9*x^13+5*x^12+9*x^11+7*x^10+8*x^9+9*x^7+6*x^6+5*x^5+x^4+6*x^3+2*x^2+2, ...]"
/// );
/// ```
#[inline]
pub fn random_u64_polynomials_reduced_mod(
    seed: Seed,
    m: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomU64PolynomialsReducedMod {
    assert!(
        m >= 2,
        "nothing is reduced modulo 0, and only the zero polynomial is reduced modulo 1"
    );
    random_u64_polynomials_from_iterators(
        seed,
        &|seed_2| random_unsigned_range(seed_2, 0, m),
        &|seed_2| random_unsigned_range(seed_2, 1, m),
        mean_length_numerator,
        mean_length_denominator,
    )
}

/// The type of the striped [`U64Polynomial`] generator whose coefficients are reduced modulo a
/// number.
pub type StripedRandomU64PolynomialsReducedMod = RandomU64Polynomials<
    GeometricRandomNaturalValues<u64>,
    StripedRandomUnsignedInclusiveRange<u64>,
    StripedRandomUnsignedInclusiveRange<u64>,
>;

/// Generates random [`U64Polynomial`]s that are reduced modulo $m$, with striped coefficients.
///
/// The coefficients are striped values in $[0, m)$, and the leading coefficient, which may not be
/// zero, is a striped value in $[1, m)$.
///
/// Unlike [`striped_random_u64_polynomials_reduced_mod_power_of_2`], where a striped bit chunk of
/// the right width is already a reduced coefficient, an arbitrary $m$ is not a bit-width boundary,
/// so the striping and the reduction are two restrictions rather than one. A striped value in a
/// range keeps the long runs of equal bits that the range allows, so the coefficients just below
/// $m$ are the ones whose bit patterns are least free.
///
/// The lengths — the number of coefficients, which is one more than the degree, or zero for the
/// zero polynomial — are sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell)$
///
/// $M(i) = O(\ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, and $\ell$ is the
/// number of coefficients of the $i$th output.
///
/// # Panics
/// Panics if `m` is less than 2, if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_length_numerator` or `mean_length_denominator` are zero, or
/// if their ratio is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::u64_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_u64_polynomials_reduced_mod(EXAMPLE_SEED, 1000, 8, 1, 2, 1),
///         5
///     ),
///     "[x^5+3*x^4+x^3+62*x^2+952*x+999, 1, x^7+511*x^3+7*x^2+999*x+7, 775, \
///     992*x^13+959*x^12+3*x^11+481*x^10+512*x^9+636*x^8+992*x^7+33*x^6+799*x^5+639*x^4+542*x^3+\
///     479*x^2+992*x+127, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_u64_polynomials_reduced_mod(
    seed: Seed,
    m: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StripedRandomU64PolynomialsReducedMod {
    assert!(
        m >= 2,
        "nothing is reduced modulo 0, and only the zero polynomial is reduced modulo 1"
    );
    random_u64_polynomials_from_iterators(
        seed,
        &|seed_2| {
            striped_random_unsigned_range(
                seed_2,
                0,
                m,
                mean_stripe_numerator,
                mean_stripe_denominator,
            )
        },
        &|seed_2| {
            striped_random_unsigned_range(
                seed_2,
                1,
                m,
                mean_stripe_numerator,
                mean_stripe_denominator,
            )
        },
        mean_length_numerator,
        mean_length_denominator,
    )
}
