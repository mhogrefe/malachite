// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

// Compares two canonical numerator-and-denominator pairs whose numerators have the same length,
// from the highest-degree coefficient down.
//
// Both of the orders on a `RationalPolynomial` use this. They differ only in what they do when the
// lengths differ; once the lengths agree, the comparison is the same for both, so it is written
// once.
//
// The $i$th coefficients are `xs[i] / x_den` and `ys[i] / y_den`. Both denominators are positive,
// so comparing those is comparing `y_den * xs[i]` against `x_den * ys[i]` — the denominators
// never need to be made equal, and nothing needs to be reduced. This is also why no GCD or LCM
// appears anywhere here, and why the difference of the two polynomials is never formed.
//
// The denominators do not change from one coefficient to the next, so everything derived from them
// is computed once, outside the loop.
pub(crate) fn cmp_coefficients_same_length(
    xs: &[Integer],
    x_den: &Natural,
    ys: &[Integer],
    y_den: &Natural,
) -> Ordering {
    // Equal denominators cancel, leaving the coefficients to be compared as they are. This is worth
    // its own branch: it is the case for every pair of polynomials with integer coefficients, where
    // both denominators are 1, and for either side of a comparison against one.
    if x_den == y_den {
        return xs.iter().rev().cmp(ys.iter().rev());
    }
    let x_den_bits = x_den.significant_bits();
    let y_den_bits = y_den.significant_bits();
    // A denominator of 1 scales nothing, so that side's multiplication is skipped. Both being 1
    // would mean equal denominators, which is already handled.
    let scale_xs = *y_den != 1u32;
    let scale_ys = *x_den != 1u32;
    let x_multiplier = Integer::from(y_den);
    let y_multiplier = Integer::from(x_den);
    for (x, y) in xs.iter().rev().zip(ys.iter().rev()) {
        // Multiplying by a positive number leaves a sign alone, so where the two signs differ they
        // settle the comparison with no multiplication at all. `Ordering`'s own order is the order
        // of the signs it stands for, negative below zero below positive.
        let x_sign = x.sign();
        let y_sign = y.sign();
        if x_sign != y_sign {
            return x_sign.cmp(&y_sign);
        }
        if x_sign == Equal {
            continue;
        }
        // A product's bit count is the sum of its factors', give or take one, so when those sums
        // are far enough apart the larger magnitude is known without multiplying. Where the two are
        // negative the larger magnitude is the smaller value.
        let x_bits = y_den_bits + x.significant_bits();
        let y_bits = x_den_bits + y.significant_bits();
        let c = if x_bits + 1 < y_bits {
            if x_sign == Greater { Less } else { Greater }
        } else if y_bits + 1 < x_bits {
            if x_sign == Greater { Greater } else { Less }
        } else {
            match (scale_xs, scale_ys) {
                (true, true) => (x * &x_multiplier).cmp(&(y * &y_multiplier)),
                (true, false) => (x * &x_multiplier).cmp(y),
                (false, true) => x.cmp(&(y * &y_multiplier)),
                (false, false) => x.cmp(y),
            }
        };
        if c != Equal {
            return c;
        }
    }
    Equal
}

impl PartialOrd for RationalPolynomial {
    /// Compares two [`RationalPolynomial`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RationalPolynomial {
    /// Compares two [`RationalPolynomial`]s by how they behave for large arguments.
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
    /// As over the [`Integer`]s, a higher degree does not settle it by itself. A polynomial of
    /// higher degree dominates, so the difference's leading coefficient is its own; but that
    /// coefficient may be negative, in which case the dominating polynomial runs off to $-\infty$
    /// and is the *smaller* of the two. So $-x^3 < x^2$, and the zero polynomial is above every
    /// polynomial with a negative leading coefficient and below every polynomial with a positive
    /// one. Polynomials of equal degree are decided by the highest-degree coefficient at which they
    /// differ.
    ///
    /// Nothing is reduced along the way, and the difference is never formed. Writing the two as
    /// $P/a$ and $Q/b$ with $a, b > 0$, the sign that decides is the sign of the leading
    /// coefficient of $bP - aQ$, and the $ab$ underneath it is positive and so irrelevant; a
    /// coefficient comparison is therefore a comparison of $b P_i$ against $a Q_i$, with no GCD, no
    /// LCM, and no common denominator to build. Equal denominators cancel and are compared
    /// directly, and a denominator of 1 scales nothing; within a coefficient, differing signs
    /// settle it outright, and the two products' bit counts settle it whenever they are far enough
    /// apart.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the smaller of the two polynomials'
    /// total number of bits, counting numerator coefficients and denominator. Polynomials of
    /// different degrees are compared in constant time.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // A dominating polynomial with a positive leading coefficient is the greater one.
    /// assert!(
    ///     RationalPolynomial::from_str("x^2").unwrap()
    ///         > RationalPolynomial::from_str("1000000*x").unwrap()
    /// );
    ///
    /// // With a negative leading coefficient it is the smaller one.
    /// assert!(
    ///     RationalPolynomial::from_str("-x^3").unwrap()
    ///         < RationalPolynomial::from_str("x").unwrap()
    /// );
    ///
    /// // At equal degrees the highest coefficient at which they differ decides.
    /// assert!(
    ///     RationalPolynomial::from_str("1/2*x").unwrap()
    ///         > RationalPolynomial::from_str("1/3*x").unwrap()
    /// );
    ///
    /// // Constant polynomials compare as the numbers they are.
    /// assert!(
    ///     RationalPolynomial::from_str("1/2").unwrap()
    ///         > RationalPolynomial::from_str("1/3").unwrap()
    /// );
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        if core::ptr::eq(self, other) {
            return Equal;
        }
        let xs = self.numerator.coefficients_asc();
        let ys = other.numerator.coefficients_asc();
        // The numerator is held with no trailing zero, so its length is the degree plus one, and 0
        // for the zero polynomial; comparing lengths is comparing degrees. A positive denominator
        // leaves the leading coefficient's sign alone, so the sign that decides when the degrees
        // differ can be read off the numerator.
        match xs.len().cmp(&ys.len()) {
            Equal => cmp_coefficients_same_length(xs, &self.denominator, ys, &other.denominator),
            Greater => self.numerator.leading_coefficient().sign(),
            Less => other.numerator.leading_coefficient().sign().reverse(),
        }
    }
}
