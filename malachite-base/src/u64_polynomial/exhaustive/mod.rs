// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::exhaustive::{
    PrimitiveIntIncreasingRange, exhaustive_positive_primitive_ints, exhaustive_unsigneds,
};
use crate::u64_polynomial::U64Polynomial;
use crate::vecs::exhaustive::{
    ExhaustiveFixedLengthVecsWithLast, ExhaustiveVecsWithLast, exhaustive_vecs_with_last,
    exhaustive_vecs_with_last_fixed_length, exhaustive_vecs_with_last_length_inclusive_range,
    exhaustive_vecs_with_last_min_length,
};

/// Generates all [`U64Polynomial`]s with coefficients from one iterator and leading coefficients
/// from another.
///
/// This `struct` is created by [`exhaustive_u64_polynomials_from_iterators`]; see its documentation
/// for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveU64Polynomials<
    I: Clone + Iterator<Item = u64>,
    J: Clone + Iterator<Item = u64>,
>(ExhaustiveVecsWithLast<u64, PrimitiveIntIncreasingRange<u64>, I, J>);

impl<I: Clone + Iterator<Item = u64>, J: Clone + Iterator<Item = u64>> Iterator
    for ExhaustiveU64Polynomials<I, J>
{
    type Item = U64Polynomial;

    #[inline]
    fn next(&mut self) -> Option<U64Polynomial> {
        self.0.next().map(U64Polynomial::from_coefficients_asc)
    }
}

/// The type of the [`U64Polynomial`] generators that draw every coefficient from every [`u64`], and
/// every leading coefficient from every positive one.
pub type ExhaustiveU64PolynomialsFromU64s =
    ExhaustiveU64Polynomials<PrimitiveIntIncreasingRange<u64>, PrimitiveIntIncreasingRange<u64>>;

