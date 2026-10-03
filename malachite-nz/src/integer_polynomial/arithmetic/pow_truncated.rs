// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::coefficient::{
    PolynomialCoefficient, trim_coefficients, truncate_coefficients,
};
use crate::integer_polynomial::arithmetic::mul_truncated::mul_truncated_to_out;
use crate::integer_polynomial::arithmetic::pow::binexp::binexp_start;
use crate::integer_polynomial::arithmetic::square_truncated::square_truncated_to_out;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::PowAssign;
use malachite_base::num::conversion::traits::SaturatingFrom;
use malachite_base::polynomial::{PowTruncated, PowTruncatedAssign};

// Sets `out` to the first `out.len()` coefficients of the `e`th power of the polynomial with
// coefficients `xs`, which is nonempty, where `e` is at least 3. `out.len()` must be positive and
// at most `e * (xs.len() - 1) + 1`.
//
// This is left-to-right binary exponentiation with truncated squaring and multiplication,
// alternating between `out` and a scratch buffer as in `pow_to_out_binexp`. Each intermediate power
// is kept at its true length, capped at `out.len()`.
//
// This is equivalent to `_fmpz_poly_pow_trunc` from `fmpz_poly/pow_trunc.c`, FLINT 3.6.0, where `n`
// is `out.len()`, except that FLINT pads the polynomial with zeros to length `n` and computes every
// intermediate power to length `n`.
crate_test_fn! {pow_truncated_to_out<C: PolynomialCoefficient>(out: &mut [C], xs: &[C], e: u64) {
    let n = out.len();
    let xs = &xs[..xs.len().min(n)];
    let len = xs.len();
    let mut v = vec![C::ZERO; n];
    let (mut bit, swaps) = binexp_start(e);
    let (mut r, mut s): (&mut [C], &mut [C]) = if swaps { (&mut v, out) } else { (out, &mut v) };
    // The first step squares xs itself
    let mut rlen = ((len << 1) - 1).min(n);
    square_truncated_to_out(&mut r[..rlen], xs);
    if bit & e != 0 {
        let new_len = (rlen + len - 1).min(n);
        mul_truncated_to_out(&mut s[..new_len], &r[..rlen], xs);
        rlen = new_len;
        swap(&mut r, &mut s);
    }
    loop {
        bit >>= 1;
        if bit == 0 {
            break;
        }
        let new_len = ((rlen << 1) - 1).min(n);
        square_truncated_to_out(&mut s[..new_len], &r[..rlen]);
        rlen = new_len;
        if bit & e != 0 {
            let new_len = (rlen + len - 1).min(n);
            mul_truncated_to_out(&mut r[..new_len], &s[..rlen], xs);
            rlen = new_len;
        } else {
            swap(&mut r, &mut s);
        }
    }
}}

// Returns the coefficients of the `e`th power of the polynomial with coefficients `xs`, which has
// no zeros at the end, keeping only the coefficients of $x^i$ for $i$ less than `len`, without
// zeros at the end.
//
// Writing the polynomial modulo $x^n$ as $x^\ell q$, with $q_0 \neq 0$, its power is $x^{e\ell}
// (q^e \bmod x^{n - e\ell})$, or 0 if $e\ell \geq n$.
//
// This is equivalent to `fmpz_poly_pow_trunc` from `fmpz_poly/pow_trunc.c`, FLINT 3.6.0, except
// that it removes the factor of $x^\ell$, and that `n` is capped at the length of the untruncated
// power, so that a `len` too large to allocate means no truncation.
crate_test_fn! {pow_truncated_ref<C: PolynomialCoefficient>(xs: &[C], e: u64, len: u64) -> Vec<C> {
    if len == 0 {
        return Vec::new();
    }
    if e == 0 {
        return vec![C::ONE];
    }
    let n = usize::saturating_from(len);
    let xs = &xs[..xs.len().min(n)];
    let Some(low) = xs.iter().position(|x| !x.is_zero()) else {
        return Vec::new();
    };
    let shift = usize::saturating_from(e).saturating_mul(low);
    if shift >= n {
        return Vec::new();
    }
    let q = &xs[low..];
    let q_len = (n - shift).min(
        usize::saturating_from(e)
            .saturating_mul(q.len() - 1)
            .saturating_add(1),
    );
    let mut out = vec![C::ZERO; shift + q_len];
    let out_q = &mut out[shift..];
    match (q_len, e) {
        (1, _) => out_q[0] = q[0].pow_ref(e),
        (_, 1) => out_q.clone_from_slice(&q[..q_len]),
        (_, 2) => square_truncated_to_out(out_q, q),
        _ => pow_truncated_to_out(out_q, q, e),
    }
    trim_coefficients(&mut out);
    out
}}

