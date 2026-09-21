// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::random::{RandomNaturals, random_naturals, random_positive_naturals};
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::random::RandomUnsignedInclusiveRange;
use malachite_base::num::random::geometric::GeometricRandomNaturalValues;
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
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
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

/// Generates random [`NaturalPolynomial`]s of a given degree.
///
/// This `struct` is created by [`random_natural_polynomials_with_degree`]; see its documentation
/// for more.
#[derive(Clone, Debug)]
pub struct RandomNaturalPolynomialsWithDegree(
    RandomFixedLengthVecsWithLast<
        Natural,
        RandomNaturals<GeometricRandomNaturalValues<u64>>,
        RandomNaturals<GeometricRandomNaturalValues<u64>>,
    >,
);

impl Iterator for RandomNaturalPolynomialsWithDegree {
    type Item = NaturalPolynomial;

    #[inline]
    fn next(&mut self) -> Option<NaturalPolynomial> {
        self.0.next().map(NaturalPolynomial::from_coefficients_asc)
    }
}

/// The type of the [`NaturalPolynomial`] generators whose degrees are uniform over a range.
pub type RandomNaturalPolynomialsInDegreeRange = RandomNaturalPolynomials<
    RandomUnsignedInclusiveRange<u64>,
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
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
) -> RandomNaturalPolynomialsWithDegree {
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
