// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::{FromRationalRoots, FromRoots};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

impl FromRoots<Rational> for RationalPolynomial {
    // $\prod_i (x - p_i/q_i) = \prod_i (q_i x - p_i) / \prod_i q_i$, and the leading coefficient of
    // the integer product is $\prod_i q_i$. The integer product is primitive, as a product of
    // primitive polynomials, so it shares no factor with the denominator and the pair is already
    // reduced.
    /// Builds the monic [`RationalPolynomial`] whose roots, with multiplicity, are the
    /// [`Rational`]s in `roots`.
    ///
    /// $$
    /// f((r_i)_{i=0}^{n-1}) = \prod_{i=0}^{n-1} (x - r_i).
    /// $$
    ///
    /// The integer polynomial $\prod_i (q_i x - p_i)$ of the roots $p_i/q_i$, built by
    /// [`from_rational_roots`](FromRationalRoots::from_rational_roots), is divided by its leading
    /// coefficient $\prod_i q_i$. That polynomial is primitive, so the quotient is already in
    /// lowest terms.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log^2 (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `roots.len()`, and $m$ is the largest
    /// number of significant bits of any numerator or denominator of a root.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::FromRoots;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(RationalPolynomial::from_roots(&[]).to_string(), "1");
    /// // (x - 1/2)(x + 2/3)
    /// let roots = [Rational::from_str("1/2").unwrap(), Rational::from_str("-2/3").unwrap()];
    /// assert_eq!(
    ///     RationalPolynomial::from_roots(&roots).to_string(),
    ///     "x^2+1/6*x-1/3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_set_fmpz_poly` and `fmpq_poly_scalar_div_fmpz` applied to
    /// the result of `fmpz_poly_product_roots_fmpq_vec` from `fmpz_poly/product_roots_fmpq_vec.c`,
    /// FLINT 3.6.0.
    fn from_roots(roots: &[Rational]) -> Self {
        let numerator = IntegerPolynomial::from_rational_roots(roots);
        let denominator = Natural::exact_from(numerator.leading_coefficient());
        Self {
            numerator,
            denominator,
        }
    }
}
