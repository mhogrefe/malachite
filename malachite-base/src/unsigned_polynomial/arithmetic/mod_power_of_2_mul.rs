// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::arithmetic::traits::{ModPowerOf2Mul, ModPowerOf2MulAssign};
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_add::assert_reduced;
use alloc::vec;
use alloc::vec::Vec;

// Multiplication of polynomials whose coefficients are words reduced modulo $2^k$, for $k$ no
// greater than the word width W. Arithmetic modulo $2^\text{W}$ is wrapping arithmetic, and $2^k$
// divides $2^\text{W}$, so the kernels compute with wrapping arithmetic throughout and reduce each
// coefficient of the result once, at the end.

// The length of the shorter factor at which Karatsuba multiplication overtakes classical
// multiplication. Measured on an Apple M-series machine, 2026-10, for 64-bit words: Karatsuba
// breaks even at 32 and wins from 40.
pub(crate) const MOD_POWER_OF_2_MUL_KARATSUBA_THRESHOLD: usize = 32;

// The length at which Karatsuba squaring overtakes classical squaring, which computes only half the
// products and so stays ahead longer. Measured as above: Karatsuba loses at 64 and wins from 80.
pub(crate) const MOD_POWER_OF_2_SQUARE_KARATSUBA_THRESHOLD: usize = 64;

// Reduces each element of `xs` modulo $2^k$, where $k$ is `pow`.
pub(crate) fn mask_coefficients<T: PrimitiveUnsigned>(xs: &mut [T], pow: u64) {
    if pow < T::WIDTH {
        let mask = T::low_mask(pow);
        for x in xs {
            *x &= mask;
        }
    }
}

// Adds each element of `ys` to the element of `xs` at the same index, wrapping.
pub(crate) fn add_wrapping_assign<T: PrimitiveUnsigned>(xs: &mut [T], ys: &[T]) {
    for (x, &y) in xs.iter_mut().zip(ys) {
        x.wrapping_add_assign(y);
    }
}

// Subtracts each element of `ys` from the element of `xs` at the same index, wrapping.
pub(crate) fn sub_wrapping_assign<T: PrimitiveUnsigned>(xs: &mut [T], ys: &[T]) {
    for (x, &y) in xs.iter_mut().zip(ys) {
        x.wrapping_sub_assign(y);
    }
}

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, modulo
// $2^\text{W}$, by schoolbook multiplication. `out` must have length `xs.len() + ys.len() - 1`.
pub(crate) fn mul_classical_wrapping<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], ys: &[T]) {
    out.fill(T::ZERO);
    for (i, &x) in xs.iter().enumerate() {
        if x != T::ZERO {
            for (o, &y) in out[i..].iter_mut().zip(ys) {
                o.wrapping_add_assign(x.wrapping_mul(y));
            }
        }
    }
}

