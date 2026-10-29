// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
//      Copyright © 2011 William Hart
//
//      Copyright © 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2IsReduced;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{CheckedModPowerOf2DivRem, ModPowerOf2DivAssignRem, ModPowerOf2DivRem};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_div_truncated::*;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::{
    from_coefficients_trimmed, mask_coefficients,
};
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul_truncated::mul_truncated_wrapping;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) fn assert_reduced<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        q.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {q} has a coefficient >= 2^{pow}"
    );
}

// The quotient of the polynomials with coefficients `xs` and `ys`, correct modulo $2^k$, where $k$
// is `pow`, which is positive, but not reduced, where `ys` is nonempty and no longer than `xs`, and
// `inverse` is an inverse of the last coefficient of `ys` modulo $2^k$. The quotient has `xs.len()
// - ys.len() + 1` coefficients; reversed, it is the quotient of the reversals of the polynomials as
// power series, which `series` computes.
//
// This is equivalent to `_gr_poly_div_newton` from `gr_poly/div_newton.c`, FLINT 3.6.0, with the
// modulus $2^k$, in wrapping arithmetic, with `series` in place of `_gr_poly_div_series`.
crate_test_fn! {mod_power_of_2_quotient_with<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    pow: u64,
    inverse: T,
    series: fn(&[T], &[T], usize, u64, T) -> Vec<T>,
) -> Vec<T> {
    let q_len = xs.len() - ys.len() + 1;
    let xs_rev: Vec<T> = xs[xs.len() - q_len..].iter().rev().copied().collect();
    let ys_rev: Vec<T> = ys[ys.len().saturating_sub(q_len)..].iter().rev().copied().collect();
    let mut q = series(&xs_rev, &ys_rev, q_len, pow, inverse);
    q.reverse();
    q
}}

// The remainder of the polynomials with coefficients `xs` and `ys`, given their quotient `qs`,
// correct modulo $2^k$, where $k$ is `pow`, but not reduced: the `ys.len() - 1` coefficients of `xs
// - qs * ys` that are not cancelled.
//
// This is equivalent to the remainder step of `_gr_poly_divrem_newton` from
// `gr_poly/divrem_newton.c`, FLINT 3.6.0, with the modulus $2^k$, in wrapping arithmetic.
fn remainder<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], qs: &[T], pow: u64) -> Vec<T> {
    let r_len = ys.len() - 1;
    if r_len == 0 {
        return Vec::new();
    }
    let mut rs = vec![T::ZERO; r_len];
    mul_truncated_wrapping(&mut rs, qs, &ys[..r_len], pow);
    for (r, &x) in rs.iter_mut().zip(xs) {
        *r = x.wrapping_sub(*r);
    }
    rs
}

// The quotient and remainder of the polynomials with coefficients `xs` and `ys`, correct modulo
// $2^k$, where $k$ is `pow`, but not reduced, where `ys` is nonempty and no longer than `xs`, and
// `inverse` is an inverse of the last coefficient of `ys` modulo $2^k$, with `series` dividing the
// reversals as power series.
//
// This is equivalent to `_gr_poly_divrem_newton` from `gr_poly/divrem_newton.c`, FLINT 3.6.0, with
// the modulus $2^k$, in wrapping arithmetic, with `series` in place of `_gr_poly_div_series`.
crate_test_fn! {mod_power_of_2_div_rem_with_series<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    pow: u64,
    inverse: T,
    series: fn(&[T], &[T], usize, u64, T) -> Vec<T>,
) -> (Vec<T>, Vec<T>) {
    let qs = mod_power_of_2_quotient_with(xs, ys, pow, inverse, series);
    let rs = remainder(xs, ys, &qs, pow);
    (qs, rs)
}}

// An inverse modulo $2^k$, where $k$ is `pow`, of the leading coefficient of the polynomial with
// coefficients `ys`, or `None` if `ys` is empty or its leading coefficient is even.
pub(crate) fn leading_inverse<T: PrimitiveUnsigned>(ys: &[T], pow: u64) -> Option<T> {
    ys.last()?.mod_power_of_2_inverse(pow)
}

