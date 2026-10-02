// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModPowerOf2MulTruncated, ModPowerOf2MulTruncatedAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_add::assert_reduced;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::{
    MOD_POWER_OF_2_MUL_KARATSUBA_THRESHOLD, add_wrapping_assign, from_coefficients_trimmed,
    mask_coefficients, mul_karatsuba_wrapping,
};
use alloc::vec;
use core::cmp::min;

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, modulo $2^\text{W}$, by schoolbook multiplication.
pub(crate) fn mul_truncated_classical_wrapping<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
) {
    let len = out.len();
    out.fill(T::ZERO);
    for (i, &x) in xs.iter().take(len).enumerate() {
        if x != T::ZERO {
            for (o, &y) in out[i..].iter_mut().zip(ys) {
                o.wrapping_add_assign(x.wrapping_mul(y));
            }
        }
    }
}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, both nonempty, modulo $2^\text{W}$. With $n$ equal to `out.len()` and
// $h = \lceil n/2 \rceil$, write x = x_0 + x^h x_1 and y = y_0 + x^h y_1; since $2h \geq n$, xy mod
// x^n is x_0 y_0 + x^h (x_1 y_0 + x_0 y_1) mod x^n. The first product is a full product of
// half-length factors, computed by Karatsuba multiplication, and the other two are truncated
// products of half the length, computed recursively.
pub(crate) fn mul_truncated_karatsuba_wrapping<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
) {
    let len = out.len();
    let xs = &xs[..min(xs.len(), len)];
    let ys = &ys[..min(ys.len(), len)];
    let (xs, ys) = if xs.len() >= ys.len() {
        (xs, ys)
    } else {
        (ys, xs)
    };
    let full_len = xs.len() + ys.len() - 1;
    if full_len <= len {
        mul_karatsuba_wrapping(&mut out[..full_len], xs, ys);
        out[full_len..].fill(T::ZERO);
        return;
    }
    if ys.len() < MOD_POWER_OF_2_MUL_KARATSUBA_THRESHOLD {
        mul_truncated_classical_wrapping(out, xs, ys);
        return;
    }
    let h = len.div_ceil(2);
    let x0 = &xs[..min(h, xs.len())];
    let y0 = &ys[..min(h, ys.len())];
    // The product of x_0 and y_0 has at most 2h - 1 <= `len` coefficients, so this is a full
    // product.
    mul_truncated_karatsuba_wrapping(out, x0, y0);
    let mut cross = vec![T::ZERO; len - h];
    if xs.len() > h {
        mul_truncated_karatsuba_wrapping(&mut cross, &xs[h..], y0);
        add_wrapping_assign(&mut out[h..], &cross);
    }
    if ys.len() > h {
        mul_truncated_karatsuba_wrapping(&mut cross, x0, &ys[h..]);
        add_wrapping_assign(&mut out[h..], &cross);
    }
}

fn assert_lengths<T>(out: &[T], xs: &[T], ys: &[T]) {
    assert!(!out.is_empty());
    assert!(!xs.is_empty());
    assert!(!ys.is_empty());
}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, all three nonempty and the inputs reduced modulo $2^k$, where $k$ is
// `pow`, by schoolbook multiplication. `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_mul_truncated_to_out_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    pow: u64,
) {
    assert_lengths(out, xs, ys);
    assert!(pow <= T::WIDTH);
    mul_truncated_classical_wrapping(out, xs, ys);
    mask_coefficients(out, pow);
}}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, all three nonempty and the inputs reduced modulo $2^k$, where $k$ is
// `pow`, by Karatsuba multiplication. `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_mul_truncated_to_out_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    pow: u64,
) {
    assert_lengths(out, xs, ys);
    assert!(pow <= T::WIDTH);
    mul_truncated_karatsuba_wrapping(out, xs, ys);
    mask_coefficients(out, pow);
}}

/// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
/// coefficients `xs` and `ys`, all three nonempty and the inputs reduced modulo $2^k$, where $k$ is
/// `pow`. `pow` must be no greater than `T::WIDTH`.
///
/// This is not part of the public API; it is public so that `malachite-nz` can multiply
/// `NaturalPolynomial`s with word-sized coefficients modulo $2^k$.
#[doc(hidden)]
pub fn mod_power_of_2_mul_truncated_to_out<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    pow: u64,
) {
    assert_lengths(out, xs, ys);
    assert!(pow <= T::WIDTH);
    mul_truncated_karatsuba_wrapping(out, xs, ys);
    mask_coefficients(out, pow);
}

