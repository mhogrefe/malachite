// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModMulTruncated, ModMulTruncatedAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_add::assert_reduced;
use crate::unsigned_polynomial::arithmetic::mod_mul::{
    MOD_MUL_KARATSUBA_THRESHOLD, ModData, column_sum, mod_add_assign_slice, mod_mul_karatsuba,
};
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::from_coefficients_trimmed;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul_truncated::truncated_len;
use alloc::vec;
use core::cmp::min;

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, both nonempty, reduced modulo $m$, by schoolbook multiplication, one
// coefficient at a time.
pub(crate) fn mod_mul_truncated_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    d: &ModData<T>,
) {
    let n = xs.len();
    let m = ys.len();
    for (k, o) in out.iter_mut().enumerate() {
        if k > n + m - 2 {
            *o = T::ZERO;
            continue;
        }
        let start = k.saturating_sub(m - 1);
        let stop = min(k, n - 1);
        let acc = column_sum(&xs[start..=stop], &ys[k - stop..=k - start], d);
        *o = d.reduce_sum(acc);
    }
}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, both nonempty, modulo $m$. With $n$ equal to `out.len()` and $h =
// \lceil n/2 \rceil$, write x = x_0 + x^h x_1 and y = y_0 + x^h y_1; since $2h \geq n$, xy mod x^n
// is x_0 y_0 + x^h (x_1 y_0 + x_0 y_1) mod x^n. The first product is a full product of half-length
// factors, computed by Karatsuba multiplication, and the other two are truncated products of half
// the length, computed recursively.
pub(crate) fn mod_mul_truncated_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    d: &ModData<T>,
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
        mod_mul_karatsuba(&mut out[..full_len], xs, ys, d);
        out[full_len..].fill(T::ZERO);
        return;
    }
    if ys.len() < MOD_MUL_KARATSUBA_THRESHOLD {
        mod_mul_truncated_classical(out, xs, ys, d);
        return;
    }
    let h = len.div_ceil(2);
    let x0 = &xs[..min(h, xs.len())];
    let y0 = &ys[..min(h, ys.len())];
    // The product of x_0 and y_0 has at most 2h - 1 <= `len` coefficients, so this is a full
    // product.
    mod_mul_truncated_karatsuba(out, x0, y0, d);
    let mut cross = vec![T::ZERO; len - h];
    if xs.len() > h {
        mod_mul_truncated_karatsuba(&mut cross, &xs[h..], y0, d);
        mod_add_assign_slice(&mut out[h..], &cross, d.m);
    }
    if ys.len() > h {
        mod_mul_truncated_karatsuba(&mut cross, x0, &ys[h..], d);
        mod_add_assign_slice(&mut out[h..], &cross, d.m);
    }
}

fn assert_lengths<T>(out: &[T], xs: &[T], ys: &[T]) {
    assert!(!out.is_empty());
    assert!(!xs.is_empty());
    assert!(!ys.is_empty());
}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, all three nonempty and the inputs reduced modulo `m`, modulo `m`, by
// schoolbook multiplication.
crate_test_fn! {
#[allow(dead_code)]
mod_mul_truncated_to_out_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    m: T,
) {
    assert_lengths(out, xs, ys);
    let terms = xs.len().min(ys.len()).min(out.len());
    mod_mul_truncated_classical(out, xs, ys, &ModData::new(m, terms));
}}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, all three nonempty and the inputs reduced modulo `m`, modulo `m`, by
// Karatsuba multiplication.
crate_test_fn! {
#[allow(dead_code)]
mod_mul_truncated_to_out_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    m: T,
) {
    assert_lengths(out, xs, ys);
    let terms = xs.len().min(ys.len()).min(out.len());
    mod_mul_truncated_karatsuba(out, xs, ys, &ModData::new(m, terms));
}}

/// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
/// coefficients `xs` and `ys`, all three nonempty and the inputs reduced modulo `m`, modulo `m`.
/// `m` must be positive.
///
/// This is not part of the public API; it is public so that `malachite-nz` can multiply
/// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
pub fn mod_mul_truncated_to_out<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], ys: &[T], m: T) {
    assert_lengths(out, xs, ys);
    mod_mul_truncated_karatsuba(
        out,
        xs,
        ys,
        &ModData::new(m, xs.len().min(ys.len()).min(out.len())),
    );
}

