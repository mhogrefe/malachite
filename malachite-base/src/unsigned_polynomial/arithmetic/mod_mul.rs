// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::arithmetic::traits::{ModMul, ModMulAssign};
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_add::assert_reduced;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::from_coefficients_trimmed;
use crate::unsigned_vector::arithmetic::mod_add::mod_add_assign_slice;
use crate::unsigned_vector::arithmetic::mod_dot::{ModData, column_sum};
use crate::unsigned_vector::arithmetic::mod_sub::mod_sub_assign_slice;
use alloc::vec;

// Multiplication of polynomials whose coefficients are words reduced modulo a word $m$. As in
// FLINT's `_nmod_poly_mul_classical`, each coefficient of a product is accumulated exactly, in one,
// two, or three words, depending on how large the sum can get, and reduced once. The reduction uses
// multiplication modulo $m$ with precomputed data, so that no two-word division is needed.

// The length of the shorter factor at which Karatsuba multiplication overtakes classical
// multiplication. Measured on an Apple M-series machine, 2026-10, with 64-bit words: about 80 for
// moduli of 16 to 64 bits.
pub(crate) const MOD_MUL_KARATSUBA_THRESHOLD: usize = 80;

// The length at which Karatsuba squaring overtakes classical squaring. Classical squaring computes
// only half the products, so it stays ahead longer: measured as above, about 256.
pub(crate) const MOD_SQUARE_KARATSUBA_THRESHOLD: usize = 256;

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, reduced modulo $m$,
// by schoolbook multiplication, one coefficient at a time. `out` must have length `xs.len() +
// ys.len() - 1`.
pub(crate) fn mod_mul_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    d: &ModData<T>,
) {
    let n = xs.len();
    let m = ys.len();
    for (k, o) in out.iter_mut().enumerate() {
        let start = k.saturating_sub(m - 1);
        let stop = core::cmp::min(k, n - 1);
        let acc = column_sum(&xs[start..=stop], &ys[k - stop..=k - start], d);
        *o = d.reduce_sum(acc);
    }
}

// The scratch length needed by `mod_mul_karatsuba_balanced` for factors of length `n`.
pub(crate) const fn mod_karatsuba_scratch_len(mut n: usize, threshold: usize) -> usize {
    let mut len = 0;
    while n >= threshold {
        let c = n - (n >> 1);
        len += (c << 2) - 1;
        n = c;
    }
    len
}

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, which have the same
// nonzero length $n$, modulo $m$, by Karatsuba multiplication, falling back to schoolbook
// multiplication below the threshold. `out` must have length $2n - 1$.
fn mod_mul_karatsuba_balanced<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    d: &ModData<T>,
    scratch: &mut [T],
) {
    let n = xs.len();
    if n < MOD_MUL_KARATSUBA_THRESHOLD {
        mod_mul_classical(out, xs, ys, d);
        return;
    }
    // Write x = x_0 + x^h x_1 and y = y_0 + x^h y_1. Then xy = x_0 y_0 + x^h ((x_0 + x_1)(y_0 +
    // y_1) - x_0 y_0 - x_1 y_1) + x^{2h} x_1 y_1.
    let m = d.m;
    let h = n >> 1;
    let c = n - h;
    let two_h = h << 1;
    let (x0, x1) = xs.split_at(h);
    let (y0, y1) = ys.split_at(h);
    split_into_chunks_mut!(scratch, c, [x_sum, y_sum], scratch);
    let (middle, scratch) = scratch.split_at_mut((c << 1) - 1);
    let (low, high) = out.split_at_mut(two_h);
    mod_mul_karatsuba_balanced(&mut low[..two_h - 1], x0, y0, d, scratch);
    low[two_h - 1] = T::ZERO;
    mod_mul_karatsuba_balanced(high, x1, y1, d, scratch);
    x_sum.copy_from_slice(x1);
    mod_add_assign_slice(x_sum, x0, m);
    y_sum.copy_from_slice(y1);
    mod_add_assign_slice(y_sum, y0, m);
    mod_mul_karatsuba_balanced(middle, x_sum, y_sum, d, scratch);
    mod_sub_assign_slice(middle, &out[..two_h - 1], m);
    mod_sub_assign_slice(middle, &out[two_h..], m);
    mod_add_assign_slice(&mut out[h..], middle, m);
}

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, both nonempty,
// modulo $m$, by Karatsuba multiplication, falling back to schoolbook multiplication for short
// factors. When the factors' lengths differ, the longer is cut into pieces as long as the shorter,
// and the products of the pieces are added together.
pub(crate) fn mod_mul_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    d: &ModData<T>,
) {
    let (xs, ys) = if xs.len() >= ys.len() {
        (xs, ys)
    } else {
        (ys, xs)
    };
    let n = xs.len();
    let m = ys.len();
    if m < MOD_MUL_KARATSUBA_THRESHOLD {
        mod_mul_classical(out, xs, ys, d);
        return;
    }
    let mut scratch = vec![T::ZERO; mod_karatsuba_scratch_len(m, MOD_MUL_KARATSUBA_THRESHOLD)];
    if n == m {
        mod_mul_karatsuba_balanced(out, xs, ys, d, &mut scratch);
        return;
    }
    out.fill(T::ZERO);
    let mut product = vec![T::ZERO; (m << 1) - 1];
    for (k, piece) in xs.chunks(m).enumerate() {
        let product = &mut product[..piece.len() + m - 1];
        if piece.len() == m {
            mod_mul_karatsuba_balanced(product, piece, ys, d, &mut scratch);
        } else {
            mod_mul_karatsuba(product, ys, piece, d);
        }
        mod_add_assign_slice(&mut out[k * m..], product, d.m);
    }
}

