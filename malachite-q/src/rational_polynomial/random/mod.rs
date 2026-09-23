// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational::random::{
    RandomRationalsFromDoubleAndSign, RandomRationalsFromSingleAndSign, random_nonzero_rationals,
    random_rationals, striped_random_nonzero_rationals, striped_random_rationals,
};
use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::random::RandomUnsignedInclusiveRange;
use malachite_base::num::random::geometric::GeometricRandomNaturalValues;
use malachite_base::polynomial::Polynomial;
use malachite_base::random::Seed;
use malachite_base::vecs::random::{
    RandomFixedLengthVecsWithLast, RandomVecsWithLast, random_vecs_with_last,
    random_vecs_with_last_fixed_length, random_vecs_with_last_length_inclusive_range,
    random_vecs_with_last_min_length,
};
use malachite_nz::natural::random::{RandomNaturals, StripedRandomNaturals};

/// Generates random [`RationalPolynomial`]s with coefficients from one iterator and leading
/// coefficients from another.
///
/// This `struct` is created by [`random_rational_polynomials_from_iterators`] and the generators
/// built on it; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomRationalPolynomials<
    I: Iterator<Item = u64>,
    J: Iterator<Item = Rational>,
    K: Iterator<Item = Rational>,
>(RandomVecsWithLast<Rational, I, J, K>);

impl<I: Iterator<Item = u64>, J: Iterator<Item = Rational>, K: Iterator<Item = Rational>> Iterator
    for RandomRationalPolynomials<I, J, K>
{
    type Item = RationalPolynomial;

    #[inline]
    fn next(&mut self) -> Option<RationalPolynomial> {
        self.0.next().map(RationalPolynomial::from_coefficients_asc)
    }
}

/// The type of the [`RationalPolynomial`] generators that draw their coefficients from every
/// [`Rational`] and their leading coefficients from every nonzero one, with lengths from a
/// geometric distribution.
pub type RandomRationalPolynomialsFromRationals = RandomRationalPolynomials<
    GeometricRandomNaturalValues<u64>,
    RandomPolynomialCoefficients,
    RandomPolynomialLeadingCoefficients,
>;

/// Generates random [`RationalPolynomial`]s whose coefficients come from one iterator and whose
/// leading coefficients come from another.
///
/// A polynomial is its coefficients, and the only thing that distinguishes them from any other list
/// of [`Rational`]s is that the last of them may not be zero. Singling out that one coefficient is
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
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::rational::random::{random_nonzero_rationals, random_rationals};
/// use malachite_q::rational_polynomial::random::random_rational_polynomials_from_iterators;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_polynomials_from_iterators(
///             EXAMPLE_SEED,
///             &|seed| random_rationals(seed, 4, 1),
///             &|seed| random_nonzero_rationals(seed, 4, 1),
///             1,
///             1,
///         ),
///         5
///     ),
///     "[-7, 157/5*x, -1/2*x+1, 53*x-1/55, -10, ...]"
/// );
/// ```
#[inline]
pub fn random_rational_polynomials_from_iterators<
    J: Iterator<Item = Rational>,
    K: Iterator<Item = Rational>,
>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> J,
    ys_gen: &dyn Fn(Seed) -> K,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomRationalPolynomials<GeometricRandomNaturalValues<u64>, J, K> {
    RandomRationalPolynomials(random_vecs_with_last(
        seed,
        xs_gen,
        ys_gen,
        mean_length_numerator,
        mean_length_denominator,
    ))
}