// The number of coefficients of a truncated product worth computing: `len`, but no more than the
// whole product of factors of lengths `len1` and `len2`, which must be positive.
pub(crate) fn truncated_len(len1: usize, len2: usize, len: u64) -> usize {
    min(usize::try_from(len).unwrap_or(usize::MAX), len1 + len2 - 1)
}

// The product of the polynomials with coefficients `xs` and `ys`, both reduced modulo $2^k$, where
// $k$ is `pow`, truncated to `len` coefficients and reduced modulo $2^k$.
fn mod_power_of_2_mul_truncated_helper<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    len: u64,
    pow: u64,
) -> UnsignedPolynomial<T> {
    if len == 0 || xs.is_empty() || ys.is_empty() {
        return UnsignedPolynomial::ZERO;
    }
    let mut out = vec![T::ZERO; truncated_len(xs.len(), ys.len(), len)];
    mod_power_of_2_mul_truncated_to_out(&mut out, xs, ys, pow);
    from_coefficients_trimmed(out)
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulTruncated<Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking both by value. The coefficients of both must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (pq \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 16.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(),
    ///             2,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "3*x+10"
    /// );
    /// // The linear coefficient of the product, 16, vanishes modulo 16.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x+15")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x+1").unwrap(),
    ///             2,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "15"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul_truncated(self, other: Self, len: u64, pow: u64) -> Self {
        assert_reduced(&self, &other, pow);
        mod_power_of_2_mul_truncated_helper(&self.coefficients, &other.coefficients, len, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulTruncated<&Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the first by value and the second by reference. The
    /// coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (pq \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 16.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(),
    ///             2,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "3*x+10"
    /// );
    /// // The linear coefficient of the product, 16, vanishes modulo 16.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x+15")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x+1").unwrap(),
    ///             2,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "15"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul_truncated(self, other: &Self, len: u64, pow: u64) -> Self {
        assert_reduced(&self, other, pow);
        mod_power_of_2_mul_truncated_helper(&self.coefficients, &other.coefficients, len, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulTruncated<UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the first by reference and the second by value. The
    /// coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (pq \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 16.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_mul_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(),
    ///             2,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "3*x+10"
    /// );
    /// // The linear coefficient of the product, 16, vanishes modulo 16.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x+15").unwrap())
    ///         .mod_power_of_2_mul_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x+1").unwrap(),
    ///             2,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "15"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul_truncated(
        self,
        other: UnsignedPolynomial<T>,
        len: u64,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        assert_reduced(self, &other, pow);
        mod_power_of_2_mul_truncated_helper(&self.coefficients, &other.coefficients, len, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulTruncated<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking both by reference. The coefficients of both must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (pq \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 16.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_mul_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(),
    ///             2,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "3*x+10"
    /// );
    /// // The linear coefficient of the product, 16, vanishes modulo 16.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x+15").unwrap())
    ///         .mod_power_of_2_mul_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x+1").unwrap(),
    ///             2,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "15"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul_truncated(
        self,
        other: &UnsignedPolynomial<T>,
        len: u64,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        assert_reduced(self, other, pow);
        mod_power_of_2_mul_truncated_helper(&self.coefficients, &other.coefficients, len, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulTruncatedAssign<Self> for UnsignedPolynomial<T> {
    /// Multiplies an [`UnsignedPolynomial`] by another [`UnsignedPolynomial`] modulo $2^k$ in
    /// place, keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand
    /// side by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets (pq \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_mul_truncated_assign(
    ///     UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(),
    ///     2,
    ///     4,
    /// );
    /// assert_eq!(p.to_string(), "3*x+10");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul_truncated_assign(&mut self, other: Self, len: u64, pow: u64) {
        assert_reduced(self, &other, pow);
        *self =
            mod_power_of_2_mul_truncated_helper(&self.coefficients, &other.coefficients, len, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulTruncatedAssign<&Self> for UnsignedPolynomial<T> {
    /// Multiplies an [`UnsignedPolynomial`] by another [`UnsignedPolynomial`] modulo $2^k$ in
    /// place, keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand
    /// side by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets (pq \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_mul_truncated_assign(
    ///     &UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(),
    ///     2,
    ///     4,
    /// );
    /// assert_eq!(p.to_string(), "3*x+10");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul_truncated_assign(&mut self, other: &Self, len: u64, pow: u64) {
        assert_reduced(self, other, pow);
        *self =
            mod_power_of_2_mul_truncated_helper(&self.coefficients, &other.coefficients, len, pow);
    }
}