// The polynomial with the coefficients `xs`, reduced modulo $2^k$, where $k$ is `pow`, and trimmed.
pub(crate) fn reduced<T: PrimitiveUnsigned>(mut xs: Vec<T>, pow: u64) -> UnsignedPolynomial<T> {
    mask_coefficients(&mut xs, pow);
    from_coefficients_trimmed(xs)
}

// The quotient and remainder modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients
// `xs` and `ys`, reduced, with `series` dividing the reversals as power series in the general case,
// or `None` if `ys` is zero or its leading coefficient is even. Modulo 1, every polynomial is 0,
// and so is the result.
//
// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, with the modulus
// $2^k$, except that `None` is returned where FLINT aborts or assumes the leading coefficient
// invertible.
crate_test_fn! {mod_power_of_2_div_rem_with<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    pow: u64,
    series: fn(&[T], &[T], usize, u64, T) -> Vec<T>,
) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
    if pow == 0 {
        return Some((UnsignedPolynomial::ZERO, UnsignedPolynomial::ZERO));
    }
    let inverse = leading_inverse(ys, pow)?;
    if xs.len() < ys.len() {
        return Some((UnsignedPolynomial::ZERO, from_coefficients_trimmed(xs.to_vec())));
    }
    let (qs, rs) = mod_power_of_2_div_rem_with_series(xs, ys, pow, inverse, series);
    Some((reduced(qs, pow), reduced(rs, pow)))
}}

/// The coefficients of the quotient and remainder modulo $2^k$ of the polynomials with
/// coefficients `xs` and `ys`, without zeros at the end, or `None` if `ys` is empty or its last
/// coefficient is even. The coefficients must be reduced modulo $2^k$, where $k$ is `pow`, which
/// must be no greater than `T::WIDTH`, and neither `xs` nor `ys` may end with a zero.
///
/// This is not part of the public API; it is public so that `malachite-nz` can divide
/// `NaturalPolynomial`s modulo a power of 2.
#[doc(hidden)]
#[inline]
pub fn mod_power_of_2_div_rem_vec<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    pow: u64,
) -> Option<(Vec<T>, Vec<T>)> {
    mod_power_of_2_div_rem_with(xs, ys, pow, mod_power_of_2_div_truncated_dispatch)
        .map(|(q, r)| (q.coefficients, r.coefficients))
}

fn checked_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
    assert_reduced(p, q, pow);
    mod_power_of_2_div_rem_with(
        &p.coefficients,
        &q.coefficients,
        pow,
        mod_power_of_2_div_truncated_dispatch,
    )
}

