// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::arithmetic::traits::ModIsReduced;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModSquareTruncated, ModSquareTruncatedAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_mul::{
    MOD_SQUARE_KARATSUBA_THRESHOLD, ModData, mod_add_assign_slice,
};
use crate::unsigned_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_karatsuba;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::from_coefficients_trimmed;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul_truncated::truncated_len;
use crate::unsigned_polynomial::arithmetic::mod_square::{
    mod_square_classical_prefix, mod_square_karatsuba,
};
use alloc::vec;
use core::cmp::min;

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, which is nonempty, modulo $m$. With $n$ equal to `out.len()` and $h = \lceil
// n/2 \rceil$, write x = x_0 + x^h x_1; since $2h \geq n$, x^2 mod x^n is x_0^2 + 2 x^h x_0 x_1 mod
// x^n. The square is a full square of a half-length polynomial, computed by Karatsuba
// multiplication, and the cross product is a truncated product of half the length.
pub(crate) fn mod_square_truncated_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    d: &ModData<T>,
) {
    let len = out.len();
    let xs = &xs[..min(xs.len(), len)];
    let n = xs.len();
    let full_len = (n << 1) - 1;
    if full_len <= len {
        mod_square_karatsuba(&mut out[..full_len], xs, d);
        out[full_len..].fill(T::ZERO);
        return;
    }
    if n < MOD_SQUARE_KARATSUBA_THRESHOLD {
        mod_square_classical_prefix(out, xs, d);
        return;
    }
    let h = len.div_ceil(2);
    let x0 = &xs[..min(h, n)];
    // The square of x_0 has at most 2h - 1 <= `len` coefficients, so this is a full square.
    mod_square_truncated_karatsuba(out, x0, d);
    if n > h {
        let mut cross = vec![T::ZERO; len - h];
        mod_mul_truncated_karatsuba(&mut cross, x0, &xs[h..], d);
        mod_add_assign_slice(&mut out[h..], &cross, d.m);
        mod_add_assign_slice(&mut out[h..], &cross, d.m);
    }
}

fn assert_lengths<T>(out: &[T], xs: &[T]) {
    assert!(!out.is_empty());
    assert!(!xs.is_empty());
}

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, both nonempty and the input reduced modulo `m`, modulo `m`, by schoolbook
// multiplication.
crate_test_fn! {
#[allow(dead_code)]
mod_square_truncated_to_out_classical<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], m: T) {
    assert_lengths(out, xs);
    mod_square_classical_prefix(out, xs, &ModData::new(m, xs.len().min(out.len())));
}}

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, both nonempty and the input reduced modulo `m`, modulo `m`, by Karatsuba
// multiplication.
crate_test_fn! {
#[allow(dead_code)]
mod_square_truncated_to_out_karatsuba<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], m: T) {
    assert_lengths(out, xs);
    mod_square_truncated_karatsuba(out, xs, &ModData::new(m, xs.len().min(out.len())));
}}

/// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
/// coefficients `xs`, both nonempty and the input reduced modulo `m`, modulo `m`. `m` must be
/// positive.
///
/// This is not part of the public API; it is public so that `malachite-nz` can square
/// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
pub fn mod_square_truncated_to_out<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], m: T) {
    assert_lengths(out, xs);
    mod_square_truncated_karatsuba(out, xs, &ModData::new(m, xs.len().min(out.len())));
}

fn assert_reduced<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, m: T) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
}

// The square of the polynomial with coefficients `xs`, reduced modulo `m`, truncated to `len`
// coefficients and reduced modulo `m`.
fn mod_square_truncated_helper<T: PrimitiveUnsigned>(
    xs: &[T],
    len: u64,
    m: T,
) -> UnsignedPolynomial<T> {
    if len == 0 || xs.is_empty() {
        return UnsignedPolynomial::ZERO;
    }
    let mut out = vec![T::ZERO; truncated_len(xs.len(), xs.len(), len)];
    mod_square_truncated_to_out(&mut out, xs, m);
    from_coefficients_trimmed(out)
}

impl<T: PrimitiveUnsigned> ModSquareTruncated<T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Squares an [`UnsignedPolynomial`] modulo $m$, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking it by value. Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, n, m) = (p^2 \bmod x^n) \bmod m.
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
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSquareTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The square is x^4+6*x^3+13*x^2+12*x+4; its low three coefficients, modulo 7.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_square_truncated(3, 7)
    ///         .to_string(),
    ///     "6*x^2+5*x+4"
    /// );
    /// // The linear coefficient of the square, 4, vanishes modulo 4.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("2*x+1")
    ///         .unwrap()
    ///         .mod_square_truncated(2, 4)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0, with both
    /// factors equal.
    fn mod_square_truncated(self, len: u64, m: T) -> Self {
        assert_reduced(&self, m);
        mod_square_truncated_helper(&self.coefficients, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModSquareTruncated<T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Squares an [`UnsignedPolynomial`] modulo $m$, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking it by reference. Its coefficients must already be reduced modulo
    /// $m$.
    ///
    /// $$
    /// f(p, n, m) = (p^2 \bmod x^n) \bmod m.
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
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSquareTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The square is x^4+6*x^3+13*x^2+12*x+4; its low three coefficients, modulo 7.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap())
    ///         .mod_square_truncated(3, 7)
    ///         .to_string(),
    ///     "6*x^2+5*x+4"
    /// );
    /// // The linear coefficient of the square, 4, vanishes modulo 4.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap())
    ///         .mod_square_truncated(2, 4)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0, with both
    /// factors equal.
    fn mod_square_truncated(self, len: u64, m: T) -> UnsignedPolynomial<T> {
        assert_reduced(self, m);
        mod_square_truncated_helper(&self.coefficients, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModSquareTruncatedAssign<T> for UnsignedPolynomial<T> {
    /// Squares an [`UnsignedPolynomial`] modulo $m$ in place, keeping only the coefficients of
    /// $x^i$ for $i$ less than `len`. Its coefficients must already be reduced modulo $m$.
    ///
    /// $$
    /// p \gets (p^2 \bmod x^n) \bmod m.
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
    /// Panics if `m` is 0, or if any coefficient of `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModSquareTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+3*x+2").unwrap();
    /// p.mod_square_truncated_assign(3, 7);
    /// assert_eq!(p.to_string(), "6*x^2+5*x+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_mullow` from `nmod_poly/mullow.c`, FLINT 3.6.0, with both
    /// factors equal.
    fn mod_square_truncated_assign(&mut self, len: u64, m: T) {
        assert_reduced(self, m);
        *self = mod_square_truncated_helper(&self.coefficients, len, m);
    }
}