fn assert_lengths<T>(out: &[T], xs: &[T], ys: &[T]) {
    assert!(!xs.is_empty());
    assert!(!ys.is_empty());
    assert_eq!(out.len(), xs.len() + ys.len() - 1);
}

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, both nonempty and
// reduced modulo `m`, modulo `m`, by schoolbook multiplication. `out` must have length `xs.len() +
// ys.len() - 1`.
crate_test_fn! {
#[allow(dead_code)]
mod_mul_to_out_classical<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], ys: &[T], m: T) {
    assert_lengths(out, xs, ys);
    mod_mul_classical(out, xs, ys, &ModData::new(m, xs.len().min(ys.len())));
}}

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, both nonempty and
// reduced modulo `m`, modulo `m`, by Karatsuba multiplication. `out` must have length `xs.len() +
// ys.len() - 1`.
crate_test_fn! {
#[allow(dead_code)]
mod_mul_to_out_karatsuba<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], ys: &[T], m: T) {
    assert_lengths(out, xs, ys);
    mod_mul_karatsuba(out, xs, ys, &ModData::new(m, xs.len().min(ys.len())));
}}

/// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, both nonempty and
/// reduced modulo `m`, modulo `m`. `out` must have length `xs.len() + ys.len() - 1`, and `m` must
/// be positive.
///
/// This is not part of the public API; it is public so that `malachite-nz` can multiply
/// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
pub fn mod_mul_to_out<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], ys: &[T], m: T) {
    assert_lengths(out, xs, ys);
    mod_mul_karatsuba(out, xs, ys, &ModData::new(m, xs.len().min(ys.len())));
}

// The product of the polynomials with coefficients `xs` and `ys`, both reduced modulo `m`, modulo
// `m`.
pub(crate) fn mod_mul_helper<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    m: T,
) -> UnsignedPolynomial<T> {
    if xs.is_empty() || ys.is_empty() {
        return UnsignedPolynomial::ZERO;
    }
    let mut out = vec![T::ZERO; xs.len() + ys.len() - 1];
    mod_mul_to_out(&mut out, xs, ys, m);
    from_coefficients_trimmed(out)
}

impl<T: PrimitiveUnsigned> ModMul<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $m$, taking both by value. The coefficients of
    /// both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = pq \bmod m.
    /// $$
    ///
    /// When $m$ is not prime, the leading coefficient of the product can vanish modulo $m$, and
    /// then the degree of the product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its coefficients modulo 7.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_mul(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 7)
    ///         .to_string(),
    ///     "2*x^3+4*x^2+5*x+3"
    /// );
    /// // The leading coefficient of the product, 6, vanishes modulo 6, so the degree drops.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("2*x+1")
    ///         .unwrap()
    ///         .mod_mul(UnsignedPolynomial::<u8>::from_str("3*x+1").unwrap(), 6)
    ///         .to_string(),
    ///     "5*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul(self, other: Self, m: T) -> Self {
        assert_reduced(&self, &other, m);
        mod_mul_helper(&self.coefficients, &other.coefficients, m)
    }
}