fn helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> (UnsignedPolynomial<T>, UnsignedPolynomial<T>) {
    assert!(
        pow == 0 || *q != UnsignedPolynomial::ZERO,
        "division by zero"
    );
    checked_helper(p, q, pow).expect("the leading coefficient of the divisor must be odd")
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivRem<Self> for UnsignedPolynomial<T> {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the quotient and remainder,
    /// taking the dividend by value and the divisor by value. The coefficients of both must already
    /// be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem(q, 3);
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_rem(self, other: Self, pow: u64) -> (Self, Self) {
        helper(&self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivRem<&Self> for UnsignedPolynomial<T> {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the quotient and remainder,
    /// taking the dividend by value and the divisor by reference. The coefficients of both must
    /// already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem(&q, 3);
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_rem(self, other: &Self, pow: u64) -> (Self, Self) {
        helper(&self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivRem<UnsignedPolynomial<T>> for &UnsignedPolynomial<T> {
    type DivOutput = UnsignedPolynomial<T>;
    type RemOutput = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the quotient and remainder,
    /// taking the dividend by reference and the divisor by value. The coefficients of both must
    /// already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem(q, 3);
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_rem(
        self,
        other: UnsignedPolynomial<T>,
        pow: u64,
    ) -> (UnsignedPolynomial<T>, UnsignedPolynomial<T>) {
        helper(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivRem<&UnsignedPolynomial<T>> for &UnsignedPolynomial<T> {
    type DivOutput = UnsignedPolynomial<T>;
    type RemOutput = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the quotient and remainder,
    /// taking the dividend by reference and the divisor by reference. The coefficients of both must
    /// already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem(&q, 3);
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_rem(
        self,
        other: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> (UnsignedPolynomial<T>, UnsignedPolynomial<T>) {
        helper(self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivAssignRem<Self> for UnsignedPolynomial<T> {
    type RemOutput = Self;

    /// Divides an [`UnsignedPolynomial`] by another modulo $2^k$ in place, replacing it with the
    /// quotient and returning the remainder, taking the divisor by value. The coefficients of both
    /// must already be reduced modulo $2^k$, and the leading coefficient of the divisor must be
    /// odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivAssignRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// let r = p.mod_power_of_2_div_assign_rem(q, 3);
    /// assert_eq!(p.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_assign_rem(&mut self, other: Self, pow: u64) -> Self {
        let (q, r) = helper(self, &other, pow);
        *self = q;
        r
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2DivAssignRem<&Self> for UnsignedPolynomial<T> {
    type RemOutput = Self;

    /// Divides an [`UnsignedPolynomial`] by another modulo $2^k$ in place, replacing it with the
    /// quotient and returning the remainder, taking the divisor by reference. The coefficients of
    /// both must already be reduced modulo $2^k$, and the leading coefficient of the divisor must
    /// be odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` is 0 (unless $k$ is 0) or its leading
    /// coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivAssignRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// let r = p.mod_power_of_2_div_assign_rem(&q, 3);
    /// assert_eq!(p.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_assign_rem(&mut self, other: &Self, pow: u64) -> Self {
        let (q, r) = helper(self, other, pow);
        *self = q;
        r
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2DivRem<Self> for UnsignedPolynomial<T> {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not odd, taking the dividend by value and the
    /// divisor by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not odd; otherwise the
    /// quotient and remainder are unique. When $k$ is 0, every polynomial is 0, and so is the
    /// result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// let (s, r) = p.checked_mod_power_of_2_div_rem(q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_power_of_2_div_rem(q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that `None` is returned where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn checked_mod_power_of_2_div_rem(self, other: Self, pow: u64) -> Option<(Self, Self)> {
        checked_helper(&self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2DivRem<&Self> for UnsignedPolynomial<T> {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not odd, taking the dividend by value and the
    /// divisor by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not odd; otherwise the
    /// quotient and remainder are unique. When $k$ is 0, every polynomial is 0, and so is the
    /// result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// let (s, r) = p.checked_mod_power_of_2_div_rem(&q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_power_of_2_div_rem(&q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that `None` is returned where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn checked_mod_power_of_2_div_rem(self, other: &Self, pow: u64) -> Option<(Self, Self)> {
        checked_helper(&self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2DivRem<UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type DivOutput = UnsignedPolynomial<T>;
    type RemOutput = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not odd, taking the dividend by reference and
    /// the divisor by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not odd; otherwise the
    /// quotient and remainder are unique. When $k$ is 0, every polynomial is 0, and so is the
    /// result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// let (s, r) = (&p).checked_mod_power_of_2_div_rem(q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_power_of_2_div_rem(q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that `None` is returned where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn checked_mod_power_of_2_div_rem(
        self,
        other: UnsignedPolynomial<T>,
        pow: u64,
    ) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
        checked_helper(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> CheckedModPowerOf2DivRem<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type DivOutput = UnsignedPolynomial<T>;
    type RemOutput = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $2^k$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not odd, taking the dividend by reference and
    /// the divisor by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not odd; otherwise the
    /// quotient and remainder are unique. When $k$ is 0, every polynomial is 0, and so is the
    /// result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^4+7*x+5").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("5*x^2+2").unwrap();
    /// let (s, r) = (&p).checked_mod_power_of_2_div_rem(&q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+3").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_power_of_2_div_rem(&q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, with the
    /// modulus $2^k$, except that `None` is returned where FLINT aborts or assumes the leading
    /// coefficient invertible.
    #[inline]
    fn checked_mod_power_of_2_div_rem(
        self,
        other: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
        checked_helper(self, other, pow)
    }
}
