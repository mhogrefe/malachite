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

use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::coefficient::{
    PolynomialCoefficient, trim_coefficients, truncate_coefficients,
};
use crate::integer_polynomial::arithmetic::mul_dispatch::{
    TinyKernel, classical_preferred, fft_preferred, karatsuba_preferred,
    schonhage_strassen_preferred, tiny_kernel,
};
use crate::integer_polynomial::arithmetic::mul_middle::fft::mul_middle_to_out_fft;
use crate::integer_polynomial::arithmetic::mul_truncated::classical::mul_truncated_to_out_classical;
use crate::integer_polynomial::arithmetic::mul_truncated::karatsuba::mul_truncated_to_out_karatsuba;
use crate::integer_polynomial::arithmetic::mul_truncated::kronecker::mul_truncated_to_out_kronecker;
use crate::integer_polynomial::arithmetic::mul_truncated::schonhage_strassen::*;
use crate::integer_polynomial::arithmetic::mul_truncated::tiny::{
    mul_truncated_to_out_tiny_1, mul_truncated_to_out_tiny_2,
};
use crate::integer_polynomial::arithmetic::square_truncated::square_truncated_to_out;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::{swap, take};
use core::ptr;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{MulTruncated, MulTruncatedAssign};

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
// `out.len()`, except that it chooses Schönhage–Strassen in a measured window (see
// `schonhage_strassen_preferred`) rather than FLINT's.
crate_test_fn! {mul_truncated_to_out<C: PolynomialCoefficient>(out: &mut [C], xs: &[C], ys: &[C]) {
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
        C::vec_mul_scalar_to_out(out, xs, &ys[0]);
        return;
    }
    if ptr::eq(xs, ys) {
        square_truncated_to_out(out, xs);
        return;
    }
    let bits1 = vec_max_bits(xs).0;
    let bits2 = vec_max_bits(ys).0;
    let len1 = u64::exact_from(xs.len());
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
        None if schonhage_strassen_preferred(len1, len2, bits1, bits2, 4097) => {
            mul_truncated_to_out_schonhage_strassen(out, xs, ys);
        }
        None => mul_truncated_to_out_kronecker(out, xs, ys),
    }
}}

// The coefficients of the product of the polynomials with coefficients `xs` and `ys`, keeping only
// the coefficients of $x^i$ for $i$ less than `len`, without zeros at the end.
//
// This is equivalent to `fmpz_poly_mullow` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
pub(crate) fn mul_truncated_ref_ref<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    len: u64,
) -> Vec<C> {
    if xs.is_empty() || ys.is_empty() || len == 0 {
        return Vec::new();
    }
    let n = usize::try_from(len)
        .unwrap_or(usize::MAX)
        .min(xs.len() + ys.len() - 1);
    let mut out = vec![C::ZERO; n];
    mul_truncated_to_out(&mut out, xs, ys);
    trim_coefficients(&mut out);
    out
}

// Multiplies the polynomial with coefficients `xs` by the one with coefficients `ys`, keeping only
// the coefficients of $x^i$ for $i$ less than `len`. When `ys` is a constant, the product is a
// scalar multiple of the truncation of `xs`, computed in place in its `Vec`.
pub(crate) fn mul_truncated_val_ref<C: PolynomialCoefficient>(
    mut xs: Vec<C>,
    ys: &[C],
    len: u64,
) -> Vec<C> {
    if let [c] = ys {
        truncate_coefficients(&mut xs, len);
        C::vec_mul_scalar_assign(&mut xs, c);
        xs
    } else {
        mul_truncated_ref_ref(&xs, ys, len)
    }
}

// Multiplies the polynomial with coefficients `xs` by the one with coefficients `ys`, keeping only
// the coefficients of $x^i$ for $i$ less than `len`. When either is a constant, the product is
// computed in place in the other's `Vec`.
pub(crate) fn mul_truncated_val_val<C: PolynomialCoefficient>(
    xs: Vec<C>,
    ys: Vec<C>,
    len: u64,
) -> Vec<C> {
    if xs.len() == 1 {
        mul_truncated_val_ref(ys, &xs, len)
    } else {
        mul_truncated_val_ref(xs, &ys, len)
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
        Self {
            coefficients: mul_truncated_val_val(self.coefficients, other.coefficients, len),
        }
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
        Self {
            coefficients: mul_truncated_val_ref(self.coefficients, &other.coefficients, len),
        }
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
        IntegerPolynomial {
            coefficients: mul_truncated_val_ref(other.coefficients, &self.coefficients, len),
        }
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
        IntegerPolynomial {
            coefficients: mul_truncated_ref_ref(&self.coefficients, &other.coefficients, len),
        }
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
        self.coefficients =
            mul_truncated_val_val(take(&mut self.coefficients), other.coefficients, len);
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
        self.coefficients =
            mul_truncated_val_ref(take(&mut self.coefficients), &other.coefficients, len);
    }
}
