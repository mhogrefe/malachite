// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::u64_polynomial::U64Polynomial;
use core::cmp::Ordering::{self, *};

impl PartialOrd for U64Polynomial {
    /// Compares two [`U64Polynomial`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for U64Polynomial {
    /// Compares two [`U64Polynomial`]s by how they behave for large arguments.
    ///
    /// The greater polynomial is the one that is eventually greater: the comparison is the one that
    /// $p(x)$ and $q(x)$ eventually settle into as $x$ grows. The coefficients are read as the
    /// numbers they are, so the values being compared are the ones a polynomial over the integers
    /// would take, not ones reduced by any modulus.
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
    /// Finding it needs no evaluation. A polynomial of higher degree eventually outgrows one of
    /// lower degree, whatever their coefficients, so the degrees decide first; the zero polynomial,
    /// having no degree at all, is below every other polynomial. Polynomials of equal degree are
    /// decided by the highest-degree coefficient at which they differ, since that term eventually
    /// outgrows the sum of everything below it.
    ///
    /// This is the order that makes the polynomials an ordered ring: it is unchanged by adding a
    /// polynomial to both sides, and by multiplying both sides by a nonzero one. Restricted to the
    /// constant polynomials it is the order on the [`u64`]s, so the embedding of a number as a
    /// polynomial preserves comparisons. It is not a well-order, and no order compatible with
    /// addition can be: $x > x - 1 > x - 2 > \ldots$ descends forever. A well-order on polynomials
    /// needs to weigh a polynomial's size against its degree, which this order does not do.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the smaller of the two polynomials'
    /// numbers of coefficients. Polynomials of different degrees are compared in constant time.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::u64_polynomial::U64Polynomial;
    ///
    /// // A higher degree wins however small its coefficients: x^2 eventually passes 1000000*x.
    /// assert!(
    ///     U64Polynomial::from_str("x^2").unwrap() > U64Polynomial::from_str("1000000*x").unwrap()
    /// );
    ///
    /// // At equal degrees the leading coefficient decides.
    /// assert!(
    ///     U64Polynomial::from_str("2*x^2").unwrap()
    ///         > U64Polynomial::from_str("x^2+1000000").unwrap()
    /// );
    ///
    /// // When that ties, the next coefficient down does.
    /// assert!(
    ///     U64Polynomial::from_str("x^2+3*x").unwrap()
    ///         > U64Polynomial::from_str("x^2+2*x+1000000").unwrap()
    /// );
    ///
    /// // The zero polynomial is below everything else.
    /// assert!(U64Polynomial::from_str("0").unwrap() < U64Polynomial::from_str("1").unwrap());
    ///
    /// // Constant polynomials compare as the numbers they are.
    /// assert!(U64Polynomial::from_str("123").unwrap() > U64Polynomial::from_str("122").unwrap());
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        if core::ptr::eq(self, other) {
            return Equal;
        }
        // The coefficients are held with no trailing zero, so the length is the degree plus one,
        // and 0 for the zero polynomial; comparing lengths is comparing degrees. Once the lengths
        // agree the iterators have the same length, so comparing them from the leading coefficient
        // down is exactly the comparison at the highest coefficient where the two differ.
        self.coefficients
            .len()
            .cmp(&other.coefficients.len())
            .then_with(|| {
                self.coefficients
                    .iter()
                    .rev()
                    .cmp(other.coefficients.iter().rev())
            })
    }
}
