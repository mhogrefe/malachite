// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010 Sebastian Pancratz
//
//      Copyright © 2026 Fredrik Johansson
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
use crate::integer_polynomial::arithmetic::mul_middle::classical::mul_middle_to_out_classical;
use crate::integer_polynomial::arithmetic::mul_middle::fft::mul_middle_to_out_fft;
use crate::integer_polynomial::arithmetic::mul_middle::kronecker::mul_middle_to_out_kronecker;
use crate::integer_polynomial::arithmetic::mul_middle::schonhage_strassen::*;
use crate::integer_polynomial::arithmetic::mul_middle::tiny::{
    mul_middle_to_out_tiny_1, mul_middle_to_out_tiny_2,
};
use crate::integer_polynomial::arithmetic::mul_truncated::karatsuba::mul_truncated_to_out_karatsuba;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::{swap, take};
use core::ptr;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{MulMiddle, MulMiddleAssign};

pub mod classical;
pub mod fft;
pub mod kronecker;
pub mod schonhage_strassen;
pub mod tiny;

// Narrows the polynomials with coefficients `xs` and `ys` to the coefficients that can contribute
// to the coefficients of $x^i$ for `nlo` $\leq i <$ `nhi` of their product, returning the narrowed
// slices and the range shifted to match. A coefficient of $x^j$ with $j \geq$ `nhi` contributes
// only above the range, and dropping low coefficients of a factor shifts the whole product down.
//
// This is equivalent to the input truncation at the start of `_fmpz_poly_mulmid` from
// `fmpz_poly/mulmid.c`, FLINT 3.6.0.
pub(crate) fn truncate_mul_middle_inputs<'a, C: PolynomialCoefficient>(
    xs: &'a [C],
    ys: &'a [C],
    mut nlo: usize,
    mut nhi: usize,
) -> (&'a [C], &'a [C], usize, usize) {
    // Low truncation of inputs
    let mut xs = &xs[..min(xs.len(), nhi)];
    let mut ys = &ys[..min(ys.len(), nhi)];
    // High truncation of inputs
    let nlo2 = xs.len() + ys.len() - 1 - nlo;
    if xs.len() > nlo2 {
        let trunc = xs.len() - nlo2;
        xs = &xs[trunc..];
        nlo -= trunc;
        nhi -= trunc;
    }
    if ys.len() > nlo2 {
        let trunc = ys.len() - nlo2;
        ys = &ys[trunc..];
        nlo -= trunc;
        nhi -= trunc;
    }
    (xs, ys, nlo, nhi)
}

// Sets `out` to the coefficients of $x^i$ for `nlo` $\leq i <$ `nhi` of the product of the
// polynomials with coefficients `xs` and `ys`, both nonempty. `nlo < nhi <= xs.len() + ys.len() -
// 1` must hold, and `out` must have length `nhi - nlo`.
//
// # Worst-case complexity
// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n) \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())`, and $m$ is the
// largest number of significant bits of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, except that it
// chooses Schönhage–Strassen in a measured window (see `schonhage_strassen_preferred`) rather
// than FLINT's.
crate_test_fn! {mul_middle_to_out<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
    ys: &[C],
    nlo: usize,
    nhi: usize,
) {
    assert_ne!(xs.len(), 0);
    assert_ne!(ys.len(), 0);
    assert!(nlo < nhi);
    assert!(nhi < xs.len() + ys.len());
    let (mut xs, mut ys, nlo, nhi) = truncate_mul_middle_inputs(xs, ys, nlo, nhi);
    let len = nhi - nlo;
    if xs.len() < ys.len() {
        swap(&mut xs, &mut ys);
    }
    if ys.len() == 1 {
        C::vec_mul_scalar_to_out(out, &xs[nlo..nhi], &ys[0]);
        return;
    }
    let bits1 = vec_max_bits(xs).0;
    let bits2 = if ptr::eq(xs, ys) { bits1 } else { vec_max_bits(ys).0 };
    let len1 = u64::exact_from(xs.len());
    let len2 = u64::exact_from(ys.len());
    if fft_preferred(len2, bits1, bits2, 100, 200) && mul_middle_to_out_fft(out, xs, ys, nlo, nhi) {
        return;
    }
    let len = u64::exact_from(len);
    let short_enough = len2 < 50 || (len2 << 2 >= 3 * len && len < 150 + bits1 + bits2);
    match tiny_kernel(bits1, bits2, len2, short_enough) {
        Some(TinyKernel::OneWord) => mul_middle_to_out_tiny_1(out, xs, ys, nlo, nhi),
        Some(TinyKernel::TwoWord) => mul_middle_to_out_tiny_2(out, xs, ys, nlo, nhi),
        None if nhi <= 8 || classical_preferred(len2, bits1, bits2) || len <= 3 => {
            mul_middle_to_out_classical(out, xs, ys, nlo, nhi);
        }
        None if nlo == 0 && karatsuba_preferred(len2, bits1, bits2) => {
            mul_truncated_to_out_karatsuba(out, xs, ys);
        }
        None if schonhage_strassen_preferred(len1, len2, bits1, bits2, 3071) => {
            mul_middle_to_out_schonhage_strassen(out, xs, ys, nlo, nhi);
        }
        None => mul_middle_to_out_kronecker(out, xs, ys, nlo, nhi),
    }
}}