/// Generates all [`U64Polynomial`]s whose coefficients come from one iterator and whose leading
/// coefficients come from another.
///
/// A polynomial is its coefficients, and the only thing that distinguishes them from any other
/// [`Vec`] of [`u64`]s is that the last of them may not be zero. Singling out that one coefficient
/// is therefore all it takes: `xs` supplies every coefficient below the leading one, and `ys`
/// supplies the leading one.
///
/// `ys` should produce no zeros, since a polynomial's leading coefficient is never zero. If it
/// does, the zeros are trimmed away, and the output has repetitions.
///
/// The leading coefficient grows at the same rate as the others, which is what makes the output
/// balanced: drawing a polynomial's lower coefficients and its leading one separately and pairing
/// them would grow the two apart. Degree $d$ is first reached after $O(d^3)$ outputs, which is slow
/// enough to leave room for the coefficients and fast enough that constants are not most of what
/// comes out.
///
/// The zero polynomial has no coefficients at all, and so takes nothing from either iterator; it is
/// generated once, first.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(d + T^\prime(i))$
///
/// $M(i) = O(d + M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $T^\prime$ and
/// $M^\prime$ are the time and memory functions of `xs` and `ys`, and $d$ is the degree of the
/// $i$th output.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::{
///     exhaustive_positive_primitive_ints, exhaustive_unsigneds,
/// };
/// use malachite_base::u64_polynomial::exhaustive::exhaustive_u64_polynomials_from_iterators;
///
/// // The same polynomials `exhaustive_u64_polynomials` gives.
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_u64_polynomials_from_iterators(
///             exhaustive_unsigneds::<u64>(),
///             exhaustive_positive_primitive_ints::<u64>()
///         ),
///         10
///     ),
///     "[0, 1, 2, x, 3, 2*x, 4, x+1, x^2, x^3, ...]"
/// );
///
/// // Only the leading coefficient is restricted, so the lower ones may be anything `xs` gives.
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_u64_polynomials_from_iterators(
///             exhaustive_positive_primitive_ints::<u64>(),
///             exhaustive_positive_primitive_ints::<u64>()
///         ),
///         10
///     ),
///     "[0, 1, 2, x+1, 3, 2*x+1, 4, x+2, x^2+x+1, x^3+x^2+x+1, ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_u64_polynomials_from_iterators<
    I: Clone + Iterator<Item = u64>,
    J: Clone + Iterator<Item = u64>,
>(
    xs: I,
    ys: J,
) -> ExhaustiveU64Polynomials<I, J> {
    ExhaustiveU64Polynomials(exhaustive_vecs_with_last(xs, ys))
}

/// Generates all [`U64Polynomial`]s.
///
/// This is [`exhaustive_u64_polynomials_from_iterators`] with every [`u64`] available as a
/// coefficient and every positive one as a leading coefficient, which between them are every
/// polynomial there is.
///
/// The output length is infinite, and degree $d$ is first reached after $O(d^3)$ outputs.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(d + \ell)$
///
/// $M(i) = O(d + \ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $d$ is the degree of
/// the $i$th output, and $\ell$ is the number of significant bits of its largest coefficient.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::{
///     exhaustive_positive_primitive_ints, exhaustive_unsigneds,
/// };
/// use malachite_base::u64_polynomial::exhaustive::exhaustive_u64_polynomials;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_u64_polynomials(), 20),
///     "[0, 1, 2, x, 3, 2*x, 4, x+1, x^2, x^3, 2*x^2, 2*x^3, x^2+x, x^3+x^2, 2*x^2+x, \
///     2*x^3+x^2, 5, 2*x+1, 6, 3*x, ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_u64_polynomials()
-> ExhaustiveU64Polynomials<PrimitiveIntIncreasingRange<u64>, PrimitiveIntIncreasingRange<u64>> {
    exhaustive_u64_polynomials_from_iterators(
        exhaustive_unsigneds::<u64>(),
        exhaustive_positive_primitive_ints::<u64>(),
    )
}

/// Generates all [`U64Polynomial`]s of a given degree.
///
/// This `struct` is created by [`exhaustive_u64_polynomials_with_degree`]; see its documentation
/// for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveU64PolynomialsWithDegree(
    ExhaustiveFixedLengthVecsWithLast<
        u64,
        PrimitiveIntIncreasingRange<u64>,
        PrimitiveIntIncreasingRange<u64>,
    >,
);

impl Iterator for ExhaustiveU64PolynomialsWithDegree {
    type Item = U64Polynomial;

    #[inline]
    fn next(&mut self) -> Option<U64Polynomial> {
        self.0.next().map(U64Polynomial::from_coefficients_asc)
    }
}

/// Generates all [`U64Polynomial`]s of a given degree.
///
/// A polynomial of degree $d$ has $d+1$ coefficients, of which the leading one is positive. The
/// zero polynomial is never generated: it has no degree at all, so no degree is the one it has.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(d + \ell)$
///
/// $M(i) = O(d + \ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $d$ is `degree`, and
/// $\ell$ is the number of significant bits of the $i$th output's largest coefficient.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::{
///     exhaustive_positive_primitive_ints, exhaustive_unsigneds,
/// };
/// use malachite_base::u64_polynomial::exhaustive::exhaustive_u64_polynomials_with_degree;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_u64_polynomials_with_degree(0), 10),
///     "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(exhaustive_u64_polynomials_with_degree(2), 10),
///     "[x^2, 2*x^2, x^2+x, 2*x^2+x, x^2+1, 2*x^2+1, x^2+x+1, 2*x^2+x+1, 3*x^2, 4*x^2, ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_u64_polynomials_with_degree(degree: u64) -> ExhaustiveU64PolynomialsWithDegree {
    ExhaustiveU64PolynomialsWithDegree(exhaustive_vecs_with_last_fixed_length(
        degree.saturating_add(1),
        exhaustive_unsigneds::<u64>(),
        exhaustive_positive_primitive_ints::<u64>(),
    ))
}