impl<T: PrimitiveUnsigned> ModMul<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $m$, taking the first by value and the second
    /// by reference. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = pq \bmod m.
    /// $$
    ///
    /// When $m$ is not prime, the leading coefficient of the product can vanish modulo $m$, and
    /// then the degree of the product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its coefficients modulo 7.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_mul(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 7)
    ///         .to_string(),
    ///     "2*x^3+4*x^2+5*x+3"
    /// );
    /// // The leading coefficient of the product, 6, vanishes modulo 6, so the degree drops.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("2*x+1")
    ///         .unwrap()
    ///         .mod_mul(&UnsignedPolynomial::<u8>::from_str("3*x+1").unwrap(), 6)
    ///         .to_string(),
    ///     "5*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul(self, other: &Self, m: T) -> Self {
        assert_reduced(&self, other, m);
        mod_mul_helper(&self.coefficients, &other.coefficients, m)
    }
}

impl<T: PrimitiveUnsigned> ModMul<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $m$, taking the first by reference and the
    /// second by value. The coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = pq \bmod m.
    /// $$
    ///
    /// When $m$ is not prime, the leading coefficient of the product can vanish modulo $m$, and
    /// then the degree of the product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its coefficients modulo 7.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 7)
    ///         .to_string(),
    ///     "2*x^3+4*x^2+5*x+3"
    /// );
    /// // The leading coefficient of the product, 6, vanishes modulo 6, so the degree drops.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap())
    ///         .mod_mul(UnsignedPolynomial::<u8>::from_str("3*x+1").unwrap(), 6)
    ///         .to_string(),
    ///     "5*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul(self, other: UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        assert_reduced(self, &other, m);
        mod_mul_helper(&self.coefficients, &other.coefficients, m)
    }
}

impl<T: PrimitiveUnsigned> ModMul<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $m$, taking both by reference. The
    /// coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, m) = pq \bmod m.
    /// $$
    ///
    /// When $m$ is not prime, the leading coefficient of the product can vanish modulo $m$, and
    /// then the degree of the product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMul;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its coefficients modulo 7.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_mul(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 7)
    ///         .to_string(),
    ///     "2*x^3+4*x^2+5*x+3"
    /// );
    /// // The leading coefficient of the product, 6, vanishes modulo 6, so the degree drops.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap())
    ///         .mod_mul(&UnsignedPolynomial::<u8>::from_str("3*x+1").unwrap(), 6)
    ///         .to_string(),
    ///     "5*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul(self, other: &UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        assert_reduced(self, other, m);
        mod_mul_helper(&self.coefficients, &other.coefficients, m)
    }
}

impl<T: PrimitiveUnsigned> ModMulAssign<Self, T> for UnsignedPolynomial<T> {
    /// Multiplies an [`UnsignedPolynomial`] by another [`UnsignedPolynomial`] modulo $m$ in place,
    /// taking the right-hand side by value. The coefficients of both must already be reduced modulo
    /// $m$.
    ///
    /// $$
    /// p \gets pq \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_mul_assign(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 7);
    /// assert_eq!(p.to_string(), "2*x^3+4*x^2+5*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul_assign(&mut self, other: Self, m: T) {
        assert_reduced(self, &other, m);
        *self = mod_mul_helper(&self.coefficients, &other.coefficients, m);
    }
}

impl<T: PrimitiveUnsigned> ModMulAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Multiplies an [`UnsignedPolynomial`] by another [`UnsignedPolynomial`] modulo $m$ in place,
    /// taking the right-hand side by reference. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// p \gets pq \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModMulAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_mul_assign(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 7);
    /// assert_eq!(p.to_string(), "2*x^3+4*x^2+5*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0.
    fn mod_mul_assign(&mut self, other: &Self, m: T) {
        assert_reduced(self, other, m);
        *self = mod_mul_helper(&self.coefficients, &other.coefficients, m);
    }
}