// The scratch length needed by `mul_karatsuba_balanced_wrapping` for factors of length `n`: at each
// level of the recursion, two sums and their product, each about half as long.
pub(crate) const fn karatsuba_wrapping_scratch_len(mut n: usize) -> usize {
    let mut len = 0;
    while n >= MOD_POWER_OF_2_MUL_KARATSUBA_THRESHOLD {
        let c = n - (n >> 1);
        len += (c << 2) - 1;
        n = c;
    }
    len
}

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, which have the same
// nonzero length $n$, modulo $2^\text{W}$, by Karatsuba multiplication, falling back to schoolbook
// multiplication below the threshold. `out` must have length $2n - 1$, and `scratch` at least
// `karatsuba_wrapping_scratch_len(n)`.
fn mul_karatsuba_balanced_wrapping<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    scratch: &mut [T],
) {
    let n = xs.len();
    if n < MOD_POWER_OF_2_MUL_KARATSUBA_THRESHOLD {
        mul_classical_wrapping(out, xs, ys);
        return;
    }
    // Write x = x_0 + x^h x_1 and y = y_0 + x^h y_1. Then xy = x_0 y_0 + x^h ((x_0 + x_1)(y_0 +
    // y_1) - x_0 y_0 - x_1 y_1) + x^{2h} x_1 y_1.
    let h = n >> 1;
    let c = n - h;
    let two_h = h << 1;
    let (x0, x1) = xs.split_at(h);
    let (y0, y1) = ys.split_at(h);
    split_into_chunks_mut!(scratch, c, [x_sum, y_sum], scratch);
    let (middle, scratch) = scratch.split_at_mut((c << 1) - 1);
    let (low, high) = out.split_at_mut(two_h);
    mul_karatsuba_balanced_wrapping(&mut low[..two_h - 1], x0, y0, scratch);
    low[two_h - 1] = T::ZERO;
    mul_karatsuba_balanced_wrapping(high, x1, y1, scratch);
    x_sum.copy_from_slice(x1);
    add_wrapping_assign(x_sum, x0);
    y_sum.copy_from_slice(y1);
    add_wrapping_assign(y_sum, y0);
    mul_karatsuba_balanced_wrapping(middle, x_sum, y_sum, scratch);
    sub_wrapping_assign(middle, &out[..two_h - 1]);
    sub_wrapping_assign(middle, &out[two_h..]);
    add_wrapping_assign(&mut out[h..], middle);
}

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, both nonempty,
// modulo $2^\text{W}$, by Karatsuba multiplication, falling back to schoolbook multiplication for
// short factors. `out` must have length `xs.len() + ys.len() - 1`. When the factors' lengths
// differ, the longer is cut into pieces as long as the shorter, and the products of the pieces are
// added together.
pub(crate) fn mul_karatsuba_wrapping<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], ys: &[T]) {
    let (xs, ys) = if xs.len() >= ys.len() {
        (xs, ys)
    } else {
        (ys, xs)
    };
    let n = xs.len();
    let m = ys.len();
    if m < MOD_POWER_OF_2_MUL_KARATSUBA_THRESHOLD {
        mul_classical_wrapping(out, xs, ys);
        return;
    }
    let mut scratch = vec![T::ZERO; karatsuba_wrapping_scratch_len(m)];
    if n == m {
        mul_karatsuba_balanced_wrapping(out, xs, ys, &mut scratch);
        return;
    }
    out.fill(T::ZERO);
    let mut product = vec![T::ZERO; (m << 1) - 1];
    for (k, piece) in xs.chunks(m).enumerate() {
        let product = &mut product[..piece.len() + m - 1];
        if piece.len() == m {
            mul_karatsuba_balanced_wrapping(product, piece, ys, &mut scratch);
        } else {
            mul_karatsuba_wrapping(product, ys, piece);
        }
        add_wrapping_assign(&mut out[k * m..], product);
    }
}

fn assert_lengths<T>(out: &[T], xs: &[T], ys: &[T]) {
    assert!(!xs.is_empty());
    assert!(!ys.is_empty());
    assert_eq!(out.len(), xs.len() + ys.len() - 1);
}

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, both nonempty and
// reduced modulo $2^k$, where $k$ is `pow`, by schoolbook multiplication. `out` must have length
// `xs.len() + ys.len() - 1`, and `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_mul_to_out_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    pow: u64,
) {
    assert_lengths(out, xs, ys);
    assert!(pow <= T::WIDTH);
    mul_classical_wrapping(out, xs, ys);
    mask_coefficients(out, pow);
}}

// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, both nonempty and
// reduced modulo $2^k$, where $k$ is `pow`, by Karatsuba multiplication. `out` must have length
// `xs.len() + ys.len() - 1`, and `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_mul_to_out_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    pow: u64,
) {
    assert_lengths(out, xs, ys);
    assert!(pow <= T::WIDTH);
    mul_karatsuba_wrapping(out, xs, ys);
    mask_coefficients(out, pow);
}}

/// Sets `out` to the product of the polynomials with coefficients `xs` and `ys`, both nonempty and
/// reduced modulo $2^k$, where $k$ is `pow`. `out` must have length `xs.len() + ys.len() - 1`, and
/// `pow` must be no greater than `T::WIDTH`.
///
/// This is not part of the public API; it is public so that `malachite-nz` can multiply
/// `NaturalPolynomial`s with word-sized coefficients modulo $2^k$.
#[doc(hidden)]
pub fn mod_power_of_2_mul_to_out<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    pow: u64,
) {
    assert_lengths(out, xs, ys);
    assert!(pow <= T::WIDTH);
    mul_karatsuba_wrapping(out, xs, ys);
    mask_coefficients(out, pow);
}