// The product of the polynomials with coefficients `xs` and `ys`, both reduced modulo `m`,
// truncated to `len` coefficients and reduced modulo `m`.
fn mod_mul_truncated_helper<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    len: u64,
    m: T,
) -> UnsignedPolynomial<T> {
    if len == 0 || xs.is_empty() || ys.is_empty() {
        return UnsignedPolynomial::ZERO;
    }
    let mut out = vec![T::ZERO; truncated_len(xs.len(), ys.len(), len)];
    mod_mul_truncated_to_out(&mut out, xs, ys, m);
    from_coefficients_trimmed(out)
}

impl<T: PrimitiveUnsigned> ModMulTruncated<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $m$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking both by value. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (pq \bmod x^n) \bmod m.
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 7.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_mul_truncated(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 2, 7)
    ///         .to_string(),
    ///     "5*x+3"
    /// );
    /// // The linear coefficient of the product, 7, vanishes modulo 7.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x+6")
    ///         .unwrap()
    ///         .mod_mul_truncated(UnsignedPolynomial::<u8>::from_str("x+1").unwrap(), 2, 7)
    ///         .to_string(),
    ///     "6"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated(self, other: Self, len: u64, m: T) -> Self {
        assert_reduced(&self, &other, m);
        mod_mul_truncated_helper(&self.coefficients, &other.coefficients, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulTruncated<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $m$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the first by value and the second by reference. The
    /// coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (pq \bmod x^n) \bmod m.
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 7.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_mul_truncated(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 2, 7)
    ///         .to_string(),
    ///     "5*x+3"
    /// );
    /// // The linear coefficient of the product, 7, vanishes modulo 7.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x+6")
    ///         .unwrap()
    ///         .mod_mul_truncated(&UnsignedPolynomial::<u8>::from_str("x+1").unwrap(), 2, 7)
    ///         .to_string(),
    ///     "6"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated(self, other: &Self, len: u64, m: T) -> Self {
        assert_reduced(&self, other, m);
        mod_mul_truncated_helper(&self.coefficients, &other.coefficients, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulTruncated<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $m$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the first by reference and the second by value. The
    /// coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (pq \bmod x^n) \bmod m.
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 7.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul_truncated(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 2, 7)
    ///         .to_string(),
    ///     "5*x+3"
    /// );
    /// // The linear coefficient of the product, 7, vanishes modulo 7.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x+6").unwrap())
    ///         .mod_mul_truncated(UnsignedPolynomial::<u8>::from_str("x+1").unwrap(), 2, 7)
    ///         .to_string(),
    ///     "6"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated(
        self,
        other: UnsignedPolynomial<T>,
        len: u64,
        m: T,
    ) -> UnsignedPolynomial<T> {
        assert_reduced(self, &other, m);
        mod_mul_truncated_helper(&self.coefficients, &other.coefficients, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulTruncated<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $m$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking both by reference. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (pq \bmod x^n) \bmod m.
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 7.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul_truncated(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 2, 7)
    ///         .to_string(),
    ///     "5*x+3"
    /// );
    /// // The linear coefficient of the product, 7, vanishes modulo 7.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x+6").unwrap())
    ///         .mod_mul_truncated(&UnsignedPolynomial::<u8>::from_str("x+1").unwrap(), 2, 7)
    ///         .to_string(),
    ///     "6"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated(
        self,
        other: &UnsignedPolynomial<T>,
        len: u64,
        m: T,
    ) -> UnsignedPolynomial<T> {
        assert_reduced(self, other, m);
        mod_mul_truncated_helper(&self.coefficients, &other.coefficients, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulTruncatedAssign<Self, T> for UnsignedPolynomial<T> {
    /// Multiplies an [`UnsignedPolynomial`] by another [`UnsignedPolynomial`] modulo $m$ in place,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side
    /// by value. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// p \gets (pq \bmod x^n) \bmod m.
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_mul_truncated_assign(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 2, 7);
    /// assert_eq!(p.to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated_assign(&mut self, other: Self, len: u64, m: T) {
        assert_reduced(self, &other, m);
        *self = mod_mul_truncated_helper(&self.coefficients, &other.coefficients, len, m);
    }
}

impl<T: PrimitiveUnsigned> ModMulTruncatedAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Multiplies an [`UnsignedPolynomial`] by another [`UnsignedPolynomial`] modulo $m$ in place,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side
    /// by reference. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// p \gets (pq \bmod x^n) \bmod m.
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
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModMulTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_mul_truncated_assign(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 2, 7);
    /// assert_eq!(p.to_string(), "5*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0.
    fn mod_mul_truncated_assign(&mut self, other: &Self, len: u64, m: T) {
        assert_reduced(self, other, m);
        *self = mod_mul_truncated_helper(&self.coefficients, &other.coefficients, len, m);
    }
}