// The coefficients of $x^i$ for `low` $\leq i <$ `high` of the product of the polynomials with
// coefficients `xs` and `ys`, shifted down by `low`, without zeros at the end. A range that extends
// past the end of the product is cut off there.
//
// This is equivalent to `fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, except that it
// panics if `low > high`, where FLINT returns zero.
pub(crate) fn mul_middle_ref_ref<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    low: u64,
    high: u64,
) -> Vec<C> {
    assert!(low <= high, "low must be at most high");
    if xs.is_empty() || ys.is_empty() {
        return Vec::new();
    }
    let high = usize::try_from(high)
        .unwrap_or(usize::MAX)
        .min(xs.len() + ys.len() - 1);
    let low = usize::try_from(low).unwrap_or(usize::MAX);
    if low >= high {
        return Vec::new();
    }
    let mut out = vec![C::ZERO; high - low];
    mul_middle_to_out(&mut out, xs, ys, low, high);
    trim_coefficients(&mut out);
    out
}

// Multiplies the polynomial with coefficients `xs` by the one with coefficients `ys`, keeping only
// the coefficients of $x^i$ for `low` $\leq i <$ `high`, shifted down by `low`. When `ys` is a
// constant, the result is a scalar multiple of a window of `xs`, computed in place in its `Vec`.
pub(crate) fn mul_middle_val_ref<C: PolynomialCoefficient>(
    mut xs: Vec<C>,
    ys: &[C],
    low: u64,
    high: u64,
) -> Vec<C> {
    if let [c] = ys {
        assert!(low <= high, "low must be at most high");
        truncate_coefficients(&mut xs, high);
        let low = usize::try_from(low).unwrap_or(usize::MAX).min(xs.len());
        xs.drain(..low);
        C::vec_mul_scalar_assign(&mut xs, c);
        xs
    } else {
        mul_middle_ref_ref(&xs, ys, low, high)
    }
}

// Multiplies the polynomial with coefficients `xs` by the one with coefficients `ys`, keeping only
// the coefficients of $x^i$ for `low` $\leq i <$ `high`, shifted down by `low`. When either is a
// constant, the result is computed in place in the other's `Vec`.
pub(crate) fn mul_middle_val_val<C: PolynomialCoefficient>(
    xs: Vec<C>,
    ys: Vec<C>,
    low: u64,
    high: u64,
) -> Vec<C> {
    if xs.len() == 1 {
        mul_middle_val_ref(ys, &xs, low, high)
    } else {
        mul_middle_val_ref(xs, &ys, low, high)
    }
}

impl MulMiddle<Self> for IntegerPolynomial {
    type Output = Self;