// The polynomial whose coefficients are `xs`, trimmed.
pub(crate) fn from_coefficients_trimmed<T: PrimitiveUnsigned>(xs: Vec<T>) -> UnsignedPolynomial<T> {
    let mut p = UnsignedPolynomial { coefficients: xs };
    p.trim();
    p
}

// The product of the polynomials with coefficients `xs` and `ys`, both reduced modulo $2^k$, where
// $k$ is `pow`, modulo $2^k$.
fn mod_power_of_2_mul_helper<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    pow: u64,
) -> UnsignedPolynomial<T> {
    if xs.is_empty() || ys.is_empty() {
        return UnsignedPolynomial::ZERO;
    }
    let mut out = vec![T::ZERO; xs.len() + ys.len() - 1];
    mod_power_of_2_mul_to_out(&mut out, xs, ys, pow);
    from_coefficients_trimmed(out)
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $2^k$, taking both by value. The coefficients
    /// of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = pq \bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the product can vanish modulo $2^k$, and then the degree of the
    /// product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The coefficients wrap around modulo 16.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "2*x^3+11*x^2+3*x+10"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("8*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "10*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_mul(self, other: Self, pow: u64) -> Self {
        assert_reduced(&self, &other, pow);
        mod_power_of_2_mul_helper(&self.coefficients, &other.coefficients, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<&Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $2^k$, taking the first by value and the
    /// second by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = pq \bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the product can vanish modulo $2^k$, and then the degree of the
    /// product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The coefficients wrap around modulo 16.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "2*x^3+11*x^2+3*x+10"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("8*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(&UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "10*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_mul(self, other: &Self, pow: u64) -> Self {
        assert_reduced(&self, other, pow);
        mod_power_of_2_mul_helper(&self.coefficients, &other.coefficients, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<UnsignedPolynomial<T>> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $2^k$, taking the first by reference and the
    /// second by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = pq \bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the product can vanish modulo $2^k$, and then the degree of the
    /// product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The coefficients wrap around modulo 16.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_mul(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "2*x^3+11*x^2+3*x+10"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap())
    ///         .mod_power_of_2_mul(UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "10*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_mul(self, other: UnsignedPolynomial<T>, pow: u64) -> UnsignedPolynomial<T> {
        assert_reduced(self, &other, pow);
        mod_power_of_2_mul_helper(&self.coefficients, &other.coefficients, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<&UnsignedPolynomial<T>> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Multiplies two [`UnsignedPolynomial`]s modulo $2^k$, taking both by reference. The
    /// coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = pq \bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the product can vanish modulo $2^k$, and then the degree of the
    /// product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the longer polynomial.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The coefficients wrap around modulo 16.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_mul(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "2*x^3+11*x^2+3*x+10"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap())
    ///         .mod_power_of_2_mul(&UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "10*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_mul(self, other: &UnsignedPolynomial<T>, pow: u64) -> UnsignedPolynomial<T> {
        assert_reduced(self, other, pow);
        mod_power_of_2_mul_helper(&self.coefficients, &other.coefficients, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulAssign<Self> for UnsignedPolynomial<T> {
    /// Multiplies an [`UnsignedPolynomial`] by another [`UnsignedPolynomial`] modulo $2^k$ in
    /// place, taking the right-hand side by value. The coefficients of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// p \gets pq \bmod 2^k.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_mul_assign(UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 4);
    /// assert_eq!(p.to_string(), "2*x^3+11*x^2+3*x+10");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_mul_assign(&mut self, other: Self, pow: u64) {
        assert_reduced(self, &other, pow);
        *self = mod_power_of_2_mul_helper(&self.coefficients, &other.coefficients, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulAssign<&Self> for UnsignedPolynomial<T> {
    /// Multiplies an [`UnsignedPolynomial`] by another [`UnsignedPolynomial`] modulo $2^k$ in
    /// place, taking the right-hand side by reference. The coefficients of both must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets pq \bmod 2^k.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_mul_assign(&UnsignedPolynomial::<u8>::from_str("2*x+5").unwrap(), 4);
    /// assert_eq!(p.to_string(), "2*x^3+11*x^2+3*x+10");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mul` from `nmod_poly/mul.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_mul_assign(&mut self, other: &Self, pow: u64) {
        assert_reduced(self, other, pow);
        *self = mod_power_of_2_mul_helper(&self.coefficients, &other.coefficients, pow);
    }
}
