// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::random::{
    RandomNaturals, RandomNaturalsLessThan, RandomNaturalsWithUpToBits,
    StripedRandomNaturalInclusiveRange, StripedRandomNaturals, StripedRandomNaturalsWithUpToBits,
    random_naturals, random_naturals_less_than, random_naturals_less_than_power_of_2,
    random_positive_naturals, striped_random_natural_range, striped_random_naturals,
    striped_random_naturals_less_than_power_of_2, striped_random_positive_naturals,
};
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::iterators::{NonzeroValues, nonzero_values};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::random::RandomUnsignedInclusiveRange;
use malachite_base::num::random::geometric::GeometricRandomNaturalValues;
use malachite_base::polynomial::Polynomial;
use malachite_base::random::Seed;
use malachite_base::vecs::random::{
    RandomFixedLengthVecsWithLast, RandomVecsWithLast, random_vecs_with_last,
    random_vecs_with_last_fixed_length, random_vecs_with_last_length_inclusive_range,
    random_vecs_with_last_min_length,
};

/// Generates random [`NaturalPolynomial`]s with coefficients from one iterator and leading
/// coefficients from another.
///
/// This `struct` is created by [`random_natural_polynomials_from_iterators`] and the generators
/// built on it; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomNaturalPolynomials<
    I: Iterator<Item = u64>,
    J: Iterator<Item = Natural>,
    K: Iterator<Item = Natural>,
>(RandomVecsWithLast<Natural, I, J, K>);

impl<I: Iterator<Item = u64>, J: Iterator<Item = Natural>, K: Iterator<Item = Natural>> Iterator
    for RandomNaturalPolynomials<I, J, K>
{
    type Item = NaturalPolynomial;

    #[inline]
    fn next(&mut self) -> Option<NaturalPolynomial> {
        self.0.next().map(NaturalPolynomial::from_coefficients_asc)
    }
}

/// The type of the [`NaturalPolynomial`] generators that draw their coefficients from every
/// [`Natural`] and their leading coefficients from every positive one, with lengths from a
/// geometric distribution.
pub type RandomNaturalPolynomialsFromNaturals = RandomNaturalPolynomials<
    GeometricRandomNaturalValues<u64>,
    RandomPolynomialCoefficients,
    RandomPolynomialCoefficients,
>;

/// Generates random [`NaturalPolynomial`]s whose coefficients come from one iterator and whose
/// leading coefficients come from another.
///
/// A polynomial is its coefficients, and the only thing that distinguishes them from any other list
/// of [`Natural`]s is that the last of them may not be zero. Singling out that one coefficient is
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
/// use malachite_nz::natural::random::{random_naturals, random_positive_naturals};
/// use malachite_nz::natural_polynomial::random::random_natural_polynomials_from_iterators;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_polynomials_from_iterators(
///             EXAMPLE_SEED,
///             &|seed| random_naturals(seed, 4, 1),
///             &|seed| random_positive_naturals(seed, 4, 1),
///             1,
///             1,
///         ),
///         5
///     ),
///     "[14, x+3, 4*x, x+13235, 30, ...]"
/// );
/// ```
#[inline]
pub fn random_natural_polynomials_from_iterators<
    J: Iterator<Item = Natural>,
    K: Iterator<Item = Natural>,
>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> J,
    ys_gen: &dyn Fn(Seed) -> K,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomNaturalPolynomials<GeometricRandomNaturalValues<u64>, J, K> {
    RandomNaturalPolynomials(random_vecs_with_last(
        seed,
        xs_gen,
        ys_gen,
        mean_length_numerator,
        mean_length_denominator,
    ))
}