// Replaces the coefficients `xs` of a polynomial, which has no zeros at the end, with those of its
// `e`th power truncated to length `len`, reusing `xs` when the power of a constant is computed or
// the power is the polynomial itself.
pub(crate) fn pow_truncated_assign_vec<C: PolynomialCoefficient + PowAssign<u64>>(
    xs: &mut Vec<C>,
    e: u64,
    len: u64,
) {
    match (xs.len(), e, len) {
        (_, _, 0) => xs.clear(),
        (0, 0, _) => xs.push(C::ONE),
        (_, 0, _) => {
            xs.truncate(1);
            xs[0] = C::ONE;
        }
        (0, _, _) => {}
        (_, 1, _) => truncate_coefficients(xs, len),
        (1, _, _) => xs[0].pow_assign(e),
        _ => *xs = pow_truncated_ref(xs, e, len),
    }
}

impl PowTruncated for IntegerPolynomial {
    type Output = Self;

    /// Raises an [`IntegerPolynomial`] to a power, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking it by value.
    ///
    /// $$
    /// f(p, e, n) = p^e \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the power of its image modulo $x^n$,
    /// so only its first `len` coefficients are read. The zeroth power of every polynomial is 1,
    /// truncated to 0 when `len` is 0. The power is computed by repeated truncated squaring and
    /// multiplication.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `exp` times the
    /// largest number of significant bits of any of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PowTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x+1").unwrap())
    ///         .pow_truncated(5, 3)
    ///         .to_string(),
    ///     "10*x^2+5*x+1"
    /// );
    /// // The power is a multiple of x^4.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2+x").unwrap())
    ///         .pow_truncated(4, 4)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pow_trunc` from `fmpz_poly/pow_trunc.c`, FLINT 3.6.0,
    /// except that a factor of $x^k$ is removed before powering, and that the intermediate powers
    /// are kept at their own lengths rather than padded to `len`.
    #[inline]
    fn pow_truncated(mut self, exp: u64, len: u64) -> Self {
        self.pow_truncated_assign(exp, len);
        self
    }
}

impl PowTruncated for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Raises an [`IntegerPolynomial`] to a power, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking it by reference.
    ///
    /// $$
    /// f(p, e, n) = p^e \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the power of its image modulo $x^n$,
    /// so only its first `len` coefficients are read. The zeroth power of every polynomial is 1,
    /// truncated to 0 when `len` is 0. The power is computed by repeated truncated squaring and
    /// multiplication.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `exp` times the
    /// largest number of significant bits of any of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PowTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x+1").unwrap())
    ///         .pow_truncated(5, 3)
    ///         .to_string(),
    ///     "10*x^2+5*x+1"
    /// );
    /// // The power is a multiple of x^4.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2+x").unwrap())
    ///         .pow_truncated(4, 4)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pow_trunc` from `fmpz_poly/pow_trunc.c`, FLINT 3.6.0,
    /// except that a factor of $x^k$ is removed before powering, and that the intermediate powers
    /// are kept at their own lengths rather than padded to `len`.
    #[inline]
    fn pow_truncated(self, exp: u64, len: u64) -> IntegerPolynomial {
        IntegerPolynomial {
            coefficients: pow_truncated_ref(&self.coefficients, exp, len),
        }
    }
}

impl PowTruncatedAssign for IntegerPolynomial {
    /// Raises an [`IntegerPolynomial`] to a power in place, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`.
    ///
    /// $$
    /// p \gets p^e \bmod x^n.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the power of its image modulo $x^n$,
    /// so only its first `len` coefficients are read. The zeroth power of every polynomial is 1,
    /// truncated to 0 when `len` is 0. The power is computed by repeated truncated squaring and
    /// multiplication.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `exp` times the
    /// largest number of significant bits of any of the first `len` coefficients of the polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PowTruncatedAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x+1").unwrap();
    /// p.pow_truncated_assign(5, 3);
    /// assert_eq!(p.to_string(), "10*x^2+5*x+1");
    ///
    /// // The power is a multiple of x^4.
    /// let mut p = IntegerPolynomial::from_str("x^2+x").unwrap();
    /// p.pow_truncated_assign(4, 4);
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pow_trunc` from `fmpz_poly/pow_trunc.c`, FLINT 3.6.0,
    /// except that a factor of $x^k$ is removed before powering, and that the intermediate powers
    /// are kept at their own lengths rather than padded to `len`.
    #[inline]
    fn pow_truncated_assign(&mut self, exp: u64, len: u64) {
        pow_truncated_assign_vec(&mut self.coefficients, exp, len);
    }
}