    /// Multiplies two [`IntegerPolynomial`]s, keeping only the coefficients of $x^i$ for `low`
    /// $\leq i <$ `high` and shifting them down by `low`, taking both by value.
    ///
    /// $$
    /// f(p, q, \ell, h) = \left \lfloor \frac{pq \bmod x^h}{x^\ell} \right \rfloor.
    /// $$
    ///
    /// The range may extend past the end of the product, which cuts it off. The result is trimmed,
    /// so when the coefficient of $x^{h-1}$ is zero, the degree is lower than $h - \ell - 1$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Panics
    /// Panics if `low > high`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulMiddle;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // (x^2-3*x+2)*(2*x+5) = 2*x^3-x^2-11*x+10.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .mul_middle(IntegerPolynomial::from_str("2*x+5").unwrap(), 1, 3)
    ///         .to_string(),
    ///     "-x-11"
    /// );
    /// // The range extends past the end of the product, x^2-1.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x+1").unwrap())
    ///         .mul_middle(IntegerPolynomial::from_str("x-1").unwrap(), 1, 100)
    ///         .to_string(),
    ///     "x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, except that
    /// it panics if `low > high`, where FLINT returns zero.
    #[inline]
    fn mul_middle(self, other: Self, low: u64, high: u64) -> Self {
        Self {
            coefficients: mul_middle_val_val(self.coefficients, other.coefficients, low, high),
        }
    }
}

impl MulMiddle<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Multiplies two [`IntegerPolynomial`]s, keeping only the coefficients of $x^i$ for `low`
    /// $\leq i <$ `high` and shifting them down by `low`, taking the first by value and the second
    /// by reference.
    ///
    /// $$
    /// f(p, q, \ell, h) = \left \lfloor \frac{pq \bmod x^h}{x^\ell} \right \rfloor.
    /// $$
    ///
    /// The range may extend past the end of the product, which cuts it off. The result is trimmed,
    /// so when the coefficient of $x^{h-1}$ is zero, the degree is lower than $h - \ell - 1$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Panics
    /// Panics if `low > high`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulMiddle;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // (x^2-3*x+2)*(2*x+5) = 2*x^3-x^2-11*x+10.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .mul_middle(&IntegerPolynomial::from_str("2*x+5").unwrap(), 1, 3)
    ///         .to_string(),
    ///     "-x-11"
    /// );
    /// // The range extends past the end of the product, x^2-1.
    /// assert_eq!(
    ///     (IntegerPolynomial::from_str("x+1").unwrap())
    ///         .mul_middle(&IntegerPolynomial::from_str("x-1").unwrap(), 1, 100)
    ///         .to_string(),
    ///     "x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, except that
    /// it panics if `low > high`, where FLINT returns zero.
    #[inline]
    fn mul_middle(self, other: &Self, low: u64, high: u64) -> Self {
        Self {
            coefficients: mul_middle_val_ref(self.coefficients, &other.coefficients, low, high),
        }
    }
}

impl MulMiddle<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Multiplies two [`IntegerPolynomial`]s, keeping only the coefficients of $x^i$ for `low`
    /// $\leq i <$ `high` and shifting them down by `low`, taking the first by reference and the
    /// second by value.
    ///
    /// $$
    /// f(p, q, \ell, h) = \left \lfloor \frac{pq \bmod x^h}{x^\ell} \right \rfloor.
    /// $$
    ///
    /// The range may extend past the end of the product, which cuts it off. The result is trimmed,
    /// so when the coefficient of $x^{h-1}$ is zero, the degree is lower than $h - \ell - 1$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Panics
    /// Panics if `low > high`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulMiddle;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // (x^2-3*x+2)*(2*x+5) = 2*x^3-x^2-11*x+10.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .mul_middle(IntegerPolynomial::from_str("2*x+5").unwrap(), 1, 3)
    ///         .to_string(),
    ///     "-x-11"
    /// );
    /// // The range extends past the end of the product, x^2-1.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x+1").unwrap())
    ///         .mul_middle(IntegerPolynomial::from_str("x-1").unwrap(), 1, 100)
    ///         .to_string(),
    ///     "x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, except that
    /// it panics if `low > high`, where FLINT returns zero.
    #[inline]
    fn mul_middle(self, other: IntegerPolynomial, low: u64, high: u64) -> IntegerPolynomial {
        IntegerPolynomial {
            coefficients: mul_middle_val_ref(other.coefficients, &self.coefficients, low, high),
        }
    }
}