/// Generates random [`NaturalPolynomial`]s.
///
/// The coefficients are sampled from [`random_naturals`] and the leading coefficient from
/// [`random_positive_naturals`], both with a mean bit count of `mean_bits_numerator /
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
/// use malachite_nz::natural_polynomial::random::random_natural_polynomials;
///
/// assert_eq!(
///     prefix_to_string(random_natural_polynomials(EXAMPLE_SEED, 4, 1, 1, 1), 5),
///     "[14, x+3, 4*x, x+13235, 30, ...]"
/// );
/// ```
#[inline]
pub fn random_natural_polynomials(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomNaturalPolynomialsFromNaturals {
    random_natural_polynomials_from_iterators(
        seed,
        &|seed_2| random_naturals(seed_2, mean_bits_numerator, mean_bits_denominator),
        &|seed_2| random_positive_naturals(seed_2, mean_bits_numerator, mean_bits_denominator),
        mean_length_numerator,
        mean_length_denominator,
    )
}

/// Generates random [`NaturalPolynomial`]s of a given degree, with coefficients from one iterator
/// and leading coefficients from another.
///
/// This `struct` is created by [`random_natural_polynomials_with_degree`] and
/// [`striped_random_natural_polynomials_with_degree`]; see their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomNaturalPolynomialsWithDegree<
    J: Iterator<Item = Natural>,
    K: Iterator<Item = Natural>,
>(RandomFixedLengthVecsWithLast<Natural, J, K>);

impl<J: Iterator<Item = Natural>, K: Iterator<Item = Natural>> Iterator
    for RandomNaturalPolynomialsWithDegree<J, K>
{
    type Item = NaturalPolynomial;

    #[inline]
    fn next(&mut self) -> Option<NaturalPolynomial> {
        self.0.next().map(NaturalPolynomial::from_coefficients_asc)
    }
}

/// The coefficients that the unstriped [`NaturalPolynomial`] generators draw on.
pub type RandomPolynomialCoefficients = RandomNaturals<GeometricRandomNaturalValues<u64>>;

/// The coefficients that the striped [`NaturalPolynomial`] generators draw on.
pub type StripedRandomPolynomialCoefficients =
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>;

/// The type of the [`NaturalPolynomial`] generators whose degrees are uniform over a range.
pub type RandomNaturalPolynomialsInDegreeRange = RandomNaturalPolynomials<
    RandomUnsignedInclusiveRange<u64>,
    RandomPolynomialCoefficients,
    RandomPolynomialCoefficients,
>;

/// The type of the striped [`NaturalPolynomial`] generators with geometrically distributed lengths.
pub type StripedRandomNaturalPolynomialsFromNaturals = RandomNaturalPolynomials<
    GeometricRandomNaturalValues<u64>,
    StripedRandomPolynomialCoefficients,
    StripedRandomPolynomialCoefficients,
>;

/// The type of the striped [`NaturalPolynomial`] generators whose degrees are uniform over a range.
pub type StripedRandomNaturalPolynomialsInDegreeRange = RandomNaturalPolynomials<
    RandomUnsignedInclusiveRange<u64>,
    StripedRandomPolynomialCoefficients,
    StripedRandomPolynomialCoefficients,
>;

/// Generates random [`NaturalPolynomial`]s of a given degree.
///
/// A polynomial of degree $d$ has $d+1$ coefficients, of which the leading one is positive. The
/// zero polynomial is never generated: it has no degree at all, so no degree is the one it has.
///
/// The coefficients are sampled from [`random_naturals`] and the leading coefficient from
/// [`random_positive_naturals`], both with a mean bit count of `mean_bits_numerator /
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
/// use malachite_nz::natural_polynomial::random::random_natural_polynomials_with_degree;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_polynomials_with_degree(EXAMPLE_SEED, 2, 4, 1),
///         5
///     ),
///     "[14*x^2+3, x^2+x+13235, 4*x^2+x+7, x^2+15*x, 30*x^2+2*x+13, ...]"
/// );
/// ```
#[inline]
pub fn random_natural_polynomials_with_degree(
    seed: Seed,
    degree: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturalPolynomialsWithDegree<RandomPolynomialCoefficients, RandomPolynomialCoefficients>
{
    RandomNaturalPolynomialsWithDegree(random_vecs_with_last_fixed_length(
        degree.saturating_add(1),
        random_naturals(seed.fork("xs"), mean_bits_numerator, mean_bits_denominator),
        random_positive_naturals(seed.fork("ys"), mean_bits_numerator, mean_bits_denominator),
    ))
}

/// Generates random [`NaturalPolynomial`]s with a minimum degree.
///
/// The zero polynomial is never generated: it has no degree at all, so it is not of any degree at
/// least `min_degree`.
///
/// The coefficients are sampled from [`random_naturals`] and the leading coefficient from
/// [`random_positive_naturals`], both with a mean bit count of `mean_bits_numerator /
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
/// use malachite_nz::natural_polynomial::random::random_natural_polynomials_min_degree;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_polynomials_min_degree(EXAMPLE_SEED, 1, 4, 1, 3, 1),
///         5
///     ),
///     "[14*x^2+3, x^3+7*x^2+x+13235, 4*x^3+15*x^2+1, x^3+x^2+2*x+13, 30*x^2+x, ...]"
/// );
/// ```
#[inline]
pub fn random_natural_polynomials_min_degree(
    seed: Seed,
    min_degree: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomNaturalPolynomialsFromNaturals {
    RandomNaturalPolynomials(random_vecs_with_last_min_length(
        seed,
        min_degree.saturating_add(1),
        &|seed_2| random_naturals(seed_2, mean_bits_numerator, mean_bits_denominator),
        &|seed_2| random_positive_naturals(seed_2, mean_bits_numerator, mean_bits_denominator),
        mean_length_numerator,
        mean_length_denominator,
    ))
}

/// Generates random [`NaturalPolynomial`]s with degrees in $[a, b)$.
///
/// The degrees are sampled from a uniform distribution on $[a, b)$. The zero polynomial is never
/// generated: it has no degree at all, so its degree is in no range.
///
/// The coefficients are sampled from [`random_naturals`] and the leading coefficient from
/// [`random_positive_naturals`], both with a mean bit count of `mean_bits_numerator /
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
/// use malachite_nz::natural_polynomial::random::random_natural_polynomials_degree_range;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 4, 1),
///         5
///     ),
///     "[14*x^2+3, x+13235, 4*x^2+7*x+1, x^2+1, 30*x+15, ...]"
/// );
/// ```
#[inline]
pub fn random_natural_polynomials_degree_range(
    seed: Seed,
    a: u64,
    b: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturalPolynomialsInDegreeRange {
    assert!(a < b, "the degree range [{a}, {b}) is empty");
    random_natural_polynomials_degree_inclusive_range(
        seed,
        a,
        b - 1,
        mean_bits_numerator,
        mean_bits_denominator,
    )
}

/// Generates random [`NaturalPolynomial`]s with degrees in $[a, b]$.
///
/// The degrees are sampled from a uniform distribution on $[a, b]$. The zero polynomial is never
/// generated: it has no degree at all, so its degree is in no range.
///
/// The coefficients are sampled from [`random_naturals`] and the leading coefficient from
/// [`random_positive_naturals`], both with a mean bit count of `mean_bits_numerator /
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
/// use malachite_nz::natural_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_polynomials_degree_inclusive_range(EXAMPLE_SEED, 1, 2, 4, 1),
///         5
///     ),
///     "[14*x^2+3, x+13235, 4*x^2+7*x+1, x^2+1, 30*x+15, ...]"
/// );
/// ```
#[inline]
pub fn random_natural_polynomials_degree_inclusive_range(
    seed: Seed,
    a: u64,
    b: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturalPolynomialsInDegreeRange {
    assert!(a <= b, "the degree range [{a}, {b}] is empty");
    RandomNaturalPolynomials(random_vecs_with_last_length_inclusive_range(
        seed,
        a.saturating_add(1),
        b.saturating_add(1),
        &|seed_2| random_naturals(seed_2, mean_bits_numerator, mean_bits_denominator),
        &|seed_2| random_positive_naturals(seed_2, mean_bits_numerator, mean_bits_denominator),
    ))
}

/// Generates random [`NaturalPolynomial`]s with striped coefficients.
///
/// The coefficients are sampled from [`striped_random_naturals`] and the leading coefficient from
/// [`striped_random_positive_naturals`], with a mean run length of `mean_stripe_numerator /
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
/// use malachite_nz::natural_polynomial::random::striped_random_natural_polynomials;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_natural_polynomials(EXAMPLE_SEED, 16, 1, 4, 1, 1, 1),
///         5
///     ),
///     "[15, x+2, 6*x, x+16376, 30, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_natural_polynomials(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StripedRandomNaturalPolynomialsFromNaturals {
    random_natural_polynomials_from_iterators(
        seed,
        &|seed_2| {
            striped_random_naturals(
                seed_2,
                mean_stripe_numerator,
                mean_stripe_denominator,
                mean_bits_numerator,
                mean_bits_denominator,
            )
        },
        &|seed_2| {
            striped_random_positive_naturals(
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

/// Generates random [`NaturalPolynomial`]s of a given degree, with striped coefficients.
///
/// A polynomial of degree $d$ has $d+1$ coefficients, of which the leading one is positive. The
/// zero polynomial is never generated: it has no degree at all, so no degree is the one it has.
///
/// The coefficients are striped, as they are in [`striped_random_natural_polynomials`].
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
/// use malachite_nz::natural_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_natural_polynomials_with_degree(EXAMPLE_SEED, 2, 16, 1, 4, 1),
///         5
///     ),
///     "[15*x^2+2, x^2+x+16376, 6*x^2+x+4, x^2+15*x, 30*x^2+3*x+15, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_natural_polynomials_with_degree(
    seed: Seed,
    degree: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomNaturalPolynomialsWithDegree<
    StripedRandomPolynomialCoefficients,
    StripedRandomPolynomialCoefficients,
> {
    RandomNaturalPolynomialsWithDegree(random_vecs_with_last_fixed_length(
        degree.saturating_add(1),
        striped_random_naturals(
            seed.fork("xs"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        striped_random_positive_naturals(
            seed.fork("ys"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    ))
}

/// Generates random [`NaturalPolynomial`]s with a minimum degree and striped coefficients.
///
/// The zero polynomial is never generated: it has no degree at all, so it is not of any degree at
/// least `min_degree`.
///
/// The coefficients are striped, as they are in [`striped_random_natural_polynomials`]. The lengths
/// — one more than the degree — are sampled from a geometric distribution with mean
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
/// use malachite_nz::natural_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_natural_polynomials_min_degree(EXAMPLE_SEED, 1, 16, 1, 4, 1, 3, 1),
///         5
///     ),
///     "[15*x^2+2, x^3+4*x^2+x+16376, 6*x^3+15*x^2+1, x^3+x^2+3*x+15, 30*x^2+x, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_natural_polynomials_min_degree(
    seed: Seed,
    min_degree: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StripedRandomNaturalPolynomialsFromNaturals {
    RandomNaturalPolynomials(random_vecs_with_last_min_length(
        seed,
        min_degree.saturating_add(1),
        &|seed_2| {
            striped_random_naturals(
                seed_2,
                mean_stripe_numerator,
                mean_stripe_denominator,
                mean_bits_numerator,
                mean_bits_denominator,
            )
        },
        &|seed_2| {
            striped_random_positive_naturals(
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

/// Generates random [`NaturalPolynomial`]s with degrees in $[a, b)$ and striped coefficients.
///
/// The degrees are sampled from a uniform distribution on $[a, b)$. The zero polynomial is never
/// generated: it has no degree at all, so its degree is in no range.
///
/// The coefficients are striped, as they are in [`striped_random_natural_polynomials`].
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
/// use malachite_nz::natural_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_natural_polynomials_degree_range(EXAMPLE_SEED, 1, 3, 16, 1, 4, 1),
///         5
///     ),
///     "[15*x^2+2, x+16376, 6*x^2+4*x+1, x^2+1, 30*x+15, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_natural_polynomials_degree_range(
    seed: Seed,
    a: u64,
    b: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomNaturalPolynomialsInDegreeRange {
    assert!(a < b, "the degree range [{a}, {b}) is empty");
    striped_random_natural_polynomials_degree_inclusive_range(
        seed,
        a,
        b - 1,
        mean_stripe_numerator,
        mean_stripe_denominator,
        mean_bits_numerator,
        mean_bits_denominator,
    )
}

/// Generates random [`NaturalPolynomial`]s with degrees in $[a, b]$ and striped coefficients.
///
/// The degrees are sampled from a uniform distribution on $[a, b]$. The zero polynomial is never
/// generated: it has no degree at all, so its degree is in no range.
///
/// The coefficients are striped, as they are in [`striped_random_natural_polynomials`].
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
/// use malachite_nz::natural_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_natural_polynomials_degree_inclusive_range(
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
///     "[15*x^2+2, x+16376, 6*x^2+4*x+1, x^2+1, 30*x+15, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_natural_polynomials_degree_inclusive_range(
    seed: Seed,
    a: u64,
    b: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> StripedRandomNaturalPolynomialsInDegreeRange {
    assert!(a <= b, "the degree range [{a}, {b}] is empty");
    RandomNaturalPolynomials(random_vecs_with_last_length_inclusive_range(
        seed,
        a.saturating_add(1),
        b.saturating_add(1),
        &|seed_2| {
            striped_random_naturals(
                seed_2,
                mean_stripe_numerator,
                mean_stripe_denominator,
                mean_bits_numerator,
                mean_bits_denominator,
            )
        },
        &|seed_2| {
            striped_random_positive_naturals(
                seed_2,
                mean_stripe_numerator,
                mean_stripe_denominator,
                mean_bits_numerator,
                mean_bits_denominator,
            )
        },
    ))
}

/// The type of the [`NaturalPolynomial`] generator whose coefficients are reduced modulo a power of
/// 2.
pub type RandomNaturalPolynomialsReducedModPowerOf2 = RandomNaturalPolynomials<
    GeometricRandomNaturalValues<u64>,
    RandomNaturalsWithUpToBits,
    NonzeroValues<RandomNaturalsWithUpToBits>,
>;

/// Generates random [`NaturalPolynomial`]s that are reduced modulo $2^k$.
///
/// A polynomial is reduced modulo $2^k$ when every one of its coefficients is, so the coefficients
/// are [`Natural`]s with no more than $k$ bits, built directly out of random bits rather than drawn
/// and rejected; the leading coefficient, which may not be zero, comes from the same source with
/// the zeros filtered out.
///
/// The lengths — the number of coefficients, which is one more than the degree, or zero for the
/// zero polynomial — are sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`. Unlike [`random_natural_polynomials`], there is no mean bit count to
/// choose: the coefficients are uniform over the whole of $[0, 2^k)$.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell k)$
///
/// $M(i) = O(\ell k)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $\ell$ is the number
/// of coefficients of the $i$th output, and $k$ is `pow`.
///
/// # Panics
/// Panics if `pow` is zero, if `mean_length_numerator` or `mean_length_denominator` are zero, or if
/// their ratio is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_polynomials_reduced_mod_power_of_2(EXAMPLE_SEED, 4, 2, 1),
///         5
///     ),
///     "[2*x^5+10*x^4+5*x^3+14*x^2+6*x+5, 4, 13*x^7+9*x^5+x^4+10*x^3+5*x^2+13*x+2, 11, \
///     12*x^13+14*x^12+9*x^11+12*x^10+3*x^9+9*x^8+11*x^7+x^6+7*x^5+4*x^4+4*x^3+13*x^2+9*x+14, \
///     ...]"
/// );
/// ```
#[inline]
pub fn random_natural_polynomials_reduced_mod_power_of_2(
    seed: Seed,
    pow: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomNaturalPolynomialsReducedModPowerOf2 {
    assert_ne!(
        pow, 0,
        "the only polynomial reduced modulo 2^0 is the zero polynomial"
    );
    random_natural_polynomials_from_iterators(
        seed,
        &|seed_2| random_naturals_less_than_power_of_2(seed_2, pow),
        &|seed_2| nonzero_values(random_naturals_less_than_power_of_2(seed_2, pow)),
        mean_length_numerator,
        mean_length_denominator,
    )
}

/// The type of the striped [`NaturalPolynomial`] generator whose coefficients are reduced modulo a
/// power of 2.
pub type StripedRandomNaturalPolynomialsReducedModPowerOf2 = RandomNaturalPolynomials<
    GeometricRandomNaturalValues<u64>,
    StripedRandomNaturalsWithUpToBits,
    NonzeroValues<StripedRandomNaturalsWithUpToBits>,
>;

/// Generates random [`NaturalPolynomial`]s that are reduced modulo $2^k$, with striped
/// coefficients.
///
/// As for [`random_natural_polynomials_reduced_mod_power_of_2`], each coefficient is built directly
/// out of $k$ bits, so it is reduced by construction; here those bits come from a
/// [`StripedBitSource`](malachite_base::num::random::striped::StripedBitSource), so they come in
/// long runs of equal bits.
///
/// The lengths — the number of coefficients, which is one more than the degree, or zero for the
/// zero polynomial — are sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(\ell k)$
///
/// $M(i) = O(\ell k)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $\ell$ is the number
/// of coefficients of the $i$th output, and $k$ is `pow`.
///
/// # Panics
/// Panics if `pow` is zero, if `mean_stripe_denominator` is zero, if `mean_stripe_numerator <
/// mean_stripe_denominator`, if `mean_length_numerator` or `mean_length_denominator` are zero, or
/// if their ratio is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_natural_polynomials_reduced_mod_power_of_2(
///             EXAMPLE_SEED,
///             8,
///             8,
///             1,
///             2,
///             1
///         ),
///         5
///     ),
///     "[31*x^5+224*x^4+248*x^3+31*x^2+59*x+255, 7, \
///     246*x^7+14*x^4+255*x^3+255*x^2+128*x+192, 143, \
///     62*x^13+249*x^12+254*x^11+247*x^10+252*x^9+63*x^8+27*x^7+225*x^6+4*x^5+192*x^4+255*x^3+\
///     128*x^2, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_natural_polynomials_reduced_mod_power_of_2(
    seed: Seed,
    pow: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StripedRandomNaturalPolynomialsReducedModPowerOf2 {
    assert_ne!(
        pow, 0,
        "the only polynomial reduced modulo 2^0 is the zero polynomial"
    );
    random_natural_polynomials_from_iterators(
        seed,
        &|seed_2| {
            striped_random_naturals_less_than_power_of_2(
                seed_2,
                pow,
                mean_stripe_numerator,
                mean_stripe_denominator,
            )
        },
        &|seed_2| {
            nonzero_values(striped_random_naturals_less_than_power_of_2(
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

/// The type of the [`NaturalPolynomial`] generator whose coefficients are reduced modulo a number.
pub type RandomNaturalPolynomialsReducedMod = RandomNaturalPolynomials<
    GeometricRandomNaturalValues<u64>,
    RandomNaturalsLessThan,
    NonzeroValues<RandomNaturalsLessThan>,
>;

/// Generates random [`NaturalPolynomial`]s that are reduced modulo $m$.
///
/// A polynomial is reduced modulo $m$ when every one of its coefficients is, so the coefficients
/// are sampled uniformly from $[0, m)$ and the leading coefficient, which may not be zero, from
/// $[1, m)$ — which is the same distribution with the zeros filtered out.
///
/// Unlike [`random_natural_polynomials_reduced_mod_power_of_2`], a coefficient here cannot be built
/// reduced: $m$ is not a bit-width boundary, so a [`Natural`] of the right bit length may still be
/// too large and has to be drawn again. At most half of the draws are wasted, since $m$ is more
/// than half of the next power of 2.
///
/// The lengths — the number of coefficients, which is one more than the degree, or zero for the
/// zero polynomial — are sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`.
///
/// # Expected complexity per iteration
/// $T(i) = O(\ell n)$
///
/// $M(i) = O(\ell n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $\ell$ is the number
/// of coefficients of the $i$th output, and $n$ is `m.significant_bits()`.
///
/// # Panics
/// Panics if `m` is less than 2, if `mean_length_numerator` or `mean_length_denominator` are zero,
/// or if their ratio is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         random_natural_polynomials_reduced_mod(EXAMPLE_SEED, &Natural::from(10u32), 2, 1),
///         5
///     ),
///     "[2*x^5+5*x^4+2*x^3+5*x^2+6*x+5, 4, 6*x^7+7*x^6+4*x^5+4*x^4+9*x^3+9*x+1, 2, \
///     9*x^13+3*x^12+3*x^11+6*x^10+5*x^9+9*x^8+7*x^7+4*x^6+3*x^5+9*x^3+3*x^2+9*x+1, ...]"
/// );
/// ```
#[inline]
pub fn random_natural_polynomials_reduced_mod(
    seed: Seed,
    m: &Natural,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomNaturalPolynomialsReducedMod {
    assert!(
        *m >= 2u32,
        "nothing is reduced modulo 0, and only the zero polynomial is reduced modulo 1"
    );
    random_natural_polynomials_from_iterators(
        seed,
        &|seed_2| random_naturals_less_than(seed_2, m.clone()),
        &|seed_2| nonzero_values(random_naturals_less_than(seed_2, m.clone())),
        mean_length_numerator,
        mean_length_denominator,
    )
}

/// The type of the striped [`NaturalPolynomial`] generator whose coefficients are reduced modulo a
/// number.
pub type StripedRandomNaturalPolynomialsReducedMod = RandomNaturalPolynomials<
    GeometricRandomNaturalValues<u64>,
    StripedRandomNaturalInclusiveRange,
    StripedRandomNaturalInclusiveRange,
>;

/// Generates random [`NaturalPolynomial`]s that are reduced modulo $m$, with striped coefficients.
///
/// The coefficients are striped [`Natural`]s in $[0, m)$, and the leading coefficient, which may
/// not be zero, is a striped [`Natural`] in $[1, m)$.
///
/// As for [`random_natural_polynomials_reduced_mod`], an arbitrary $m$ is not a bit-width boundary,
/// so the striping and the reduction are two restrictions rather than one: a striped value in a
/// range keeps the long runs of equal bits that the range allows, and the coefficients just below
/// $m$ are the ones whose bit patterns are least free.
///
/// The lengths — the number of coefficients, which is one more than the degree, or zero for the
/// zero polynomial — are sampled from a geometric distribution with mean `mean_length_numerator /
/// mean_length_denominator`.
///
/// # Expected complexity per iteration
/// $T(i) = O(\ell n)$
///
/// $M(i) = O(\ell n)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $\ell$ is the number
/// of coefficients of the $i$th output, and $n$ is `m.significant_bits()`.
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
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_polynomial::random::*;
///
/// assert_eq!(
///     prefix_to_string(
///         striped_random_natural_polynomials_reduced_mod(
///             EXAMPLE_SEED,
///             &Natural::from(1000u32),
///             8,
///             1,
///             2,
///             1
///         ),
///         5
///     ),
///     "[x^5+3*x^4+x^3+62*x^2+952*x+999, 1, x^7+511*x^3+7*x^2+999*x+7, 775, \
///     992*x^13+959*x^12+3*x^11+481*x^10+512*x^9+636*x^8+992*x^7+33*x^6+799*x^5+639*x^4+542*x^3+\
///     479*x^2+992*x+127, ...]"
/// );
/// ```
#[inline]
pub fn striped_random_natural_polynomials_reduced_mod(
    seed: Seed,
    m: &Natural,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> StripedRandomNaturalPolynomialsReducedMod {
    assert!(
        *m >= 2u32,
        "nothing is reduced modulo 0, and only the zero polynomial is reduced modulo 1"
    );
    random_natural_polynomials_from_iterators(
        seed,
        &|seed_2| {
            striped_random_natural_range(
                seed_2,
                Natural::ZERO,
                m.clone(),
                mean_stripe_numerator,
                mean_stripe_denominator,
            )
        },
        &|seed_2| {
            striped_random_natural_range(
                seed_2,
                Natural::ONE,
                m.clone(),
                mean_stripe_numerator,
                mean_stripe_denominator,
            )
        },
        mean_length_numerator,
        mean_length_denominator,
    )
}