/// Generates all [`U64Polynomial`]s with a minimum degree.
///
/// The zero polynomial is never generated: it has no degree at all, so it is not of any degree at
/// least `min_degree`. Every other polynomial of degree at least `min_degree` is generated once.
///
/// The output length is infinite, and degree $d$ is first reached after $O(d^3)$ outputs.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(d + \ell)$
///
/// $M(i) = O(d + \ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $d$ is the degree of
/// the $i$th output, and $\ell$ is the number of significant bits of its largest coefficient.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::{
///     exhaustive_positive_primitive_ints, exhaustive_unsigneds,
/// };
/// use malachite_base::u64_polynomial::exhaustive::exhaustive_u64_polynomials_min_degree;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_u64_polynomials_min_degree(0), 10),
///     "[1, x, 2, 2*x, 3, x+1, 4, 2*x+1, x^2, x^3, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(exhaustive_u64_polynomials_min_degree(2), 10),
///     "[x^2, x^3, 2*x^2, 2*x^3, x^2+x, x^3+x^2, 2*x^2+x, 2*x^3+x^2, x^4, x^5, ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_u64_polynomials_min_degree(min_degree: u64) -> ExhaustiveU64PolynomialsFromU64s {
    ExhaustiveU64Polynomials(exhaustive_vecs_with_last_min_length(
        min_degree.saturating_add(1),
        exhaustive_unsigneds::<u64>(),
        exhaustive_positive_primitive_ints::<u64>(),
    ))
}

/// Generates all [`U64Polynomial`]s with degrees in $[a, b)$.
///
/// The zero polynomial is never generated: it has no degree at all, so its degree is in no range.
///
/// If $a \geq b$, the output is empty.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(d + \ell)$
///
/// $M(i) = O(d + \ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $d$ is the degree of
/// the $i$th output, and $\ell$ is the number of significant bits of its largest coefficient.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::{
///     exhaustive_positive_primitive_ints, exhaustive_unsigneds,
/// };
/// use malachite_base::u64_polynomial::exhaustive::exhaustive_u64_polynomials_degree_range;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_u64_polynomials_degree_range(1, 3), 10),
///     "[x, x^2, 2*x, 2*x^2, x+1, x^2+x, 2*x+1, 2*x^2+x, 3*x, x^2+1, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(exhaustive_u64_polynomials_degree_range(1, 1), 10),
///     "[]"
/// );
/// ```
#[inline]
pub fn exhaustive_u64_polynomials_degree_range(a: u64, b: u64) -> ExhaustiveU64PolynomialsFromU64s {
    // Degrees $[a, b)$ are lengths $[a+1, b]$, which is why an inclusive range is what this reaches
    // for: written as the half-open length range $[a+1, b+1)$, a $b$ of `u64::MAX` would overflow.
    if a >= b {
        exhaustive_u64_polynomials_degree_inclusive_range(1, 0)
    } else {
        exhaustive_u64_polynomials_degree_inclusive_range(a, b - 1)
    }
}

/// Generates all [`U64Polynomial`]s with degrees in $[a, b]$.
///
/// The zero polynomial is never generated: it has no degree at all, so its degree is in no range.
///
/// If $a > b$, the output is empty.
///
/// # Worst-case complexity per iteration
/// $T(i) = O(d + \ell)$
///
/// $M(i) = O(d + \ell)$
///
/// where $T$ is time, $M$ is additional memory, $i$ is the iteration number, $d$ is the degree of
/// the $i$th output, and $\ell$ is the number of significant bits of its largest coefficient.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::{
///     exhaustive_positive_primitive_ints, exhaustive_unsigneds,
/// };
/// use malachite_base::u64_polynomial::exhaustive::*;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_u64_polynomials_degree_inclusive_range(1, 2), 10),
///     "[x, x^2, 2*x, 2*x^2, x+1, x^2+x, 2*x+1, 2*x^2+x, 3*x, x^2+1, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(exhaustive_u64_polynomials_degree_inclusive_range(1, 0), 10),
///     "[]"
/// );
/// ```
#[inline]
pub fn exhaustive_u64_polynomials_degree_inclusive_range(
    a: u64,
    b: u64,
) -> ExhaustiveU64PolynomialsFromU64s {
    let (a, b) = if a > b {
        (1, 0)
    } else {
        (a.saturating_add(1), b.saturating_add(1))
    };
    ExhaustiveU64Polynomials(exhaustive_vecs_with_last_length_inclusive_range(
        a,
        b,
        exhaustive_unsigneds::<u64>(),
        exhaustive_positive_primitive_ints::<u64>(),
    ))
}