impl MulMiddle<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Multiplies two [`IntegerPolynomial`]s, keeping only the coefficients of $x^i$ for `low`
    /// $\leq i <$ `high` and shifting them down by `low`, taking both by reference.
    ///
    /// $$
    /// f(p, q, \ell, h) = \left \lfloor \frac{pq \bmod x^h}{x^\ell} \right \rfloor.
    /// $$
    ///
    /// The range may extend past the end of the product, which cuts it off. The result is trimmed,
    /// so when the coefficient of $x^{h-1}$ is zero, the degree is lower than $h - \ell - 1$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Panics
    /// Panics if `low > high`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulMiddle;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // (x^2-3*x+2)*(2*x+5) = 2*x^3-x^2-11*x+10.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .mul_middle(&IntegerPolynomial::from_str("2*x+5").unwrap(), 1, 3)
    ///         .to_string(),
    ///     "-x-11"
    /// );
    /// // The range extends past the end of the product, x^2-1.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x+1").unwrap())
    ///         .mul_middle(&IntegerPolynomial::from_str("x-1").unwrap(), 1, 100)
    ///         .to_string(),
    ///     "x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, except that
    /// it panics if `low > high`, where FLINT returns zero.
    #[inline]
    fn mul_middle(self, other: &IntegerPolynomial, low: u64, high: u64) -> IntegerPolynomial {
        IntegerPolynomial {
            coefficients: mul_middle_ref_ref(&self.coefficients, &other.coefficients, low, high),
        }
    }
}

impl MulMiddleAssign<Self> for IntegerPolynomial {
    /// Multiplies an [`IntegerPolynomial`] by another [`IntegerPolynomial`] in place, keeping only
    /// the coefficients of $x^i$ for `low` $\leq i <$ `high` and shifting them down by `low`,
    /// taking the right-hand side by value.
    ///
    /// $$
    /// p \gets \left \lfloor \frac{pq \bmod x^h}{x^\ell} \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Panics
    /// Panics if `low > high`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulMiddleAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p.mul_middle_assign(IntegerPolynomial::from_str("2*x+5").unwrap(), 1, 3);
    /// assert_eq!(p.to_string(), "-x-11");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, except that
    /// it panics if `low > high`, where FLINT returns zero.
    #[inline]
    fn mul_middle_assign(&mut self, other: Self, low: u64, high: u64) {
        self.coefficients =
            mul_middle_val_val(take(&mut self.coefficients), other.coefficients, low, high);
    }
}

impl MulMiddleAssign<&Self> for IntegerPolynomial {
    /// Multiplies an [`IntegerPolynomial`] by another [`IntegerPolynomial`] in place, keeping only
    /// the coefficients of $x^i$ for `low` $\leq i <$ `high` and shifting them down by `low`,
    /// taking the right-hand side by reference.
    ///
    /// $$
    /// p \gets \left \lfloor \frac{pq \bmod x^h}{x^\ell} \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is the largest number of significant bits of any coefficient of either polynomial.
    ///
    /// # Panics
    /// Panics if `low > high`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::MulMiddleAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p.mul_middle_assign(&IntegerPolynomial::from_str("2*x+5").unwrap(), 1, 3);
    /// assert_eq!(p.to_string(), "-x-11");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, except that
    /// it panics if `low > high`, where FLINT returns zero.
    #[inline]
    fn mul_middle_assign(&mut self, other: &Self, low: u64, high: u64) {
        self.coefficients =
            mul_middle_val_ref(take(&mut self.coefficients), &other.coefficients, low, high);
    }
}
