// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Sign;

impl PartialOrd for IntegerPolynomial {
    /// Compares two [`IntegerPolynomial`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IntegerPolynomial {
    /// Compares two [`IntegerPolynomial`]s by how they behave for large arguments.
    ///
    /// The greater polynomial is the one that is eventually greater: the comparison is the one that
    /// $p(x)$ and $q(x)$ eventually settle into as $x$ grows.
    ///
    /// $$
    /// f(p, q) = \lim_{x \to \infty} \operatorname{cmp}(p(x), q(x)).
    /// $$
    ///
    /// The limit always exists. $p - q$ is a polynomial, so it has finitely many roots, and past
    /// the largest of them its sign is the sign of its leading coefficient and never changes again.
    /// That also makes this a total order agreeing with [`Eq`]: the limit is $0$ exactly when $p -
    /// q$ is the zero polynomial.
    ///
    /// Finding it needs no evaluation, but a higher degree alone does not settle it the way it does
    /// over the [`Natural`](crate::natural::Natural)s. A polynomial of higher degree does dominate
    /// one of lower degree, so the difference's leading coefficient is its own; but that
    /// coefficient may be negative, in which case the dominating polynomial runs off to $-\infty$
    /// and is the *smaller* of the two. So $-x^3 < x$, and the zero polynomial is above every
    /// polynomial with a negative leading coefficient and below every polynomial with a positive
    /// one. Polynomials of equal degree are decided by the highest-degree coefficient at which they
    /// differ, since that term eventually outgrows the sum of everything below it.
    ///
    /// This is the order that makes the polynomials an ordered ring: it is unchanged by adding a
    /// polynomial to both sides, and by multiplying both sides by a positive one. Restricted to the
    /// constant polynomials it is the order on the [`Integer`](crate::integer::Integer)s, so the
    /// embedding of a number as a polynomial preserves comparisons. It is not a well-order, and no
    /// order compatible with addition can be: $x > x - 1 > x - 2 > \ldots$ descends forever.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the smaller of the two polynomials'
    /// total number of bits, summed over their coefficients. Polynomials of different degrees are
    /// compared in constant time.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // A higher degree dominates, and a positive leading coefficient makes it the greater.
    /// assert!(
    ///     IntegerPolynomial::from_str("x^2").unwrap()
    ///         > IntegerPolynomial::from_str("1000000*x").unwrap()
    /// );
    ///
    /// // But a dominating polynomial with a negative leading coefficient runs off downwards, so
    /// // it is the smaller one.
    /// assert!(
    ///     IntegerPolynomial::from_str("-x^3").unwrap()
    ///         < IntegerPolynomial::from_str("x").unwrap()
    /// );
    ///
    /// // The zero polynomial sits between the two signs.
    /// assert!(
    ///     IntegerPolynomial::from_str("-x").unwrap() < IntegerPolynomial::from_str("0").unwrap()
    /// );
    /// assert!(
    ///     IntegerPolynomial::from_str("0").unwrap() < IntegerPolynomial::from_str("x").unwrap()
    /// );
    ///
    /// // At equal degrees the highest coefficient at which they differ decides.
    /// assert!(
    ///     IntegerPolynomial::from_str("-x^2+5").unwrap()
    ///         > IntegerPolynomial::from_str("-2*x^2+1000000").unwrap()
    /// );
    ///
    /// // Constant polynomials compare as the numbers they are.
    /// assert!(
    ///     IntegerPolynomial::from_str("-122").unwrap()
    ///         > IntegerPolynomial::from_str("-123").unwrap()
    /// );
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        if core::ptr::eq(self, other) {
            return Equal;
        }
        // The coefficients are held with no trailing zero, so the length is the degree plus one,
        // and 0 for the zero polynomial; comparing lengths is comparing degrees.
        match self.coefficients.len().cmp(&other.coefficients.len()) {
            // Equal degrees: the iterators have the same length, so comparing them from the leading
            // coefficient down is the comparison at the highest coefficient where the two differ.
            // Two zero polynomials compare equal, both iterators being empty.
            Equal => self
                .coefficients
                .iter()
                .rev()
                .cmp(other.coefficients.iter().rev()),
            // Different degrees: the difference's leading coefficient is the dominating
            // polynomial's own, so its sign decides. Normalization makes it nonzero, so there is no
            // `Equal` to fall through to.
            Greater => self.leading_coefficient().sign(),
            Less => other.leading_coefficient().sign().reverse(),
        }
    }
}
