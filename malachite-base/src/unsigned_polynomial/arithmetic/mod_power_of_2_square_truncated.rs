// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::arithmetic::traits::ModPowerOf2IsReduced;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModPowerOf2SquareTruncated, ModPowerOf2SquareTruncatedAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::{
    MOD_POWER_OF_2_SQUARE_KARATSUBA_THRESHOLD, add_wrapping_assign, from_coefficients_trimmed,
    mask_coefficients,
};
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_square::square_karatsuba_wrapping;
use alloc::vec;
use core::cmp::min;

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, modulo $2^\text{W}$, by schoolbook multiplication: each product of two
// different coefficients is computed once and doubled.
pub(crate) fn square_truncated_classical_wrapping<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T]) {
    let len = out.len();
    out.fill(T::ZERO);
    for (i, &x) in xs.iter().enumerate() {
        if (i << 1) + 1 >= len {
            break;
        }
        if x != T::ZERO {
            for (o, &y) in out[(i << 1) + 1..].iter_mut().zip(&xs[i + 1..]) {
                o.wrapping_add_assign(x.wrapping_mul(y));
            }
        }
    }
    for o in out.iter_mut() {
        *o = o.wrapping_add(*o);
    }
    for (o, &x) in out.iter_mut().step_by(2).zip(xs) {
        o.wrapping_add_assign(x.wrapping_mul(x));
    }
}

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, which is nonempty, modulo $2^\text{W}$. With $n$ equal to `out.len()` and $h =
// \lceil n/2 \rceil$, write x = x_0 + x^h x_1; since $2h \geq n$, x^2 mod x^n is x_0^2 + 2 x^h x_0
// x_1 mod x^n. The square is a full square of a half-length polynomial, computed by Karatsuba
// multiplication, and the cross product is a truncated product of half the length.
pub(crate) fn square_truncated_karatsuba_wrapping<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T]) {
    let len = out.len();
    let xs = &xs[..min(xs.len(), len)];
    let n = xs.len();
    let full_len = (n << 1) - 1;
    if full_len <= len {
        square_karatsuba_wrapping(&mut out[..full_len], xs);
        out[full_len..].fill(T::ZERO);
        return;
    }
    if n < MOD_POWER_OF_2_SQUARE_KARATSUBA_THRESHOLD {
        square_truncated_classical_wrapping(out, xs);
        return;
    }
    let h = len.div_ceil(2);
    let x0 = &xs[..min(h, n)];
    // The square of x_0 has at most 2h - 1 <= `len` coefficients, so this is a full square.
    square_truncated_karatsuba_wrapping(out, x0);
    if n > h {
        let mut cross = vec![T::ZERO; len - h];
        mul_truncated_karatsuba_wrapping(&mut cross, x0, &xs[h..]);
        add_wrapping_assign(&mut out[h..], &cross);
        add_wrapping_assign(&mut out[h..], &cross);
    }
}

fn assert_lengths<T>(out: &[T], xs: &[T]) {
    assert!(!out.is_empty());
    assert!(!xs.is_empty());
}

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, both nonempty and the input reduced modulo $2^k$, where $k$ is `pow`, by
// schoolbook multiplication. `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_square_truncated_to_out_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    pow: u64,
) {
    assert_lengths(out, xs);
    assert!(pow <= T::WIDTH);
    square_truncated_classical_wrapping(out, xs);
    mask_coefficients(out, pow);
}}

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, both nonempty and the input reduced modulo $2^k$, where $k$ is `pow`, by
// Karatsuba multiplication. `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_square_truncated_to_out_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    pow: u64,
) {
    assert_lengths(out, xs);
    assert!(pow <= T::WIDTH);
    square_truncated_karatsuba_wrapping(out, xs);
    mask_coefficients(out, pow);
}}

/// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
/// coefficients `xs`, both nonempty and the input reduced modulo $2^k$, where $k$ is `pow`. `pow`
/// must be no greater than `T::WIDTH`.
///
/// This is not part of the public API; it is public so that `malachite-nz` can square
/// `NaturalPolynomial`s with word-sized coefficients modulo $2^k$.
#[doc(hidden)]
pub fn mod_power_of_2_square_truncated_to_out<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    pow: u64,
) {
    assert_lengths(out, xs);
    assert!(pow <= T::WIDTH);
    square_truncated_karatsuba_wrapping(out, xs);
    mask_coefficients(out, pow);
}

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, pow: u64) {
    assert!(pow <= T::WIDTH);
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

// The square of the polynomial with coefficients `xs`, reduced modulo $2^k$, where $k$ is `pow`,
// truncated to `len` coefficients and reduced modulo $2^k$.
pub(crate) fn mod_power_of_2_square_truncated_helper<T: PrimitiveUnsigned>(
    xs: &[T],
    len: u64,
    pow: u64,
) -> UnsignedPolynomial<T> {
    if len == 0 || xs.is_empty() {
        return UnsignedPolynomial::ZERO;
    }
    let mut out = vec![T::ZERO; truncated_len(xs.len(), xs.len(), len)];
    mod_power_of_2_square_truncated_to_out(&mut out, xs, pow);
    from_coefficients_trimmed(out)
}

impl<T: PrimitiveUnsigned> ModPowerOf2SquareTruncated for UnsignedPolynomial<T> {
    type Output = Self;

    /// Squares an [`UnsignedPolynomial`] modulo $2^k$, keeping only the coefficients of $x^i$ for
    /// $i$ less than `len`, taking it by value. Its coefficients must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(p, n, k) = (p^2 \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the square of its image modulo $x^n$,
    /// so only its first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2SquareTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The square is x^4+6*x^3+13*x^2+12*x+4; its low three coefficients, modulo 8.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_square_truncated(3, 3)
    ///         .to_string(),
    ///     "5*x^2+4*x+4"
    /// );
    /// // The linear coefficient of the square, 8, vanishes modulo 8.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("4*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_square_truncated(2, 3)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0, with both
    /// factors equal and the modulus $2^k$.
    fn mod_power_of_2_square_truncated(self, len: u64, pow: u64) -> Self {
        assert_reduced(&self, pow);
        mod_power_of_2_square_truncated_helper(&self.coefficients, len, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SquareTruncated for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Squares an [`UnsignedPolynomial`] modulo $2^k$, keeping only the coefficients of $x^i$ for
    /// $i$ less than `len`, taking it by reference. Its coefficients must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(p, n, k) = (p^2 \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// The polynomial need not already be truncated: this is the square of its image modulo $x^n$,
    /// so only its first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{\log_2 3})$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2SquareTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The square is x^4+6*x^3+13*x^2+12*x+4; its low three coefficients, modulo 8.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_power_of_2_square_truncated(3, 3)
    ///         .to_string(),
    ///     "5*x^2+4*x+4"
    /// );
    /// // The linear coefficient of the square, 8, vanishes modulo 8.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("4*x+1").unwrap())
    ///         .mod_power_of_2_square_truncated(2, 3)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0, with both
    /// factors equal and the modulus $2^k$.
    fn mod_power_of_2_square_truncated(self, len: u64, pow: u64) -> UnsignedPolynomial<T> {
        assert_reduced(self, pow);
        mod_power_of_2_square_truncated_helper(&self.coefficients, len, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SquareTruncatedAssign for UnsignedPolynomial<T> {
    /// Squares an [`UnsignedPolynomial`] modulo $2^k$ in place, keeping only the coefficients of
    /// $x^i$ for $i$ less than `len`. Its coefficients must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets (p^2 \bmod x^n) \bmod 2^k.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2SquareTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_square_truncated_assign(3, 3);
    /// assert_eq!(p.to_string(), "5*x^2+4*x+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0, with both
    /// factors equal and the modulus $2^k$.
    fn mod_power_of_2_square_truncated_assign(&mut self, len: u64, pow: u64) {
        assert_reduced(self, pow);
        *self = mod_power_of_2_square_truncated_helper(&self.coefficients, len, pow);
    }
}