/// Generates random [`RationalPolynomial`]s.
///
/// The coefficients are sampled from [`random_rationals`] and the leading coefficient from
/// [`random_nonzero_rationals`], both with a mean bit count of `mean_bits_numerator /
/// mean_bits_denominator`.
///
/// The lengths — the number of coefficients, which is one more than the degree, or zero for the
/// zero polynomial — are sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`, so the zero polynomial is generated with the probability that that
/// distribution gives to 0.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell b)$
///
/// $M(i) = O(\ell b)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $\ell$ is the number
/// of coefficients of the $i$th output, and $b$ is `mean_bits_numerator / mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if their ratio is less than
/// or equal to 1, or if `mean_length_numerator` or `mean_length_denominator` are zero or their
/// ratio is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::rational_polynomial::random::random_rational_polynomials;
///
/// assert_eq!(
///     prefix_to_string(random_rational_polynomials(EXAMPLE_SEED, 4, 1, 1, 1), 5),
///     "[-7, 157/5*x, -1/2*x+1, 53*x-1/55, -10, ...]"
/// );
/// ```
#[inline]
pub fn random_rational_polynomials(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomRationalPolynomialsFromRationals {
    random_rational_polynomials_from_iterators(
        seed,
        &|seed_2| random_rationals(seed_2, mean_bits_numerator, mean_bits_denominator),
        &|seed_2| random_nonzero_rationals(seed_2, mean_bits_numerator, mean_bits_denominator),
        mean_length_numerator,
        mean_length_denominator,
    )
}

/// Generates random [`RationalPolynomial`]s of a given degree, with coefficients from one iterator
/// and leading coefficients from another.
///
/// This `struct` is created by [`random_rational_polynomials_with_degree`] and
/// [`striped_random_rational_polynomials_with_degree`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomRationalPolynomialsWithDegree<
    J: Iterator<Item = Rational>,
    K: Iterator<Item = Rational>,
>(RandomFixedLengthVecsWithLast<Rational, J, K>);

impl<J: Iterator<Item = Rational>, K: Iterator<Item = Rational>> Iterator
    for RandomRationalPolynomialsWithDegree<J, K>
{
    type Item = RationalPolynomial;

    #[inline]
    fn next(&mut self) -> Option<RationalPolynomial> {
        self.0.next().map(RationalPolynomial::from_coefficients_asc)
    }
}

/// The coefficients that the unstriped [`RationalPolynomial`] generators draw on.
pub type RandomPolynomialCoefficients = RandomRationalsFromDoubleAndSign<
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
>;

/// The leading coefficients that the unstriped [`RationalPolynomial`] generators draw on: a
/// polynomial's leading coefficient is never zero.
pub type RandomPolynomialLeadingCoefficients =
    RandomRationalsFromSingleAndSign<RandomNaturals<GeometricRandomNaturalValues<u64>>>;

/// The coefficients that the striped [`RationalPolynomial`] generators draw on.
pub type StripedRandomPolynomialCoefficients = RandomRationalsFromDoubleAndSign<
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
>;

/// The leading coefficients that the striped [`RationalPolynomial`] generators draw on.
pub type StripedRandomPolynomialLeadingCoefficients =
    RandomRationalsFromSingleAndSign<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>>;

/// The type of the [`RationalPolynomial`] generators whose degrees are uniform over a range.
pub type RandomRationalPolynomialsInDegreeRange = RandomRationalPolynomials<
    RandomUnsignedInclusiveRange<u64>,
    RandomPolynomialCoefficients,
    RandomPolynomialLeadingCoefficients,
>;

/// The type of the striped [`RationalPolynomial`] generators with geometrically distributed
/// lengths.
pub type StripedRandomRationalPolynomialsFromRationals = RandomRationalPolynomials<
    GeometricRandomNaturalValues<u64>,
    StripedRandomPolynomialCoefficients,
    StripedRandomPolynomialLeadingCoefficients,
>;

/// The type of the striped [`RationalPolynomial`] generators whose degrees are uniform over a
/// range.
pub type StripedRandomRationalPolynomialsInDegreeRange = RandomRationalPolynomials<
    RandomUnsignedInclusiveRange<u64>,
    StripedRandomPolynomialCoefficients,
    StripedRandomPolynomialLeadingCoefficients,
>;

/// Generates random [`RationalPolynomial`]s of a given degree.
///
/// A polynomial of degree $d$ has $d+1$ coefficients, of which the leading one is nonzero. The zero
/// polynomial is never generated: it has no degree at all, so no degree is the one it has.
///
/// The coefficients are sampled from [`random_rationals`] and the leading coefficient from
/// [`random_nonzero_rationals`], both with a mean bit count of `mean_bits_numerator /
/// mean_bits_denominator`.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(db)$
///
/// $M(i) = O(db)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $d$ is `degree`, and
/// $b$ is `mean_bits_numerator / mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if their ratio is less
/// than or equal to 1.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::rational_polynomial::random::random_rational_polynomials_with_degree;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_polynomials_with_degree(EXAMPLE_SEED, 2, 4, 1),
///         5
///     ),
///     "[-7*x^2+x, 157/5*x^2-1/55, -1/2*x^2+7/118*x-5/166, 53*x^2-1/6*x-3/194, \
///     -10*x^2-1/6*x+265/42, ...]"
/// );
/// ```
#[inline]
pub fn random_rational_polynomials_with_degree(
    seed: Seed,
    degree: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalPolynomialsWithDegree<
    RandomPolynomialCoefficients,
    RandomPolynomialLeadingCoefficients,
> {
    RandomRationalPolynomialsWithDegree(random_vecs_with_last_fixed_length(
        degree.saturating_add(1),
        random_rationals(seed.fork("xs"), mean_bits_numerator, mean_bits_denominator),
        random_nonzero_rationals(seed.fork("ys"), mean_bits_numerator, mean_bits_denominator),
    ))
}

/// Generates random [`RationalPolynomial`]s with a minimum degree.
///
/// The zero polynomial is never generated: it has no degree at all, so it is not of any degree at
/// least `min_degree`.
///
/// The coefficients are sampled from [`random_rationals`] and the leading coefficient from
/// [`random_nonzero_rationals`], both with a mean bit count of `mean_bits_numerator /
/// mean_bits_denominator`. The lengths — the number of coefficients, which is one more than the
/// degree — are sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`, which must be greater than `min_degree + 1`.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell b)$
///
/// $M(i) = O(\ell b)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $\ell$ is the number
/// of coefficients of the $i$th output, and $b$ is `mean_bits_numerator / mean_bits_denominator`.
///
/// # Panics
/// Panics if `mean_bits_numerator` or `mean_bits_denominator` are zero, if their ratio is less than
/// or equal to 1, or if `mean_length_numerator / mean_length_denominator` is less than or equal to
/// `min_degree + 1`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::rational_polynomial::random::random_rational_polynomials_min_degree;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_polynomials_min_degree(EXAMPLE_SEED, 1, 4, 1, 3, 1),
///         5
///     ),
///     "[-7*x^2+x, 157/5*x^3-5/166*x^2-1/55, -1/2*x^3-1/6*x^2-3/194*x+7/118, \
///      53*x^3+x^2-1/6*x+265/42, -10*x^2, ...]"
/// );
/// ```
#[inline]
pub fn random_rational_polynomials_min_degree(
    seed: Seed,
    min_degree: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomRationalPolynomialsFromRationals {
    RandomRationalPolynomials(random_vecs_with_last_min_length(
        seed,
        min_degree.saturating_add(1),
        &|seed_2| random_rationals(seed_2, mean_bits_numerator, mean_bits_denominator),
        &|seed_2| random_nonzero_rationals(seed_2, mean_bits_numerator, mean_bits_denominator),
        mean_length_numerator,
        mean_length_denominator,
    ))
}

/// Generates random [`RationalPolynomial`]s with degrees in $[a, b)$.
///
/// The degrees are sampled from a uniform distribution on $[a, b)$. The zero polynomial is never
/// generated: it has no degree at all, so its degree is in no range.
///
/// The coefficients are sampled from [`random_rationals`] and the leading coefficient from
/// [`random_nonzero_rationals`], both with a mean bit count of `mean_bits_numerator /
/// mean_bits_denominator`.
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
/// Panics if $a \geq b$, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if their
/// ratio is less than or equal to 1.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::rational_polynomial::random::random_rational_polynomials_degree_range;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 4, 1),
///         5
///     ),
///     "[-7*x^2+x, 157/5*x-1/55, -1/2*x^2-5/166*x, 53*x^2-3/194*x+7/118, -10*x-1/6, ...]"
/// );
/// ```
#[inline]
pub fn random_rational_polynomials_degree_range(
    seed: Seed,
    a: u64,
    b: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalPolynomialsInDegreeRange {
    assert!(a < b, "the degree range [{a}, {b}) is empty");
    random_rational_polynomials_degree_inclusive_range(
        seed,
        a,
        b - 1,
        mean_bits_numerator,
        mean_bits_denominator,
    )
}

/// Generates random [`RationalPolynomial`]s with degrees in $[a, b]$.
///
/// The degrees are sampled from a uniform distribution on $[a, b]$. The zero polynomial is never
/// generated: it has no degree at all, so its degree is in no range.
///
/// The coefficients are sampled from [`random_rationals`] and the leading coefficient from
/// [`random_nonzero_rationals`], both with a mean bit count of `mean_bits_numerator /
/// mean_bits_denominator`.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(b^\prime b)$
///
/// $M(i) = O(b^\prime b)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $b^\prime$ is the
/// largest degree, and $b$ is `mean_bits_numerator / mean_bits_denominator`.
///
/// # Panics
/// Panics if $a > b$, if `mean_bits_numerator` or `mean_bits_denominator` are zero, or if their
/// ratio is less than or equal to 1.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_q::rational_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         random_rational_polynomials_degree_inclusive_range(EXAMPLE_SEED, 1, 2, 4, 1),
///         5
///     ),
///     "[-7*x^2+x, 157/5*x-1/55, -1/2*x^2-5/166*x, 53*x^2-3/194*x+7/118, -10*x-1/6, ...]"
/// );
/// ```
#[inline]
pub fn random_rational_polynomials_degree_inclusive_range(
    seed: Seed,
    a: u64,
    b: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalPolynomialsInDegreeRange {
    assert!(a <= b, "the degree range [{a}, {b}] is empty");
    RandomRationalPolynomials(random_vecs_with_last_length_inclusive_range(
        seed,
        a.saturating_add(1),
        b.saturating_add(1),
        &|seed_2| random_rationals(seed_2, mean_bits_numerator, mean_bits_denominator),
        &|seed_2| random_nonzero_rationals(seed_2, mean_bits_numerator, mean_bits_denominator),
    ))
}

/// Generates random [`RationalPolynomial`]s with striped coefficients.
///
/// The coefficients are sampled from [`striped_random_rationals`] and the leading coefficient from
/// [`striped_random_nonzero_rationals`], with a mean run length of `mean_stripe_numerator /
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
/// $T(i) = O(\ell b)$
///
/// $M(i) = O(\ell b)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $\ell$ is the number
/// of coefficients of the $i$th output, and $b$ is `mean_bits_numerator / mean_bits_denominator`.
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
/// use malachite_q::rational_polynomial::random::striped_random_rational_polynomials;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_rational_polynomials(EXAMPLE_SEED, 16, 1, 4, 1, 1, 1),
///         5
///     ),
///     "[-4, 85/2*x, -1/3*x+1, 63*x-3/127, -15, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_rational_polynomials(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StripedRandomRationalPolynomialsFromRationals {
    random_rational_polynomials_from_iterators(
        seed,
        &|seed_2| {
            striped_random_rationals(
                seed_2,
                mean_stripe_numerator,
                mean_stripe_denominator,
                mean_bits_numerator,
                mean_bits_denominator,
            )
        },
        &|seed_2| {
            striped_random_nonzero_rationals(
                seed_2,
                mean_stripe_numerator,
                mean_stripe_denominator,
                mean_bits_numerator,
                mean_bits_denominator,
            )
        },
        mean_length_numerator,
        mean_length_denominator,
    )
}

/// Generates random [`RationalPolynomial`]s of a given degree, with striped coefficients.
///
/// A polynomial of degree $d$ has $d+1$ coefficients, of which the leading one is nonzero. The zero
/// polynomial is never generated: it has no degree at all, so no degree is the one it has.
///
/// The coefficients are striped, as they are in [`striped_random_rational_polynomials`].
///
/// # Worst-case complexity per iteration
/// $T(i) = O(db)$
///
/// $M(i) = O(db)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $d$ is `degree`, and
/// $b$ is `mean_bits_numerator / mean_bits_denominator`.
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
/// use malachite_q::rational_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_rational_polynomials_with_degree(EXAMPLE_SEED, 2, 16, 1, 4, 1),
///         5
///     ),
///     "[-4*x^2+x, 85/2*x^2-3/127, -1/3*x^2+1/16*x-7/128, 63*x^2-1/4*x-5/341, \
///     -15*x^2-1/7*x+363/85, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_rational_polynomials_with_degree(
    seed: Seed,
    degree: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalPolynomialsWithDegree<
    StripedRandomPolynomialCoefficients,
    StripedRandomPolynomialLeadingCoefficients,
> {
    RandomRationalPolynomialsWithDegree(random_vecs_with_last_fixed_length(
        degree.saturating_add(1),
        striped_random_rationals(
            seed.fork("xs"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        striped_random_nonzero_rationals(
            seed.fork("ys"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    ))
}

/// Generates random [`RationalPolynomial`]s with a minimum degree and striped coefficients.
///
/// The zero polynomial is never generated: it has no degree at all, so it is not of any degree at
/// least `min_degree`.
///
/// The coefficients are striped, as they are in [`striped_random_rational_polynomials`]. The
/// lengths — one more than the degree — are sampled from a geometric distribution with mean
/// `mean_length_numerator / mean_length_denominator`, which must be greater than `min_degree + 1`.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell b)$
///
/// $M(i) = O(\ell b)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $\ell$ is the number
/// of coefficients of the $i$th output, and $b$ is `mean_bits_numerator / mean_bits_denominator`.
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
/// use malachite_q::rational_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_rational_polynomials_min_degree(EXAMPLE_SEED, 1, 16, 1, 4, 1, 3, 1),
///         5
///     ),
///     "[-4*x^2+x, 85/2*x^3-7/128*x^2-3/127, -1/3*x^3-1/4*x^2-5/341*x+1/16, \
///      63*x^3+x^2-1/7*x+363/85, -15*x^2, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_rational_polynomials_min_degree(
    seed: Seed,
    min_degree: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StripedRandomRationalPolynomialsFromRationals {
    RandomRationalPolynomials(random_vecs_with_last_min_length(
        seed,
        min_degree.saturating_add(1),
        &|seed_2| {
            striped_random_rationals(
                seed_2,
                mean_stripe_numerator,
                mean_stripe_denominator,
                mean_bits_numerator,
                mean_bits_denominator,
            )
        },
        &|seed_2| {
            striped_random_nonzero_rationals(
                seed_2,
                mean_stripe_numerator,
                mean_stripe_denominator,
                mean_bits_numerator,
                mean_bits_denominator,
            )
        },
        mean_length_numerator,
        mean_length_denominator,
    ))
}

/// Generates random [`RationalPolynomial`]s with degrees in $[a, b)$ and striped coefficients.
///
/// The degrees are sampled from a uniform distribution on $[a, b)$. The zero polynomial is never
/// generated: it has no degree at all, so its degree is in no range.
///
/// The coefficients are striped, as they are in [`striped_random_rational_polynomials`].
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
/// use malachite_q::rational_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_rational_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 16, 1, 4, 1),
///         5
///     ),
///     "[-4*x^2+x, 85/2*x-3/127, -1/3*x^2-7/128*x, 63*x^2-5/341*x+1/16, -15*x-1/4, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_rational_polynomials_degree_range(
    seed: Seed,
    a: u64,
    b: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomRationalPolynomialsInDegreeRange {
    assert!(a < b, "the degree range [{a}, {b}) is empty");
    striped_random_rational_polynomials_degree_inclusive_range(
        seed,
        a,
        b - 1,
        mean_stripe_numerator,
        mean_stripe_denominator,
        mean_bits_numerator,
        mean_bits_denominator,
    )
}

/// Generates random [`RationalPolynomial`]s with degrees in $[a, b]$ and striped coefficients.
///
/// The degrees are sampled from a uniform distribution on $[a, b]$. The zero polynomial is never
/// generated: it has no degree at all, so its degree is in no range.
///
/// The coefficients are striped, as they are in [`striped_random_rational_polynomials`].
///
/// # Worst-case complexity per iteration
/// $T(i) = O(b^\prime b)$
///
/// $M(i) = O(b^\prime b)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $b^\prime$ is the
/// largest degree, and $b$ is `mean_bits_numerator / mean_bits_denominator`.
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
/// use malachite_q::rational_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_rational_polynomials_degree_inclusive_range(
///             EXAMPLE_SEED,
///             1,
///             2,
///             16,
///             1,
///             4,
///             1
///         ),
///         5
///     ),
///     "[-4*x^2+x, 85/2*x-3/127, -1/3*x^2-7/128*x, 63*x^2-5/341*x+1/16, -15*x-1/4, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_rational_polynomials_degree_inclusive_range(
    seed: Seed,
    a: u64,
    b: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomRationalPolynomialsInDegreeRange {
    assert!(a <= b, "the degree range [{a}, {b}] is empty");
    RandomRationalPolynomials(random_vecs_with_last_length_inclusive_range(
        seed,
        a.saturating_add(1),
        b.saturating_add(1),
        &|seed_2| {
            striped_random_rationals(
                seed_2,
                mean_stripe_numerator,
                mean_stripe_denominator,
                mean_bits_numerator,
                mean_bits_denominator,
            )
        },
        &|seed_2| {
            striped_random_nonzero_rationals(
                seed_2,
                mean_stripe_numerator,
                mean_stripe_denominator,
                mean_bits_numerator,
                mean_bits_denominator,
            )
        },
    ))
}
