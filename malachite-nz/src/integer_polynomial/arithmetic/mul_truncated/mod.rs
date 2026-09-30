// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::mul_middle::fft::mul_middle_to_out_fft;
use crate::integer_polynomial::arithmetic::mul_truncated::classical::mul_truncated_to_out_classical;
use crate::integer_polynomial::arithmetic::mul_truncated::karatsuba::mul_truncated_to_out_karatsuba;
use crate::integer_polynomial::arithmetic::mul_truncated::kronecker::mul_truncated_to_out_kronecker;
use crate::integer_polynomial::arithmetic::mul_truncated::schonhage_strassen::*;
use crate::integer_polynomial::arithmetic::mul_truncated::tiny::{
    mul_truncated_to_out_tiny_1, mul_truncated_to_out_tiny_2,
};
use crate::integer_polynomial::arithmetic::scalar_mul::{
    integers_mul_scalar_assign, integers_mul_scalar_to_out,
};
use crate::integer_polynomial::arithmetic::square_truncated::square_truncated_to_out;
use crate::integer_polynomial::arithmetic::vec::max_bits::vec_max_bits;
use crate::integer_polynomial::arithmetic::vec::{
    TinyKernel, classical_preferred, fft_preferred, karatsuba_preferred,
    schonhage_strassen_preferred, tiny_kernel,
};
use alloc::vec;
use core::cmp::min;
use core::mem::{swap, take};
use core::ptr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{MulTruncated, MulTruncatedAssign, Polynomial};

pub mod classical;
pub mod karatsuba;
pub mod kronecker;
pub mod schonhage_strassen;
pub mod tiny;

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, both nonempty. `out.len()` must be positive and at most `xs.len() +
// ys.len() - 1`.
//
// # Worst-case complexity
// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n) \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `out.len()`, and $m$ is the largest number of
// significant bits of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0, where `n` is
// `out.len()`. FLINT's first choice for long inputs, which multiplies polynomials directly with its
// small-prime FFT (`_fmpz_poly_mul_mid_default_mpn_ctx` from `fft_small/fmpz_poly_mul.c`), and
// Schönhage–Strassen, which it chooses for some inputs of medium size, have not been ported yet;
// Kronecker substitution stands in for both. (Its single integer multiplication reaches the port of
// the small-prime FFT's integer multiplication in `natural/arithmetic/mul/fft.rs` when the operands
// are large.)
crate_test_fn! {mul_truncated_to_out(out: &mut [Integer], xs: &[Integer], ys: &[Integer]) {
    let n = out.len();
    let mut xs = &xs[..min(xs.len(), n)];
    let mut ys = &ys[..min(ys.len(), n)];
    assert_ne!(n, 0);
    assert_ne!(xs.len(), 0);
    assert_ne!(ys.len(), 0);
    assert!(n < xs.len() + ys.len());
    if xs.len() < ys.len() {
        swap(&mut xs, &mut ys);
    }
    if ys.len() == 1 {
        integers_mul_scalar_to_out(out, xs, &ys[0]);
        return;
    }
    if ptr::eq(xs, ys) {
        square_truncated_to_out(out, xs);
        return;
    }
    let bits1 = vec_max_bits(xs).0;
    let bits2 = vec_max_bits(ys).0;
    let len2 = u64::exact_from(ys.len());
    if fft_preferred(len2, bits1, bits2, 100, 200) && mul_middle_to_out_fft(out, xs, ys, 0, n) {
        return;
    }
    let n = u64::exact_from(n);
    let short_enough = len2 < 50 || (len2 << 2 >= 3 * n && n < 150 + bits1 + bits2);
    match tiny_kernel(bits1, bits2, len2, short_enough) {
        Some(TinyKernel::OneWord) => mul_truncated_to_out_tiny_1(out, xs, ys),
        Some(TinyKernel::TwoWord) => mul_truncated_to_out_tiny_2(out, xs, ys),
        None if classical_preferred(len2, bits1, bits2) => {
            mul_truncated_to_out_classical(out, xs, ys);
        }
        None if karatsuba_preferred(len2, bits1, bits2) => {
            mul_truncated_to_out_karatsuba(out, xs, ys);
        }
        None if schonhage_strassen_preferred(len2, bits1, bits2) => {
            mul_truncated_to_out_schonhage_strassen(out, xs, ys);
        }
        None => mul_truncated_to_out_kronecker(out, xs, ys),
    }
}}

// This is equivalent to `fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
fn mul_truncated_ref_ref(xs: &[Integer], ys: &[Integer], len: u64) -> IntegerPolynomial {
    if xs.is_empty() || ys.is_empty() || len == 0 {
        return IntegerPolynomial::ZERO;
    }
    let n = usize::try_from(len)
        .unwrap_or(usize::MAX)
        .min(xs.len() + ys.len() - 1);
    let mut out = vec![Integer::ZERO; n];
    mul_truncated_to_out(&mut out, xs, ys);
    let mut p = IntegerPolynomial { coefficients: out };
    p.trim();
    p
}

// Multiplies `p` by the polynomial with coefficients `ys`, keeping only the coefficients of $x^i$
// for $i$ less than `len`. When `ys` is a constant, the product is a scalar multiple of the
// truncation of `p`, computed in place in its `Vec`.
fn mul_truncated_val_ref(mut p: IntegerPolynomial, ys: &[Integer], len: u64) -> IntegerPolynomial {
    if let [c] = ys {
        p.truncate_assign(len);
        integers_mul_scalar_assign(&mut p.coefficients, c);
        p
    } else {
        mul_truncated_ref_ref(&p.coefficients, ys, len)
    }
}

