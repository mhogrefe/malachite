// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{NegAssign, SubMulAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::FromRationalRoots;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;

// The number of roots below which the product is built one factor at a time.
const FROM_RATIONAL_ROOTS_BASECASE_CUTOFF: usize = 20;

// Writes into `out`, which has length `ps.len() + 1`, the coefficients of $(q_0 x - p_0)(q_1 x -
// p_1) \cdots (q_{n-1} x - p_{n-1})$, where the `ps` are the numerators and the `qs` the
// denominators.
//
// This is equivalent to `_fmpz_poly_product_roots_fmpq_vec` from
// `fmpz_poly/product_roots_fmpq_vec.c`, FLINT 3.6.0.
fn from_rational_roots_to_out(out: &mut [Integer], ps: &[Integer], qs: &[Integer]) {
    let n = ps.len();
    assert_eq!(out.len(), n + 1);
    if n == 0 {
        out[0] = Integer::ONE;
    } else if n < FROM_RATIONAL_ROOTS_BASECASE_CUTOFF {
        out[n] = qs[0].clone();
        out[n - 1] = -&ps[0];
        for i in 1..n {
            out[n - i - 1] = &out[n - i] * &ps[i];
            out[n - i - 1].neg_assign();
            for j in 0..i {
                out[n - i + j] *= &qs[i];
                let (lo, hi) = out.split_at_mut(n - i + j + 1);
                lo[n - i + j].sub_mul_assign(&hi[0], &ps[i]);
            }
            out[n] *= &qs[i];
        }
    } else {
        let half = n.div_ceil(2);
        let mut tmp = vec![Integer::ZERO; n + 2];
        let (left, right) = tmp.split_at_mut(half + 1);
        from_rational_roots_to_out(left, &ps[..half], &qs[..half]);
        from_rational_roots_to_out(right, &ps[half..], &qs[half..]);
        let right = IntegerPolynomial::from_coefficients_asc(tmp[half + 1..].to_vec());
        tmp.truncate(half + 1);
        let product = IntegerPolynomial::from_coefficients_asc(tmp) * right;
        for (o, c) in out.iter_mut().zip(product.into_coefficients_asc()) {
            *o = c;
        }
    }
}

impl FromRationalRoots<Rational> for IntegerPolynomial {
    /// Builds the primitive [`IntegerPolynomial`], with positive leading coefficient, whose roots,
    /// with multiplicity, are the [`Rational`]s in `roots`.
    ///
    /// $$
    /// f((p_i/q_i)_{i=0}^{n-1}) = \prod_{i=0}^{n-1} (q_i x - p_i),
    /// $$
    ///
    /// where each $p_i/q_i$ is in lowest terms with $q_i > 0$. Each factor is primitive, so by
    /// Gauss's lemma the product is too, and its leading coefficient is $\prod_i q_i$.
    ///
    /// Fewer than 20 factors are multiplied in one at a time; more are split into two halves, whose
    /// products are found the same way and then multiplied.
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
    /// use malachite_base::polynomial::arithmetic::traits::FromRationalRoots;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(IntegerPolynomial::from_rational_roots(&[]).to_string(), "1");
    /// // (2x - 1)(3x + 2)
    /// let roots = [Rational::from_str("1/2").unwrap(), Rational::from_str("-2/3").unwrap()];
    /// assert_eq!(
    ///     IntegerPolynomial::from_rational_roots(&roots).to_string(),
    ///     "6*x^2+x-2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_product_roots_fmpq_vec` from
    /// `fmpz_poly/product_roots_fmpq_vec.c`, FLINT 3.6.0.
    fn from_rational_roots(roots: &[Rational]) -> Self {
        let ps: Vec<Integer> = roots
            .iter()
            .map(|r| Integer::from_sign_and_abs_ref(*r >= 0u32, r.numerator_ref()))
            .collect();
        let qs: Vec<Integer> = roots
            .iter()
            .map(|r| Integer::from(r.denominator_ref()))
            .collect();
        let mut out = vec![Integer::ZERO; roots.len() + 1];
        from_rational_roots_to_out(&mut out, &ps, &qs);
        Self::from_coefficients_asc(out)
    }
}