// Multiplies `p` by `q`, keeping only the coefficients of $x^i$ for $i$ less than `len`. When
// either is a constant, the product is computed in place in the other's `Vec`.
fn mul_truncated_val_val(
    p: IntegerPolynomial,
    q: IntegerPolynomial,
    len: u64,
) -> IntegerPolynomial {
    if p.coefficients.len() == 1 {
        mul_truncated_val_ref(q, &p.coefficients, len)
    } else {
        mul_truncated_val_ref(p, &q.coefficients, len)
    }
}

impl MulTruncated<Self> for IntegerPolynomial {
    type Output = Self;

    /// Multiplies two [`IntegerPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less
    /// than `len`, taking both by value.
    ///
    /// $$
    /// f(p, q, n) = pq \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The product is trimmed, so
    /// when the coefficient of $x^{n-1}$ is zero, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .mul_truncated(IntegerPolynomial::from_str("2*x+5").unwrap(), 2)
    ///         .to_string(),
    ///     "-11*x+10"
    /// );
    /// // The linear coefficient cancels.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x+1").unwrap())
    ///         .mul_truncated(IntegerPolynomial::from_str("x-1").unwrap(), 2)
    ///         .to_string(),
    ///     "-1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
    #[inline]
    fn mul_truncated(self, other: Self, len: u64) -> Self {
        mul_truncated_val_val(self, other, len)
    }
}

impl MulTruncated<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Multiplies two [`IntegerPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less
    /// than `len`, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q, n) = pq \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The product is trimmed, so
    /// when the coefficient of $x^{n-1}$ is zero, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .mul_truncated(&IntegerPolynomial::from_str("2*x+5").unwrap(), 2)
    ///         .to_string(),
    ///     "-11*x+10"
    /// );
    /// // The linear coefficient cancels.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x+1").unwrap())
    ///         .mul_truncated(&IntegerPolynomial::from_str("x-1").unwrap(), 2)
    ///         .to_string(),
    ///     "-1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
    #[inline]
    fn mul_truncated(self, other: &Self, len: u64) -> Self {
        mul_truncated_val_ref(self, &other.coefficients, len)
    }
}

impl MulTruncated<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Multiplies two [`IntegerPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less
    /// than `len`, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q, n) = pq \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The product is trimmed, so
    /// when the coefficient of $x^{n-1}$ is zero, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .mul_truncated(IntegerPolynomial::from_str("2*x+5").unwrap(), 2)
    ///         .to_string(),
    ///     "-11*x+10"
    /// );
    /// // The linear coefficient cancels.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x+1").unwrap())
    ///         .mul_truncated(IntegerPolynomial::from_str("x-1").unwrap(), 2)
    ///         .to_string(),
    ///     "-1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
    #[inline]
    fn mul_truncated(self, other: IntegerPolynomial, len: u64) -> IntegerPolynomial {
        mul_truncated_val_ref(other, &self.coefficients, len)
    }
}

impl MulTruncated<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Multiplies two [`IntegerPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less
    /// than `len`, taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = pq \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only the first `len` coefficients of each are read. The product is trimmed, so
    /// when the coefficient of $x^{n-1}$ is zero, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncated;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .mul_truncated(&IntegerPolynomial::from_str("2*x+5").unwrap(), 2)
    ///         .to_string(),
    ///     "-11*x+10"
    /// );
    /// // The linear coefficient cancels.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x+1").unwrap())
    ///         .mul_truncated(&IntegerPolynomial::from_str("x-1").unwrap(), 2)
    ///         .to_string(),
    ///     "-1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
    #[inline]
    fn mul_truncated(self, other: &IntegerPolynomial, len: u64) -> IntegerPolynomial {
        mul_truncated_ref_ref(&self.coefficients, &other.coefficients, len)
    }
}

impl MulTruncatedAssign<Self> for IntegerPolynomial {
    /// Multiplies an [`IntegerPolynomial`] by another [`IntegerPolynomial`] in place, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side by value.
    ///
    /// $$
    /// p \gets pq \bmod x^n.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncatedAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p.mul_truncated_assign(IntegerPolynomial::from_str("2*x+5").unwrap(), 2);
    /// assert_eq!(p.to_string(), "-11*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
    #[inline]
    fn mul_truncated_assign(&mut self, other: Self, len: u64) {
        *self = mul_truncated_val_val(take(self), other, len);
    }
}

impl MulTruncatedAssign<&Self> for IntegerPolynomial {
    /// Multiplies an [`IntegerPolynomial`] by another [`IntegerPolynomial`] in place, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side by reference.
    ///
    /// $$
    /// p \gets pq \bmod x^n.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the largest number of
    /// significant bits of any of the first `len` coefficients of either polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulTruncatedAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p.mul_truncated_assign(&IntegerPolynomial::from_str("2*x+5").unwrap(), 2);
    /// assert_eq!(p.to_string(), "-11*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
    #[inline]
    fn mul_truncated_assign(&mut self, other: &Self, len: u64) {
        *self = mul_truncated_val_ref(take(self), &other.coefficients, len);
    }
}
